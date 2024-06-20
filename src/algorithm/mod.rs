//! Contains all methods involving the algorithm itself

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
};

mod grid;

use grid::draw_grid;

use crate::{
    components::MapState,
    models::Drawable,
};

/// Redraws the given canvas based on the given state
pub fn redraw_canvas(canvas: &HtmlCanvasElement, state: &MapState) {
    // Get a 2d canvas rendering context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    draw_grid(
        &context,
        state.get_size(),
        state.get_square_size(),
    );

    state
        .get_map()
        .inspect(|m| m.draw(&context, state.get_square_size()));

    state
        .get_selected_station()
        .inspect(|s| s.draw(&context, state.get_square_size()));
}
