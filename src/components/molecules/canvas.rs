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
    components::{
        state::ActionType,
        CanvasState,
        MapState,
    },
    models::{
        EdgeID,
        GridNode,
        Map,
        SelectedLine,
        SelectedStation,
    },
    utils::line_sections::trace_line_section,
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
    logging::log!(
        "new canvas size: ({}, {})",
        height,
        width
    );
    map_state.update(|state| state.update_canvas_state(|canvas| canvas.set_size((height, width))));
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

/// Helper function for recalculating an edge nodes.
fn recalculate_edge_nodes(map: &mut Map, edge_id: EdgeID) {
    let edge = map
        .get_edge(edge_id)
        .cloned()
        .expect("edge should exist");
    let mut edge = edge.clone();
    edge.calculate_nodes(map);
    map.add_edge(edge);
}

/// Listener for the [mousedown] event on the canvas.
///
/// [mousedown]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
fn on_mouse_down(map_state: &mut MapState, ev: &UiEvent) {
    // Actions are only performed on mouseup
    if map_state
        .get_selected_action()
        .is_some()
    {
        return;
    }

    let mut map = map_state
        .get_map()
        .clone();
    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

    // Handle a click while having a new station selected.
    if let Some(selected) = map_state
        .get_selected_station()
        .cloned()
    {
        let mut new_station = selected.deselect();
        new_station.set_pos(mouse_pos);
        new_station.set_original_pos(mouse_pos);

        map.add_station(new_station);
        map_state.clear_selected_station();
        map_state.set_map(map);
        return;
    }

    // Handle a click while having a new line selected
    if let Some(selected_line) = map_state
        .get_selected_line()
        .copied()
    {
        if let Some(station_at_pos) = map.station_at_node(mouse_pos) {
            let (before, after) = selected_line.get_before_after();
            let mut line = map
                .get_or_add_line(selected_line.get_line())
                .clone();

            line.add_station(&mut map, station_at_pos, before, after);

            map.add_line(line);
            map_state.set_map(map);
            map_state.clear_selected_line();
        }
        return;
    }

    if let Some(mut selected_station) = map
        .station_at_node(mouse_pos)
        .and_then(|s| map.get_station(s))
        .cloned()
        .map(SelectedStation::new)
    {
        for line in map.get_lines() {
            let (before, after) = line.get_station_neighbors(
                &map,
                selected_station
                    .get_station()
                    .get_id(),
            );
            selected_station.add_before(before);
            selected_station.add_after(after);
        }

        selected_station
            .get_station()
            .print_info();

        map_state.set_selected_station(selected_station);
        return;
    }

    if let Some(selected_line) = map
        .line_at_node(mouse_pos)
        .cloned()
        .map(|l| SelectedLine::new(&l, &map, mouse_pos, Some(mouse_pos)))
    {
        map_state.set_selected_line(selected_line);
        for edge in map.get_edges() {
            if edge
                .get_nodes()
                .contains(&mouse_pos)
            {
                edge.print_info();
                break;
            }
        }
    }
}

/// Listener for the [mouseup] event on the canvas.
///
/// [mouseup]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
fn on_mouse_up(map_state: &mut MapState, ev: &UiEvent, shift_key: bool) {
    let mut map = map_state
        .get_map()
        .clone();

    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

    // Handle a click while having an operation selected
    if let Some(action_type) = map_state.get_selected_action() {
        match action_type {
            ActionType::RemoveStation => {
                if let Some(station_id) = map.station_at_node(mouse_pos) {
                    map.remove_station(station_id);
                }
            },
            ActionType::RemoveLine => {
                if let Some(selected_line) = map.line_at_node(mouse_pos) {
                    map.remove_line(selected_line.get_id());
                }
            },
            ActionType::LockStation => {
                if let Some(station_id) = map.station_at_node(mouse_pos) {
                    map.get_mut_station(station_id)
                        .expect("Found station but id does not exit")
                        .lock();
                }
            },
            ActionType::UnlockStation => {
                if let Some(station_id) = map.station_at_node(mouse_pos) {
                    map.get_mut_station(station_id)
                        .expect("Found station but id does not exit")
                        .unlock();
                }
            },
            ActionType::LockEdge => {
                if let Some(edge_id) = map.edge_at_node(mouse_pos) {
                    map.get_mut_edge(edge_id)
                        .expect("Found edge but id does not exit")
                        .lock();
                }
            },
            ActionType::UnlockEdge => {
                if let Some(edge_id) = map.edge_at_node(mouse_pos) {
                    map.get_mut_edge(edge_id)
                        .expect("Found edge but id does not exit")
                        .unlock();
                }
            },
        }
        map_state.set_map(map);
        if !shift_key {
            map_state.clear_selected_action();
        }
        return;
    }

    // Handle a mouseup while having a line selected
    if let Some(selected_line) = map_state
        .get_selected_line()
        .copied()
    {
        if let Some(station_at_pos) = map.station_at_node(mouse_pos) {
            let (before, after) = selected_line.get_before_after();
            let mut line = map
                .get_or_add_line(selected_line.get_line())
                .clone();

            line.add_station(&mut map, station_at_pos, before, after);

            if let Some(before_station) = before {
                let edge_id = map.get_edge_id_between(before_station, station_at_pos);
                recalculate_edge_nodes(&mut map, edge_id);
            }

            if let Some(after_station) = after {
                let edge_id = map.get_edge_id_between(station_at_pos, after_station);
                recalculate_edge_nodes(&mut map, edge_id);
            }

            map.add_line(line);
            map_state.set_map(map);
            map_state.clear_selected_line();
            return;
        }

        map_state.clear_selected_line();

        if let Some(grabbed_at) = selected_line.get_grabbed_at() {
            if grabbed_at != mouse_pos {
                return;
            }
        }
    }

    // Handle a mouseup while having a station selected
    if let Some(selected_station) = map_state
        .get_selected_station()
        .cloned()
        .map(SelectedStation::deselect)
    {
        let mut edge_ids = Vec::new();
        for station in map.get_mut_stations() {
            if *station == selected_station {
                if station.get_pos() == selected_station.get_pos() {
                    break;
                }

                station.set_pos(selected_station.get_pos());
                station.set_original_pos(selected_station.get_pos());
                station.lock();
                edge_ids = station
                    .get_edges()
                    .to_vec();
                break;
            }
        }

        for edge_id in edge_ids {
            recalculate_edge_nodes(&mut map, edge_id);
        }

        map_state.set_map(map);
        map_state.clear_selected_station();
        return;
    }

    // Select the clicked edge, unless this was a double click.
    if let Some(edge_id) = map.edge_at_node(mouse_pos) {
        if ev.detail() > 1 {
            return;
        }

        for selected_id in map_state.get_selected_edges() {
            map.get_mut_edge(*selected_id)
                .expect("selected edge should exist")
                .deselect();
        }

        map_state.set_selected_edges(vec![edge_id]);
        map.get_mut_edge(edge_id)
            .expect("edge should exist")
            .select();

        map_state.set_map(map);
        return;
    }

    // Someone clicked on an empty node, deselect the currently selected edges
    if !map_state
        .get_selected_edges()
        .is_empty()
    {
        map_state.clear_selected_edges();
    }
}

/// Listener for the [mousemove] event on the canvas.
///
/// [mousemove]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
fn on_mouse_move(map_state_signal: &RwSignal<MapState>, ev: &UiEvent) {
    let mut map_state = map_state_signal.get();
    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

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

/// Listener for the [dblclick] event on the canvas.
///
/// [dblclick]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event
fn on_dblclick(map_state: &mut MapState, ev: &UiEvent) {
    let map = map_state.get_map();

    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

    if let Some(edge_id) = map.edge_at_node(mouse_pos) {
        map_state.set_selected_edges(
            trace_line_section(map, edge_id, false)
                .into_iter()
                .map(|e| e.get_id())
                .collect(),
        );
    } else {
        map_state.clear_selected_edges();
    }
}

/// Listener for when the user scrolls on the canvas.
fn on_scroll(map_state: &mut MapState, amount: f64) {
    if amount > 0.0 {
        map_state.update_canvas_state(CanvasState::zoom_in);
    } else {
        map_state.update_canvas_state(CanvasState::zoom_out);
    };
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
            .get_canvas_state()
            .get_size();
        canvas_node.set_height(s.0);
        canvas_node.set_width(s.1);

        map_state
            .get()
            .draw_to_canvas(&canvas_ref);
    });

    view! {
        <div class="absolute grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas
                _ref=canvas_ref

                on:mousedown=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref()))
                on:mouseup=move |ev| map_state.update(|state| on_mouse_up(state, ev.as_ref(), ev.shift_key()))
                on:mousemove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:mouseout=move |_| map_state.update(on_mouse_out)
                on:dblclick=move |ev| map_state.update(|state| on_dblclick(state, ev.as_ref()))

                on:touchstart=move |ev| map_state.update(|state| on_mouse_down(state, ev.as_ref()))
                on:touchend=move |ev| map_state.update(|state| on_mouse_up(state, ev.as_ref(), ev.shift_key()))
                on:touchmove=move |ev| on_mouse_move(&map_state, ev.as_ref())
                on:touchcancel=move |_| map_state.update(on_mouse_out)

                on:wheel=move |ev| map_state.update(|state| on_scroll(state, ev.delta_y()))

                id="canvas"
                style="touch-action: none;"
                class="object-contain"/>
        </div>
    }
}
