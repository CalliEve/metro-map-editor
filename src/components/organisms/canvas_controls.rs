//! Contains the [`CanvasControls`] component.

// Async is used for futures, which are used in the worker, even though the algorithm itself is
// sync.
#![allow(clippy::unused_async)]
// This otherwise gets triggered by one in the wasm worker.
#![allow(unexpected_cfgs)]

use ev::KeyboardEvent;
use futures_util::StreamExt;
use html::Div;
use leptos::*;
use leptos_workers::{
    executors::{
        AbortHandle,
        PoolExecutor,
    },
    worker,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    algorithm::{
        AlgorithmExecutor,
        AlgorithmResponse,
        AlgorithmSettings,
    },
    components::{
        atoms::Button,
        canvas::Canvas,
        molecules::{
            EdgeInfoBox,
            StationInfoBox,
        },
        CanvasState,
        ErrorState,
        MapState,
    },
    models::Map,
    unwrap_or_return,
    utils::{
        IDData,
        IDManager,
    },
};

/// The request to run the algorithm.
#[derive(Clone, Serialize, Deserialize)]
struct AlgorithmRequest {
    /// The settings for the algorithm.
    settings: AlgorithmSettings,
    /// The data for the [`IDManager`] to ensure the ids potentially generated
    /// in the algorithm are unique.
    id_manager_data: IDData,
    /// The map to run the algorithm on.
    map: Map,
    /// If the map is a partial map or not.
    partial: bool,
    /// If the algorithm should output the midway responses to the canvas.
    midway_updates: bool,
}

/// The worker that runs the algorithm.
#[allow(dead_code)] // usage is hidden
#[worker(AlgorithmWorker)]
fn run_algorithm(req: AlgorithmRequest) -> impl leptos_workers::Stream<Item = AlgorithmResponse> {
    IDManager::from_data(req.id_manager_data);

    let mut temp_state = MapState::new(req.map);
    temp_state.set_algorithm_settings(req.settings);
    temp_state.calculate_algorithm_settings();

    // Start the stream and thus the algorithm.
    AlgorithmExecutor::new(
        temp_state.get_algorithm_settings(),
        temp_state
            .get_map()
            .clone(),
        req.midway_updates,
    )
}

/// The canvas and the controls overlayed on it.
#[component]
pub fn CanvasControls() -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");
    let error_state =
        use_context::<RwSignal<ErrorState>>().expect("to have found the global error state");
    let (executor, _) = create_signal(
        PoolExecutor::<AlgorithmWorker>::new(1).expect("failed to start web-worker pool"),
    );
    let (abort_handle, set_abort_handle) =
        create_signal(Option::<(AbortHandle<AlgorithmWorker>, Map)>::None);

    create_effect(move |_| {
        window_event_listener(
            ev::keydown,
            move |keyevent: KeyboardEvent| {
                map_state.update(|state| {
                    state.update_canvas_state(|canvas| {
                        match keyevent
                            .key()
                            .as_str()
                        {
                            "ArrowDown" => canvas.move_down(),
                            "ArrowUp" => canvas.move_up(),
                            "ArrowLeft" => canvas.move_left(),
                            "ArrowRight" => canvas.move_right(),
                            _ => {},
                        }
                    });
                });
            },
        );
    });

    // If parts of the map has been selected and is not being moved.
    let has_parts_selected = Signal::derive(move || {
        let state = map_state.get();
        (!state
            .get_selected_edges()
            .is_empty())
            && state
                .get_selected_stations()
                .iter()
                .all(|s| !s.has_moved())
    });

    // Handle the response from the algorithm.
    let handle_algorithm_response = move |resp: AlgorithmResponse, partial: bool| {
        if resp.success
            || map_state
                .get_untracked()
                .get_algorithm_settings()
                .output_on_fail
        {
            map_state.update(|state| {
                if partial {
                    unwrap_or_return!(
                        error_state,
                        state
                            .get_mut_map()
                            .update_from_partial(&resp.map)
                    );
                } else {
                    state.set_map(resp.map);
                }
            });
            IDManager::from_data(resp.id_manager_data);
        } else {
            map_state.update(|state| {
                if let Some(map) = state
                    .get_last_loaded()
                    .cloned()
                {
                    state.set_map(map);
                }
            });
        }
        if let Some(error) = resp.error {
            error_state.update(|state| {
                state.set_error(error);
            });
        }
    };

    // Dispatch the algorithm request.
    let algorithm_req = create_action(move |req: &AlgorithmRequest| {
        map_state.update(|state| {
            state.clear_all_selections();
        });

        let req = req.clone();
        async move {
            let (abort_handle, resp_stream) = executor
                .get_untracked()
                .stream(&req)
                .expect("failed to start algorithm worker");
            set_abort_handle(Some((
                abort_handle,
                req.map
                    .clone(),
            )));

            // Handle the responses from the algorithm.
            // This is done in a fold to ensure only the last response is handled later, but
            // all midway updates are handled conditionally.
            let last = resp_stream
                .inspect(|resp| {
                    if req.midway_updates {
                        handle_algorithm_response(resp.clone(), req.partial);
                    }
                })
                .fold(
                    None,
                    |_, next| async move { Some(next) },
                )
                .await;

            // If we got a response and it wasn't handled by the midway handler, handle it
            // now.
            if !req.midway_updates {
                if let Some(resp) = last {
                    handle_algorithm_response(resp, req.partial);
                }
            }
        }
    });

    let zoom_in =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_in));
    let zoom_out =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_out));

    // Run the algorithm on the entire map.
    let run_algorithm = move |_| {
        let req = AlgorithmRequest {
            settings: map_state
                .get_untracked()
                .get_algorithm_settings(),
            map: map_state
                .get_untracked()
                .get_map()
                .clone(),
            id_manager_data: IDManager::to_data(),
            partial: false,
            midway_updates: false,
        };

        algorithm_req.dispatch(req);
    };

    // Run the algorithm only on the selected stations and edges.
    let run_partial_algorithm = move |_| {
        let req = AlgorithmRequest {
            settings: map_state
                .get_untracked()
                .get_algorithm_settings(),
            map: map_state
                .get_untracked()
                .lock_all_unselected(),
            id_manager_data: IDManager::to_data(),
            partial: true,
            midway_updates: false,
        };

        algorithm_req.dispatch(req);
    };

    // Run the algorithm on the entire map.
    let run_stream_algorithm = move |_| {
        let partial = has_parts_selected.get_untracked();
        let req = AlgorithmRequest {
            settings: map_state
                .get_untracked()
                .get_algorithm_settings(),
            map: map_state
                .get_untracked()
                .get_map()
                .clone(),
            id_manager_data: IDManager::to_data(),
            partial,
            midway_updates: true,
        };

        algorithm_req.dispatch(req);
    };

    // Abort the algorithm.
    let abort_algorithm = move |_| {
        if algorithm_req
            .pending()
            .get()
        {
            if let Some((handle, original_map)) = abort_handle.get() {
                handle.abort();
                algorithm_req.set_pending(false);
                map_state.update(|state| {
                    state.set_map(original_map);
                });
            }
        }
    };

    // The class for the algorithm button.
    let algorithm_button_class = move || {
        let mut class = "absolute right-5 top-5 group".to_owned();

        if algorithm_req
            .pending()
            .get()
        {
            class += " is-calculating";
        }

        class
    };

    // Toggle the original map overlay.
    let overlay_original_map = move |_| {
        map_state.update(|state| {
            state.set_original_overlay_enabled(!state.is_original_overlay_enabled());
        });
    };

    // If the original map overlay is active.
    let is_original_overlay_active = Signal::derive(move || {
        map_state
            .get()
            .is_original_overlay_enabled()
    });

    view! {
    <div _ref=container_ref id="canvas-container" class="grow flex self-stretch relative">
        <Canvas/>
        <div class=algorithm_button_class>
            <Show
                when=has_parts_selected
                fallback=move || view!{
                    <Button text="recalculate map" on_click=Box::new(run_algorithm) overlay=true bigger=true>
                        <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-reverse-spin group-[.is-calculating]:cursor-wait"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z"/>
                            <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                            <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5" />
                        </svg>
                    </Button>
                }>
                <Button text="recalculate selected parts" on_click=Box::new(run_partial_algorithm) overlay=true bigger=true>
                        <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-reverse-spin group-[.is-calculating]:cursor-wait"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z"/>
                            <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                        </svg>
                    </Button>
            </Show>
        </div>
        <Show when=move || algorithm_req.pending().get()>
            <div class="absolute right-5 top-24">
                <Button text="abort" on_click=Box::new(abort_algorithm) overlay=true><span class="text-red-300">x</span></Button>
            </div>
        </Show>
        <Show when=move || !algorithm_req.pending().get()>
            <div class="absolute right-5 top-24">
                <Button text="recalculate with\nreal-time updates" on_click=Box::new(run_stream_algorithm) overlay=true>
                    <svg class="text-blue-500 -ml-1 mt-1 h-6 w-6"  width="24" height="24" viewBox="0 0 28 28" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z"/>
                        <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                        <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5" />
                    </svg>
                </Button>
            </div>
        </Show>
        <div class="absolute right-24 top-5 group">
            <Button text="show original\nmap overlay" on_click=Box::new(overlay_original_map) overlay=true active=is_original_overlay_active>
                <svg class="text-blue-500 -m-1" width="20" height="20" viewBox="0 0 32 32" stroke-width="2.1" stroke="currentColor" fill="none">
                    <path d="M28,8H24V4a2.0023,2.0023,0,0,0-2-2H4A2.0023,2.0023,0,0,0,2,4V22a2.0023,2.0023,0,0,0,2,2H8v4a2.0023,2.0023,0,0,0,2,2H28a2.0023,2.0023,0,0,0,2-2V10A2.0023,2.0023,0,0,0,28,8ZM4,22V4H22V8H10a2.0023,2.0023,0,0,0-2,2V22Zm18,0H19.4141L10,12.586V10h2.5859l9.4153,9.4156ZM10,15.4141,16.5859,22H10ZM22.001,16.587,15.4141,10H22ZM10,28V24H22a2.0023,2.0023,0,0,0,2-2V10h4V28Z" transform="translate(0 0)"/>
                </svg>
            </Button>
        </div>
        <div class="absolute right-5 bottom-20">
            <Button text="zoom in" on_click=Box::new(zoom_in) overlay=true>+</Button>
        </div>
        <div class="absolute right-5 bottom-5">
            <Button text="zoom out" on_click=Box::new(zoom_out) overlay=true>-</Button>
        </div>
        <StationInfoBox/>
        <EdgeInfoBox/>
    </div>
    }
}
