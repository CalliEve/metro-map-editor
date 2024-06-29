//! Contains everything for depicting the grid onto the canvas.

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

/// Draws the grid onto the canvas based on the given screen size and grid
/// square size. This should be called before anything else is drawn, so the
/// grid is in the background.
pub fn draw_grid(canvas: &CanvasRenderingContext2d, size: (u32, u32), square_size: u32) {
    canvas.begin_path();
    canvas.set_line_width(0.3);
    canvas.set_stroke_style(&JsValue::from_str("grey"));

    draw_vertical_lines(
        canvas,
        size.0,
        square_size,
        size.1 / square_size,
    );
    draw_horizontal_lines(
        canvas,
        size.1,
        square_size,
        size.0 / square_size,
    );

    canvas.stroke();
}

/// Draw all vertical grid lines
fn draw_vertical_lines(
    canvas: &CanvasRenderingContext2d,
    length: u32,
    square_size: u32,
    count: u32,
) {
    for i in 0..count {
        let x = f64::from(i * square_size + square_size);
        canvas.move_to(x, 0.0);
        canvas.line_to(x, f64::from(length));
    }
}

/// Draw all horizontal grid lines
fn draw_horizontal_lines(
    canvas: &CanvasRenderingContext2d,
    length: u32,
    square_size: u32,
    count: u32,
) {
    for i in 0..count {
        let y = f64::from(i * square_size + square_size);
        canvas.move_to(0.0, y);
        canvas.line_to(f64::from(length), y);
    }
}
