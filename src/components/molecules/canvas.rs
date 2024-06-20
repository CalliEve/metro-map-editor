//! Contains the [`Canvas`] component.

use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

use ev::UiEvent;
use leptos::{
    html::Canvas as HtmlCanvas,
    *,
};
use wasm_bindgen::{
    closure::Closure,
    JsCast,
};

use crate::{
    components::MapState,
    models::Station,
    utils::calc_grid_loc,
};

/// If the document has fully loaded.
static DOCUMENT_LOADED: AtomicBool = AtomicBool::new(false);

/// Calculates and updates the size of the canvas.
///
/// To have a canvas resize dynamically, we need to manually adjust its size
/// CSS will NOT work, as it will just make everything blurry.
/// This means we have to manually calculate the desired size of the canvas.
fn update_canvas_size(map_state: &RwSignal<MapState>) {
    let doc = document();

    // the navbar borders the top, so the height is `window - navbar`.
    let win_height = window()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap();

    let navbar = doc
        .get_element_by_id("navbar")
        .expect("navbar should exist");
    let nav_height_px = window()
        .get_computed_style(&navbar)
        .unwrap()
        .expect("should have style")
        .get_property_value("height")
        .expect("should have height property");

    let height = (win_height
        - nav_height_px
            .trim_end_matches("px")
            .parse::<f64>()
            .expect("height should be a number")
            .round()) as u32;

    // the sidebar borders its side, so width is `window - sidebar`.
    let win_width = window()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();

    let sidebar = doc
        .get_element_by_id("sidebar")
        .expect("sidebar should exist");
    let side_width_px = window()
        .get_computed_style(&sidebar)
        .unwrap()
        .expect("should have style")
        .get_property_value("width")
        .expect("should have width property");

    let width = (win_width
        - side_width_px
            .trim_end_matches("px")
            .parse::<f64>()
            .expect("width should be a number")
            .round()) as u32;

    // update the state with the new size.
    map_state.update(|state| state.set_size((height, width)));
}

/// Gets the position on the canvas that was clicked.
fn canvas_click_pos(map_size: (u32, u32), ev: &UiEvent) -> (f64, f64) {
    let win_height = window()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();
    let win_width = window()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();

    (
        (f64::from(ev.page_x()) - (win_width - f64::from(map_size.1))),
        (f64::from(ev.page_y()) - (win_height - f64::from(map_size.0))),
    )
}

/// Listener for the [mousedown] event on the canvas.
///
/// [mousedown]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
fn on_mouse_down(map_state_signal: &RwSignal<MapState>, ev: &UiEvent) {
    let mut map_state = map_state_signal.get();
    let Some(map) = map_state.get_map() else {
        return;
    };

    let canvas_pos = canvas_click_pos(map_state.get_size(), ev);

    let mouse_pos = calc_grid_loc(canvas_pos, map_state.get_square_size());
    let selected_opt = map
        .station_at_pos(mouse_pos)
        .map(Station::clone_non_ref);
    if selected_opt.is_none() {
        return;
    }
    let mut selected = selected_opt.unwrap();

    selected.set_pos(mouse_pos);
    selected.set_is_ghost(true);
    map_state.set_selected_station(selected);

    map_state_signal.set(map_state);
}

/// Listener for the [mouseup] event on the canvas.
///
/// [mouseup]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
fn on_mouse_up(map_state_signal: &RwSignal<MapState>) {
    let mut map_state = map_state_signal.get();
    if !map_state.has_selected_station() {
        return;
    }

    let map = map_state
        .get_map()
        .cloned()
        .unwrap();
    let mut selected = map_state
        .get_selected_station()
        .cloned()
        .unwrap();
    selected.set_is_ghost(false);

    for station in map.get_stations() {
        if *station == selected {
            station.set_pos(selected.get_pos());
            break;
        }
    }

    map_state.set_map(map);
    map_state.clear_selected_station();

    map_state_signal.set(map_state);
}

/// Listener for the [mousemove] event on the canvas.
///
/// [mousemove]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
fn on_mouse_move(map_state_signal: &RwSignal<MapState>, ev: &UiEvent) {
    let mut map_state = map_state_signal.get();
    let selected_opt = map_state.get_selected_station();
    if selected_opt.is_none() {
        return;
    }
    let selected = selected_opt
        .cloned()
        .unwrap();

    let canvas_pos = canvas_click_pos(map_state.get_size(), ev);

    let mouse_pos = calc_grid_loc(canvas_pos, map_state.get_square_size());
    if mouse_pos == selected.get_pos() {
        return;
    }

    selected.set_pos(mouse_pos);
    map_state.set_selected_station(selected);

    map_state_signal.set(map_state);
}

/// Listener for the [mouseout] event on the canvas.
///
/// [mouseout]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event
fn on_mouse_out(map_state_signal: &RwSignal<MapState>) {
    let mut map_state = map_state_signal.get();
    if !map_state.has_selected_station() {
        return;
    }

    map_state.clear_selected_station();

    map_state_signal.set(map_state);
}

/// Listener for when the user scrolls on the canvas.
fn on_scroll(amount: f64, map_state_signal: &RwSignal<MapState>) {
    let current = map_state_signal
        .get()
        .get_square_size();

    let size = if amount > 0.0 {
        if current >= 100 {
            return;
        }
        current + 5
    } else {
        if current <= 5 {
            return;
        }
        current - 5
    };

    map_state_signal.update(|state| state.set_square_size(size));
}

/// The canvas itself.
/// This is where the map is drawn on and the user can interact with the map.
#[component]
pub fn Canvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<HtmlCanvas>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    // ensures we know the size of the canvas and that one page resizing, the canvas
    // is also resized.
    create_effect(move |_| {
        update_canvas_size(&map_state);

        if !DOCUMENT_LOADED.load(Ordering::Relaxed) {
            DOCUMENT_LOADED.store(true, Ordering::Release);
            let f = Closure::<dyn Fn()>::new(move || update_canvas_size(&map_state));
            window().set_onresize(Some(
                f.as_ref()
                    .unchecked_ref(),
            ));
            f.forget();
        }
    });

    // redraw the canvas if the map state changes.
    create_effect(move |_| {
        let canvas_node = &canvas_ref
            .get()
            .expect("should be loaded now");
        let s = map_state
            .get()
            .get_size();
        canvas_node.set_height(s.0);
        canvas_node.set_width(s.1);

        map_state
            .get()
            .draw_to_canvas(&canvas_ref);
    });

    view! {
        <div class="grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas
                _ref=canvas_ref
                on:mousedown=move |ev| on_mouse_down(&map_state, ev.as_ref())
                on:mouseup=move |_| on_mouse_up(&map_state)
                on:mousemove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:mouseout=move |_| on_mouse_out(&map_state)
                on:touchstart=move |ev| on_mouse_down(&map_state, ev.as_ref())
                on:touchend=move |_| on_mouse_up(&map_state)
                on:touchmove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:touchcancel=move |_| on_mouse_out(&map_state)
                on:wheel=move |ev| on_scroll(ev.delta_y(), &map_state)
                id="canvas"
                style="touch-action: none;"
                class="object-contain"/>
        </div>
    }
}
