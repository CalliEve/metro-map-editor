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

    view! {
    <div _ref=container_ref id="canvas-container" class="grow flex self-stretch relative">
        <Canvas/>
        <div class="absolute right-5 bottom-20">
            <Button text="+" on_click=Box::new(zoom_in) overlay=true can_focus=false />
        </div>
        <div class="absolute right-5 bottom-5">
            <Button text="-" on_click=Box::new(zoom_out) overlay=true can_focus=false />
        </div>
    </div>
    }
}
