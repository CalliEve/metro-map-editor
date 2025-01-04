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
    // Get a canvas rendering context
    let context: CanvasContext = canvas.into();

    // If we're offscreen, then this is for an image of the map to get downloaded,
    // so we only need to draw the map and no grid.
    if !context.is_onscreen() {
        let map = state
            .get_map()
            .without_checkpoints();

        map.draw(&context, state.get_canvas_state(), 1.0);

        return;
    }

    draw_grid(&context, state.get_canvas_state());

    if state.is_original_overlay_enabled() {
        if let Some(original) = state.get_last_loaded() {
            original.draw(&context, state.get_canvas_state(), 0.3);
        }
    }

    let map = state.get_map();

    state
        .get_selected_stations()
        .iter()
        .for_each(|d| {
            d.draw(
                map,
                &context,
                state.get_canvas_state(),
                state.get_selected_stations(),
            );
        });

    map.draw(&context, state.get_canvas_state(), 1.0);

    state
        .get_selected_lines()
        .iter()
        .for_each(|d| d.draw(map, &context, state.get_canvas_state()));

    state
        .get_box_select()
        .inspect(|(start, end)| draw_box_select(&context, *start, *end));
}

/// Draws a box select overlay on the canvas.
fn draw_box_select(context: &CanvasContext, start: (f64, f64), end: (f64, f64)) {
    context.set_stroke_style_str("black");
    context
        .set_line_dash(&[10, 2, 2, 2])
        .unwrap();
    context.set_line_width(1.0);
    context.set_global_alpha(0.6);

    context.begin_path();
    context.rect(
        start.0,
        start.1,
        end.0 - start.0,
        end.1 - start.1,
    );
    context.stroke();
}
