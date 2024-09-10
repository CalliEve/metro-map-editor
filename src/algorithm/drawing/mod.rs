use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
};

mod draw_edge;
mod grid;

pub use draw_edge::draw_edge;
use grid::draw_grid;

use crate::components::MapState;

/// Redraws the given canvas based on the given state
pub fn redraw_canvas(canvas: &HtmlCanvasElement, state: &MapState) {
    // Get a 2d canvas rendering context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

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
