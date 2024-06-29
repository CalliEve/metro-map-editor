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
    models::{
        GridNode,
        SelectedLine,
        SelectedStation,
        Station,
    },
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
fn on_mouse_down(map_state: &mut MapState, ev: &UiEvent) {
    let Some(mut map) = map_state
        .get_map()
        .cloned()
    else {
        return;
    };

    // Handle a click while having a new station selected.
    if let Some(selected) = map_state
        .get_selected_station()
        .cloned()
    {
        map.add_station(selected.deselect());
        map_state.clear_selected_station();
        map_state.set_map(map);
        return;
    }

    let canvas_pos = canvas_click_pos(map_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, map_state.get_square_size());

    // Handle a click while having a new line selected
    if let Some(selected_line) = map_state
        .get_selected_line()
        .cloned()
    {
        if let Some(station_at_pos) = map
            .station_at_node(mouse_pos)
            .cloned()
        {
            let (before, _) = selected_line.get_before_after();
            let mut line = selected_line
                .get_line()
                .clone();

            line.add_station(station_at_pos, before);

            map.add_line(line);
            map_state.set_map(map);
            map_state.clear_selected_line();
        }
        return;
    }

    if let Some(mut selected_station) = map
        .station_at_node(mouse_pos)
        .map(Station::clone_non_ref)
        .map(SelectedStation::new)
    {
        for line in map.get_lines() {
            let (before, after) = line.get_neighbors(
                selected_station
                    .get_station()
                    .get_pos(),
            );
            if let Some(before) = before {
                selected_station.add_before(before);
            }
            if let Some(after) = after {
                selected_station.add_after(after);
            }
        }

        map_state.set_selected_station(selected_station);
        return;
    }

    if let Some(selected_line) = map
        .line_at_node(mouse_pos)
        .cloned()
        .map(|l| SelectedLine::new(l, mouse_pos, Some(mouse_pos)))
    {
        map_state.set_selected_line(selected_line);
    }
}

/// Listener for the [mouseup] event on the canvas.
///
/// [mouseup]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
fn on_mouse_up(map_state: &mut MapState, ev: &UiEvent) {
    let mut map = map_state
        .get_map()
        .cloned()
        .unwrap();

    // Handle a mouseup while having a line selected
    if let Some(selected_line) = map_state
        .get_selected_line()
        .cloned()
    {
        let canvas_pos = canvas_click_pos(map_state.get_size(), ev);
        let mouse_pos = GridNode::from_canvas_pos(canvas_pos, map_state.get_square_size());

        if let Some(station_at_pos) = map
            .station_at_node(mouse_pos)
            .cloned()
        {
            let (before, _) = selected_line.get_before_after();
            let mut line = selected_line
                .get_line()
                .clone();

            line.add_station(station_at_pos, before);

            map.add_line(line);
            map_state.set_map(map);
        }

        map_state.clear_selected_line();
        return;
    }

    // Handle a mouseup while having a station selected
    let Some(selected_station) = map_state
        .get_selected_station()
        .cloned()
        .map(SelectedStation::deselect)
    else {
        return;
    };

    for station in map.get_mut_stations() {
        if *station == selected_station {
            station.set_pos(selected_station.get_pos());
            break;
        }
    }

    map_state.set_map(map);
    map_state.clear_selected_station();
}

/// Listener for the [mousemove] event on the canvas.
///
/// [mousemove]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
fn on_mouse_move(map_state_signal: &RwSignal<MapState>, ev: &UiEvent) {
    let mut map_state = map_state_signal.get();
    let canvas_pos = canvas_click_pos(map_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, map_state.get_square_size());

    // Handle move of selected line
    if let Some(selected) = map_state.get_mut_selected_line() {
        if selected.get_current_hover() != mouse_pos {
            selected.set_current_hover(mouse_pos);
            map_state_signal.set(map_state);
        }
        return;
    }

    // Handle move of selected station
    let Some(mut selected) = map_state
        .get_selected_station()
        .cloned()
    else {
        return;
    };

    if mouse_pos == selected.get_pos() {
        return;
    }

    selected.update_pos(mouse_pos);
    map_state.set_selected_station(selected);
    map_state_signal.set(map_state);
}

/// Listener for the [mouseout] event on the canvas.
///
/// [mouseout]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event
fn on_mouse_out(map_state: &mut MapState) {
    map_state.clear_selected_station();
    map_state.clear_selected_line();
}

/// Listener for when the user scrolls on the canvas.
fn on_scroll(map_state: &mut MapState, amount: f64) {
    let current = map_state.get_square_size();

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

    map_state.set_square_size(size);
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

        map_state.update(MapState::run_local_search);

        map_state
            .get()
            .draw_to_canvas(&canvas_ref);
    });

    view! {
        <div class="grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas
                _ref=canvas_ref

                on:mousedown=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref()))
                on:mouseup=move |ev| map_state.update(|state| on_mouse_up(state, ev.as_ref()))
                on:mousemove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:mouseout=move |_| map_state.update(on_mouse_out)

                on:touchstart=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref()))
                on:touchend=move |ev| map_state.update(|state| on_mouse_up(state, ev.as_ref()))
                on:touchmove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:touchcancel=move |_| map_state.update(on_mouse_out)

                on:wheel=move |ev| map_state.update(|state| on_scroll(state, ev.delta_y()))

                id="canvas"
                style="touch-action: none;"
                class="object-contain"/>
        </div>
    }
}
