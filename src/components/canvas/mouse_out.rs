//! Contains the mouseout event handler for the [`Canvas`] component.

use crate::MapState;

/// Listener for the [mouseout] event on the canvas.
///
/// [mouseout]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event
pub fn on_mouse_out(map_state: &mut MapState) {
    map_state.clear_selected_lines();
    map_state.clear_box_select();
    map_state.clear_drag_offset();
}
