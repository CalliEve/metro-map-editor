//! Contains the mouseup event handler for the [`Canvas`] component.

use web_sys::UiEvent;

use super::other::{
    canvas_click_pos,
    recalculate_edge_nodes,
};
use crate::{
    components::state::ActionType,
    models::{
        GridNode,
        SelectedStation,
    },
    MapState,
};

/// Listener for the [mouseup] event on the canvas.
///
/// [mouseup]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
pub fn on_mouse_up(map_state: &mut MapState, ev: &UiEvent, shift_key: bool) {
    if ev.detail() > 1 {
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

    // if we were dragging, we aren't anymore now.
    if map_state
        .get_drag_offset()
        .is_some()
    {
        map_state.set_drag_offset(None);
    }

    // Handle a click while having an operation selected
    if let Some(action_type) = map_state.get_selected_action() {
        match action_type {
            ActionType::RemoveStation => {
                if let Some(station_id) = station_at_node {
                    map.remove_station(station_id);
                }
            },
            ActionType::RemoveLine => {
                if let Some(selected_line) = map.line_at_node(mouse_pos) {
                    map.remove_line(selected_line.get_id());
                }
            },
            ActionType::Lock => {
                if let Some(station_id) = station_at_node {
                    map.get_mut_station(station_id)
                        .expect("Found station but id does not exit")
                        .lock();
                } else if let Some(edge_id) = edge_at_node {
                    map.get_mut_edge(edge_id)
                        .expect("Found edge but id does not exit")
                        .lock();
                }
            },
            ActionType::Unlock => {
                if let Some(station_id) = station_at_node {
                    map.get_mut_station(station_id)
                        .expect("Found station but id does not exit")
                        .unlock();
                } else if let Some(edge_id) = edge_at_node {
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
        if let Some(station_at_pos) = station_at_node {
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
            map_state.clear_all_selections();
            return;
        }

        map_state.clear_selected_line();

        if let Some(grabbed_at) = selected_line.get_grabbed_at() {
            if grabbed_at != mouse_pos {
                map_state.clear_all_selections();
                return;
            } else {
                // Handle a single click on an edge
                if edge_at_node.is_some()
                    && !shift_key
                    && map_state
                        .get_selected_edges()
                        .len()
                        == 1
                {
                    let selected_edge_id = map_state
                        .get_selected_edges()
                        .first()
                        .cloned()
                        .unwrap();

                    let selected_edge = map
                        .get_edge(selected_edge_id)
                        .cloned()
                        .expect("selected edge should exist");

                    map_state.clear_all_selections();
                    map_state.set_clicked_on_edge(selected_edge, canvas_pos);
                    return;
                }
            }
        }
    }

    // Handle a single click on a station
    if station_at_node.is_some()
        && !shift_key
        && map_state
            .get_selected_stations()
            .len()
            == 1
    {
        let selected_station = map_state
            .get_selected_stations()
            .first()
            .cloned()
            .unwrap();

        if !selected_station.has_moved() {
            map_state.clear_all_selections();
            map_state.set_clicked_on_station(
                selected_station
                    .get_station()
                    .clone(),
            );
            return;
        }
    }

    // Handle a mouseup while having a station selected
    if !map_state
        .get_selected_stations()
        .is_empty()
        && !shift_key
        && map_state
            .get_selected_stations()
            .iter()
            .any(SelectedStation::has_moved)
    {
        for selected_station in map_state
            .get_selected_stations()
            .iter()
            .cloned()
            .map(SelectedStation::deselect)
        {
            let station = map
                .get_mut_station(selected_station.get_id())
                .expect("selected station does not exist");

            if station.get_pos() == selected_station.get_pos() {
                continue;
            }

            station.set_pos(selected_station.get_pos());
            station.set_original_pos(selected_station.get_pos());
            station.lock();

            for edge_id in selected_station.get_edges() {
                recalculate_edge_nodes(&mut map, *edge_id);
            }
        }

        map_state.set_map(map);
        map_state.clear_all_selections();
        return;
    }

    // Handle the box-select selecting things.
    if let Some((start_canvas, end_canvas)) = map_state.get_box_select() {
        let start = GridNode::from_canvas_pos(start_canvas, canvas_state);
        let end = GridNode::from_canvas_pos(end_canvas, canvas_state);

        let mut selected_stations = Vec::new();
        let mut selected_edges = Vec::new();

        for station in map.get_stations() {
            let pos = station.get_pos();
            if pos.0 >= start.0 && pos.0 <= end.0 && pos.1 >= start.1 && pos.1 <= end.1 {
                let mut selected_station = SelectedStation::new(station.clone());

                for edge_id in station.get_edges() {
                    let edge = map
                        .get_edge(*edge_id)
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

                map_state.select_station(selected_station);
                selected_stations.push(station.get_id());
            }
        }

        for edge in map.get_edges() {
            if selected_stations.contains(&edge.get_from())
                && selected_stations.contains(&edge.get_to())
            {
                selected_edges.push(edge.get_id());
            }
        }

        map_state.set_selected_edges(selected_edges);
        map_state.clear_box_select();
        return;
    }

    // Someone clicked on an empty node, deselect everything.
    if !shift_key {
        map_state.clear_all_selections();
    }
}
