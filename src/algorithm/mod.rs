use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod grid;

use grid::draw_grid;

pub fn redraw_canvas(canvas: &HtmlCanvasElement, size: (u32, u32)) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    draw_grid(&context, size, 30);
}
