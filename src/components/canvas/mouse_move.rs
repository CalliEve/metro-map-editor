//! Contains the mousemove event handler for the [`Canvas`] component.

use leptos::prelude::*;
use web_sys::UiEvent;

use super::other::canvas_click_pos;
use crate::{
    models::GridNode,
    utils::canvas_offset_to_grid_offset,
    MapState,
};

/// Listener for the [mousemove] event on the canvas.
///
/// [mousemove]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
pub fn on_mouse_move(map_state_signal: &RwSignal<MapState>, ev: &UiEvent) {
    let mut map_state = map_state_signal.get();
    let canvas_state = map_state.get_canvas_state();
    let canvas_pos = canvas_click_pos(canvas_state.get_size(), ev);
    let mouse_pos = GridNode::from_canvas_pos(canvas_pos, canvas_state);

    // Handle move of selected line.
    if !map_state
        .get_selected_lines()
        .is_empty()
    {
        for selected in map_state.get_mut_selected_lines() {
            if selected.get_current_hover() != mouse_pos {
                selected.set_current_hover(mouse_pos);
            }
        }
        map_state_signal.set(map_state);
        return;
    }

    // Handle a move while having a new station selected.
    if let Some(selected) = map_state
        .get_mut_selected_stations()
        .first_mut()
    {
        if selected.is_new() {
            selected.update_pos(mouse_pos);
            map_state_signal.set(map_state);
            return;
        }
    }

    // Handle move of selected stations.
    if let Some((drag_origin, true)) = map_state.get_drag_offset() {
        let grid_offset = canvas_offset_to_grid_offset(
            (
                canvas_pos.0 - drag_origin.0,
                canvas_pos.1 - drag_origin.1,
            ),
            map_state
                .get_canvas_state()
                .drawn_square_size(),
        );

        for selected in map_state.get_mut_selected_stations() {
            if let Some(original_pos) = selected.get_original_position() {
                selected.update_pos(original_pos + grid_offset.into());
            } else {
                selected.update_pos(mouse_pos);
            }
        }

        map_state_signal.set(map_state);
        return;
    }

    // Handle the map as a whole getting dragged.
    if let Some((drag_origin, false)) = map_state.get_drag_offset() {
        let grid_offset = canvas_offset_to_grid_offset(
            (
                canvas_pos.0 - drag_origin.0,
                canvas_pos.1 - drag_origin.1,
            ),
            map_state
                .get_canvas_state()
                .drawn_square_size(),
        );

        if grid_offset == (0, 0) {
            return;
        }

        let current_offset = map_state
            .get_canvas_state()
            .get_offset();

        map_state.update_canvas_state(|canvas| {
            canvas.set_offset((
                current_offset.0 - grid_offset.0,
                current_offset.1 - grid_offset.1,
            ));
        });

        map_state.set_drag_offset(Some((canvas_pos, false)));
        map_state_signal.set(map_state);
        return;
    }

    // Handle the size of the box select changing.
    if map_state
        .get_box_select()
        .is_some()
    {
        map_state.update_box_select_end(canvas_pos);
        map_state_signal.set(map_state);
    }
}
