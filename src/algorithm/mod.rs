use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod grid;
mod objects;

use grid::draw_grid;
pub use objects::*;

use crate::state::MapState;

pub fn redraw_canvas(canvas: &HtmlCanvasElement, state: &MapState) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    draw_grid(&context, state.get_size(), state.get_square_size());

    state
        .get_map()
        .inspect(|m| m.draw(&context, state.get_square_size()));

    state
        .get_selected_station()
        .inspect(|s| s.draw(&context, state.get_square_size()));
}
