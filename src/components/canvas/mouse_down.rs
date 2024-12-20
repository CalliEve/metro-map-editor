//! Contains the mousedown event handler for the [`Canvas`] component.

use web_sys::UiEvent;

use super::other::canvas_click_pos;
use crate::{
    models::{
        GridNode,
        SelectedLine,
        SelectedStation,
    },
    MapState,
};

/// Listener for the [mousedown] event on the canvas.
///
/// [mousedown]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
pub fn on_mouse_down(map_state: &mut MapState, ev: &UiEvent, shift_key: bool) {
    if ev.detail() > 1 {
        return;
    }

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
    let station_at_node = map.station_at_node(mouse_pos);
    let edge_at_node = map.edge_at_node(mouse_pos);

    // Handle a click while having a new station selected.
    if let Some(selected) = map_state
        .get_selected_stations()
        .first()
        .cloned()
    {
        if selected.is_new() {
            let mut new_station = selected.deselect();
            new_station.set_pos(mouse_pos);
            new_station.set_original_pos(mouse_pos);

            map.add_station(new_station);
            map_state.clear_selected_stations();
            map_state.set_map(map);
            return;
        }
    }

    // Handle a click while having a new line selected
    let selected_lines = map_state
        .get_selected_lines()
        .to_vec();
    if !selected_lines.is_empty() {
        for selected_line in selected_lines {
            if let Some(station_at_pos) = station_at_node {
                let (before, after) = selected_line.get_before_after();
                let mut line = map
                    .get_or_add_line(selected_line.get_line())
                    .clone();

                line.add_station(&mut map, station_at_pos, before, after);

                map.add_line(line);
            }
        }
        map_state.set_map(map);
        map_state.clear_selected_lines();
        return;
    }

    // Handle a click on an edge has been selected
    if let Some(selected_edge) = edge_at_node {
        if map_state
            .get_selected_edges()
            .contains(&selected_edge)
        {
            map_state.set_drag_offset(Some((canvas_pos, true)));
        }
    }
    // Handle a click on a station has been selected
    if let Some(selected_station) = station_at_node {
        if map_state
            .get_selected_stations()
            .iter()
            .any(|s| {
                s.get_station()
                    .get_id()
                    == selected_station
            })
        {
            map_state.set_drag_offset(Some((canvas_pos, true)));
        }
    }

    // Handle a click on a station
    if let Some(mut selected_station) = station_at_node
        .and_then(|s| map.get_station(s))
        .cloned()
        .map(SelectedStation::new)
    {
        selected_station
            .get_station()
            .print_info();

        if map_state
            .get_selected_stations()
            .contains(&selected_station)
        {
            return;
        }

        #[allow(clippy::unnecessary_to_owned)] // otherwise conflicts with mutable borrow
        for edge_id in selected_station
            .get_station()
            .get_edges()
            .to_vec()
        {
            let edge = map
                .get_edge(edge_id)
                .cloned()
                .expect("edge should exist");

            if edge.get_from()
                == selected_station
                    .get_station()
                    .get_id()
            {
                selected_station.add_after(edge.get_to());
            } else {
                selected_station.add_before(edge.get_from());
            }
        }

        if !shift_key {
            map_state.clear_selected_stations();
        }

        map_state.select_station(selected_station);
        map_state.set_drag_offset(Some((canvas_pos, true)));
        return;
    }

    // Handle a click on a line
    let selected_lines = map
        .lines_at_node(mouse_pos)
        .into_iter()
        .map(|l| SelectedLine::new(&l, &map, mouse_pos, Some(mouse_pos)))
        .collect::<Vec<_>>();
    if !selected_lines.is_empty() {
        map_state.set_selected_lines(selected_lines);
    }

    // Select the clicked edge, unless this was a double click.
    if let Some(edge_id) = map.edge_at_node(mouse_pos) {
        if map_state
            .get_selected_edges()
            .contains(&edge_id)
        {
            return;
        }

        if shift_key {
            map_state.select_edge(edge_id);
        } else {
            map_state.set_selected_edges(vec![edge_id]);
        }

        map_state.set_drag_offset(Some((canvas_pos, true)));
        return;
    }

    // If clicking outside of anything with shiftkey down, then we're doing a
    // box-select.
    if shift_key
        && map_state
            .get_drag_offset()
            .is_none()
    {
        map_state.set_box_select_start(canvas_pos);
        return;
    }

    // Then we are not dragging anything, but instead possibly the canvas as a whole
    if map_state
        .get_drag_offset()
        .is_none()
    {
        map_state.set_drag_offset(Some((canvas_pos, false)));
    }
}
