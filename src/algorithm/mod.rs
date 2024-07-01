//! Contains all methods involving the algorithm itself

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
};

mod a_star;
mod closest_corner;
mod draw_edge;
mod grid;

pub use a_star::run_a_star;
pub use draw_edge::draw_edge;
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

    draw_grid(&context, state.get_canvas_state());

    let draw_drawable = |d: &dyn Drawable| d.draw(&context, state.get_canvas_state());

    state
        .get_map()
        .inspect(|d| draw_drawable(*d));

    state
        .get_selected_station()
        .inspect(|d| draw_drawable(*d));

    state
        .get_selected_line()
        .inspect(|d| draw_drawable(*d));
}
