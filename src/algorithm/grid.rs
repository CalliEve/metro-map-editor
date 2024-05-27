use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub fn draw_grid(canvas: &CanvasRenderingContext2d, size: (u32, u32), square_size: u32) {
    canvas.set_line_width(1.0);
    canvas.set_stroke_style(&JsValue::from_str("grey"));

    draw_vertical_lines(canvas, size.0, square_size, size.1 / square_size);
    draw_horizontal_lines(canvas, size.1, square_size, size.0 / square_size);
}

fn draw_vertical_lines(
    canvas: &CanvasRenderingContext2d,
    length: u32,
    square_size: u32,
    count: u32,
) {
    for i in 0..count {
        let x = (i * square_size + square_size) as f64;
        canvas.begin_path();
        canvas.move_to(x, 0.0);
        canvas.line_to(x, length as f64);
        canvas.stroke();
    }
}

fn draw_horizontal_lines(
    canvas: &CanvasRenderingContext2d,
    length: u32,
    square_size: u32,
    count: u32,
) {
    for i in 0..count {
        let y = (i * square_size + square_size) as f64;
        canvas.begin_path();
        canvas.move_to(0.0, y);
        canvas.line_to(length as f64, y);
        canvas.stroke();
    }
}
