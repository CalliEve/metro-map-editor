use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod grid;
mod objects;
mod utils;

use grid::draw_grid;
pub use objects::*;

pub fn redraw_canvas(
    canvas: &HtmlCanvasElement,
    size: (u32, u32),
    map: Option<&Map>,
    hover_station: Option<&Station>,
) {
    let square_size = map.as_ref().map_or(30, |m| m.get_square_size());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    draw_grid(&context, size, square_size);

    map.inspect(|m| m.draw(&context, square_size));

    hover_station.inspect(|s| s.draw(&context, square_size));
}
