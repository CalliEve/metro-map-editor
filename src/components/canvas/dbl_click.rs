//! Contains the dblclick event handler for the [`Canvas`] component.

use web_sys::UiEvent;

use super::other::canvas_click_pos;
use crate::{
    models::{
        GridNode,
        SelectedStation,
    },
    utils::line_sections::{
        get_line_section_parts,
        trace_line_section,
    },
    MapState,
};

/// Listener for the [dblclick] event on the canvas.
///
/// [dblclick]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event
pub fn on_dbl_click(map_state: &mut MapState, ev: &UiEvent) {
    let map = map_state.get_map();

    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

    if let Some(edge_id) = map.edge_at_node(mouse_pos) {
        let line_section = trace_line_section(map, edge_id, false);

        let (_, middles) = get_line_section_parts(&line_section);

        map_state.set_selected_stations(
            middles
                .into_iter()
                .map(|s| {
                    SelectedStation::new(
                        map.get_station(s)
                            .expect("Station in line section does not exist.")
                            .clone(),
                    )
                })
                .collect(),
        );

        map_state.set_selected_edges(
            line_section
                .into_iter()
                .map(|e| e.get_id())
                .collect(),
        );
    } else {
        map_state.clear_all_selections();
    }
}
