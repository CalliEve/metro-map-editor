//! This module contains all functions for drawing the [`crate::models::Map`] to
//! the canvas.

mod calc_direction;
mod canvas_context;
mod closest_corner;
mod draw_edge;
mod grid;
mod labeling;

pub use canvas_context::CanvasContext;
use closest_corner::calc_closest_corner;
pub use draw_edge::draw_edge;
use grid::draw_grid;
pub use labeling::calc_label_pos;

use crate::components::MapState;

/// Redraws the given canvas based on the given state
pub fn redraw_canvas<'a, C>(canvas: C, state: &MapState)
where
    C: Into<CanvasContext<'a>>,
{
    // Get a 2d canvas rendering context
    let context: CanvasContext = canvas.into();

    draw_grid(&context, state.get_canvas_state());

    if state.is_original_overlay_enabled() {
        if let Some(original) = state.get_last_loaded() {
            original.draw(&context, state.get_canvas_state(), 0.3);
        }
    }

    let map = state.get_map();

    map.draw(&context, state.get_canvas_state(), 1.0);

    state
        .get_selected_station()
        .inspect(|d| d.draw(map, &context, state.get_canvas_state()));

    state
        .get_selected_line()
        .inspect(|d| d.draw(map, &context, state.get_canvas_state()));
}
