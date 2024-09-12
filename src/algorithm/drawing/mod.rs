//! This module contains all functions for drawing the [`crate::models::Map`] to
//! the canvas.

mod canvas_context;
mod draw_edge;
mod grid;
mod closest_corner;

pub use canvas_context::CanvasContext;
pub use draw_edge::draw_edge;
use grid::draw_grid;
use closest_corner::calc_closest_corner;

use crate::components::MapState;

/// Redraws the given canvas based on the given state
pub fn redraw_canvas<'a, C>(canvas: C, state: &MapState)
where
    C: Into<CanvasContext<'a>>,
{
    // Get a 2d canvas rendering context
    let context: CanvasContext = canvas.into();

    draw_grid(&context, state.get_canvas_state());

    let map = state.get_map();

    map.draw(&context, state.get_canvas_state());

    state
        .get_selected_station()
        .inspect(|d| d.draw(map, &context, state.get_canvas_state()));

    state
        .get_selected_line()
        .inspect(|d| d.draw(map, &context, state.get_canvas_state()));
}
