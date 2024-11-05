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
        molecules::Canvas,
        CanvasState,
        MapState,
    },
    models::Map,
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
                    state.set_map(resp.map);
                });
                IDManager::from_data(resp.id_manager_data);
            }
        }
    });

    let zoom_in =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_in));
    let zoom_out =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_out));
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
        };

        algorithm_req.dispatch(req);
    };
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

    view! {
    <div _ref=container_ref id="canvas-container" class="grow flex self-stretch relative">
        <Canvas/>
        <div class=algorithm_button_class>
            <Button text="recalculate map" on_click=Box::new(run_algorithm) overlay=true bigger=true>
                <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-spin"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z"/>
                    <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                    <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5" />
                </svg>
            </Button>
        </div>
        <Show when=move || algorithm_req.pending().get()>
            <div class="absolute right-5 top-24">
                <Button text="abort" on_click=Box::new(abort_algorithm) overlay=true><span class="text-red-300">x</span></Button>
            </div>
        </Show>
        <div class="absolute right-5 bottom-20">
            <Button text="zoom in" on_click=Box::new(zoom_in) overlay=true>+</Button>
        </div>
        <div class="absolute right-5 bottom-5">
            <Button text="zoom out" on_click=Box::new(zoom_out) overlay=true>-</Button>
        </div>
    </div>
    }
}
