//! Contains the scroll event handler for the [`Canvas`] component.

use crate::{
    CanvasState,
    MapState,
};

/// Listener for when the user scrolls on the canvas.
pub fn on_scroll(map_state: &mut MapState, amount: f64) {
    if amount > 0.0 {
        map_state.update_canvas_state(CanvasState::zoom_in);
    } else {
        map_state.update_canvas_state(CanvasState::zoom_out);
    };
}
