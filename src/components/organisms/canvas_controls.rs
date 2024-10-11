//! Contains the [`CanvasControls`] component.

use ev::KeyboardEvent;
use html::Div;
use leptos::*;
use leptos_workers::worker;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    algorithm::{
        self,
        AlgorithmSettings,
    },
    components::{
        atoms::Button,
        molecules::Canvas,
        CanvasState,
        MapState,
    },
    models::Map,
    unwrap_or_return,
    utils::Result as AlgrithmResult,
};

/// The request to run the algorithm.
#[derive(Clone, Serialize, Deserialize)]
struct AlgorithmRequest {
    settings: AlgorithmSettings,
    map: Map,
}

/// The response from the algorithm.
#[derive(Clone, Serialize, Deserialize)]
struct AlgorithmResponse {
    map: Map,
}

/// The worker that runs the algorithm.
#[worker(AlgorithmWorker)]
fn run_algorithm(req: AlgorithmRequest) -> AlgorithmResponse {
    logging::log!("{:?}", &req.map);
    let mut temp_state = MapState::new(req.map);
    temp_state.set_algorithm_settings(req.settings);

    temp_state.run_algorithm();

    AlgorithmResponse {
        map: temp_state
            .get_map()
            .clone(),
    }
}

/// The canvas and the controls overlayed on it.
#[component]
pub fn CanvasControls() -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");
    let (is_calculating, set_calculating) = create_signal(false);

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

    let algorithm_req = create_action(move |_: &()| {
        async move {
            let req = AlgorithmRequest {
                settings: map_state
                    .get_untracked()
                    .get_algorithm_settings(),
                map: map_state
                    .get_untracked()
                    .get_map()
                    .clone(),
            };
            let resp = run_algorithm(req)
                .await
                .expect("Errored on thread startup");
            map_state.update(|state| {
                state.set_map(resp.map);
            });
        }
    });

    let zoom_in =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_in));
    let zoom_out =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_out));
    let run_algorithm = move |_| {
        algorithm_req.dispatch(());
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
            <Button text="recalculate map" on_click=Box::new(run_algorithm) overlay=true can_focus=false bigger=true>
                <svg class="h-8 w-8 text-blue-500 group-[.is-calculating]:animate-spin"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z"/>
                    <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5" />
                    <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5" />
                </svg>
            </Button>
        </div>
        <div class="absolute right-5 bottom-20">
            <Button text="zoom in" on_click=Box::new(zoom_in) overlay=true can_focus=false >+</Button>
        </div>
        <div class="absolute right-5 bottom-5">
            <Button text="zoom out" on_click=Box::new(zoom_out) overlay=true can_focus=false>-</Button>
        </div>
    </div>
    }
}
