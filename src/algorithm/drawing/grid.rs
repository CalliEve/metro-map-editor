//! Contains everything for depicting the grid onto the canvas.

use super::CanvasContext;
use crate::components::CanvasState;

/// Draws the grid onto the canvas based on the given screen size and grid
/// square size. This should be called before anything else is drawn, so the
/// grid is in the background.
pub fn draw_grid(canvas: &CanvasContext, state: CanvasState) {
    canvas.begin_path();
    canvas.set_line_width(0.3);
    canvas.set_stroke_style_str("grey");

    let (height, width) = state.get_size();
    let drawn_square_size = state.drawn_square_size();

    draw_vertical_lines(
        canvas,
        height,
        drawn_square_size,
        f64::from(width) / drawn_square_size,
    );
    draw_horizontal_lines(
        canvas,
        width,
        drawn_square_size,
        f64::from(height) / drawn_square_size,
    );

    canvas.stroke();
}

/// Draw all vertical grid lines
fn draw_vertical_lines(canvas: &CanvasContext, length: f64, square_size: f64, count: f64) {
    for i in 0..(count
        .round()
        .abs() as u32)
    {
        let x = f64::from(i) * square_size + square_size;
        canvas.move_to(x, 0.0);
        canvas.line_to(x, length);
    }
}

/// Draw all horizontal grid lines
fn draw_horizontal_lines(canvas: &CanvasContext, length: f64, square_size: f64, count: f64) {
    for i in 0..(count
        .round()
        .abs() as u32)
    {
        let y = f64::from(i) * square_size + square_size;
        canvas.move_to(0.0, y);
        canvas.line_to(length, y);
    }
}
