//! Contains the [`Canvas`] component.
//! In the files under this module are the different event handlers for the
//! component.

use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

use leptos::{
    html::Canvas as HtmlCanvas,
    *,
};
use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};

use crate::components::{
    ErrorState,
    MapState,
};

mod dbl_click;
mod keydown;
mod mouse_down;
mod mouse_move;
mod mouse_out;
mod mouse_up;
mod other;
mod scroll;

use dbl_click::on_dbl_click;
use keydown::on_keydown;
use mouse_down::on_mouse_down;
use mouse_move::on_mouse_move;
use mouse_out::on_mouse_out;
use mouse_up::on_mouse_up;
use other::update_canvas_size;
use scroll::on_scroll;

// TODO: Document the code in this module more thoroughly.

/// If the document has fully loaded.
static DOCUMENT_LOADED: AtomicBool = AtomicBool::new(false);

/// The canvas itself.
/// This is where the map is drawn on and the user can interact with the map.
#[component]
pub fn Canvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<HtmlCanvas>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");
    let error_state =
        use_context::<RwSignal<ErrorState>>().expect("to have found the global error state");

    // ensures we know the size of the canvas and that one page resizing, the canvas
    // is also resized.
    create_effect(move |_| {
        update_canvas_size(&map_state);

        if !DOCUMENT_LOADED.load(Ordering::Relaxed) {
            DOCUMENT_LOADED.store(true, Ordering::Release);
            let on_resize = Closure::<dyn Fn()>::new(move || update_canvas_size(&map_state));
            let on_keydown = Closure::<dyn Fn(JsValue)>::new(move |ev: JsValue| {
                on_keydown(&map_state, ev.unchecked_ref());
            });
            window().set_onresize(Some(
                on_resize
                    .as_ref()
                    .unchecked_ref(),
            ));
            on_resize.forget();
            window().set_onkeydown(Some(
                on_keydown
                    .as_ref()
                    .unchecked_ref(),
            ));
            on_keydown.forget();
        }
    });

    // redraw the canvas if the map state changes.
    create_effect(move |_| {
        let canvas_node = &canvas_ref
            .get()
            .expect("should be loaded now");
        let s = map_state
            .get()
            .get_canvas_state()
            .get_size();
        canvas_node.set_height(s.0 as u32);
        canvas_node.set_width(s.1 as u32);

        map_state
            .get()
            .draw_to_canvas(&canvas_ref);
    });

    view! {
        <div class="absolute grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas
                _ref=canvas_ref

                on:mousedown=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref(), ev.shift_key()))
                on:mouseup=move |ev| map_state.update(|state| on_mouse_up(state, error_state, ev.as_ref(), ev.shift_key()))
                on:mousemove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:mouseout=move |_| map_state.update(on_mouse_out)
                on:dblclick=move |ev| map_state.update(|state| on_dbl_click(state, ev.as_ref(), ev.shift_key()))

                on:touchstart=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref(), ev.shift_key()))
                on:touchend=move |ev| map_state.update(|state| on_mouse_up(state, error_state, ev.as_ref(), ev.shift_key()))
                on:touchmove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:touchcancel=move |_| map_state.update(on_mouse_out)

                on:wheel=move |ev| map_state.update(|state| on_scroll(state, ev.delta_y()))

                id="canvas"
                style="touch-action: none;"
                class="object-contain"/>
        </div>
    }
}
