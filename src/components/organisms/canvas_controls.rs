//! Contains the [`CanvasControls`] component.

use ev::KeyboardEvent;
use html::Div;
use leptos::*;

use crate::components::{
    atoms::Button,
    molecules::Canvas,
    CanvasState,
    MapState,
};

/// The canvas and the controls overlayed on it.
#[component]
pub fn CanvasControls() -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

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

    let zoom_in =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_in));
    let zoom_out =
        move |_| map_state.update(|state| state.update_canvas_state(CanvasState::zoom_out));
    let run_algorithm = move |_| map_state.update(|state| state.run_algorithm());

    view! {
    <div _ref=container_ref id="canvas-container" class="grow flex self-stretch relative">
        <Canvas/>
        <div class="absolute right-5 top-5 group">
            <Button text="recalculate map" on_click=Box::new(run_algorithm) overlay=true can_focus=false bigger=true>
                <svg class="h-8 w-8 text-blue-500 group-focus:animate-spin"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
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
