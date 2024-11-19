//! Contains the [`CanvasControls`] component.

// Async is used for futures, which are used in the worker, even though the algorithm itself is
// sync.
#![allow(clippy::unused_async)]

use ev::KeyboardEvent;
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
    algorithm::AlgorithmSettings,
    components::{
        atoms::Button,
        molecules::{
            Canvas,
            EdgeInfoBox,
            StationInfoBox,
        },
        CanvasState,
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
}

/// The response from the algorithm.
#[derive(Clone, Serialize, Deserialize)]
struct AlgorithmResponse {
    /// If the algorithm ran successfully.
    success: bool,
    /// The Map outputted by the algorithm.
    map: Map,
    /// The data for the [`IDManager`] after the algorithm has run, ensuring the
    /// main thread will not create IDs in conflict with those in the map.
    id_manager_data: IDData,
}

/// The worker that runs the algorithm.
#[allow(dead_code)] // usage is hidden
#[worker(AlgorithmWorker)]
fn run_algorithm(req: AlgorithmRequest) -> AlgorithmResponse {
    IDManager::from_data(req.id_manager_data);

    let mut temp_state = MapState::new(req.map);
    temp_state.set_algorithm_settings(req.settings);

    let success = temp_state.run_algorithm();

    AlgorithmResponse {
        success,
        map: temp_state
            .get_map()
            .clone(),
        id_manager_data: IDManager::to_data(),
    }
}

/// The canvas and the controls overlayed on it.
#[component]
pub fn CanvasControls() -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");
    let (executor, _) = create_signal(
        PoolExecutor::<AlgorithmWorker>::new(1).expect("failed to start web-worker pool"),
    );
    let (abort_handle, set_abort_handle) =
        create_signal(Option::<AbortHandle<AlgorithmWorker>>::None);

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

    let algorithm_req = create_action(move |req: &AlgorithmRequest| {
        let req = req.clone();
        let is_partial = req.partial;
        async move {
            let (abort_handle, resp_fut) = executor
                .get_untracked()
                .run(req)
                .expect("failed to start algorithm worker");
            set_abort_handle(Some(abort_handle));
            let resp = resp_fut.await;
            if resp.success
                || map_state
                    .get_untracked()
                    .get_algorithm_settings()
                    .output_on_fail
            {
                map_state.update(|state| {
                    if is_partial {
                        state.clear_all_selections();
                        unwrap_or_return!(state
                            .get_mut_map()
                            .update_from_partial(&resp.map));
                    } else {
                        state.set_map(resp.map);
                    }
                });
                IDManager::from_data(resp.id_manager_data);
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
        };

        algorithm_req.dispatch(req);
    };

    // Abort the algorithm.
    let abort_algorithm = move |_| {
        if algorithm_req
            .pending()
            .get()
        {
            if let Some(handle) = abort_handle.get() {
                handle.abort();
                algorithm_req.set_pending(false);
            }
        }
    };

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

    view! {
    <div _ref=container_ref id="canvas-container" class="grow flex self-stretch relative">
        <Canvas/>
        <div class=algorithm_button_class>
            <Show
                when=has_parts_selected
                fallback=move || view!{
                    <Button text="recalculate map" on_click=Box::new(run_algorithm) overlay=true bigger=true>
                        <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-reverse-spin"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z"/>
                            <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                            <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5" />
                        </svg>
                    </Button>
                }>
                <Button text="recalculate selected parts" on_click=Box::new(run_partial_algorithm) overlay=true bigger=true>
                        <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-reverse-spin"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
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
