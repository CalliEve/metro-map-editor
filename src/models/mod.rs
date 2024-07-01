//! Contains all structs used to represent the metro map within the algorithm.

use web_sys::CanvasRenderingContext2d;

mod grid_node;
mod line;
mod map;
mod selected_line;
mod selected_station;
mod station;

pub use grid_node::GridNode;
pub use line::Line;
pub use map::Map;
pub use selected_line::SelectedLine;
pub use selected_station::SelectedStation;
pub use station::Station;

use crate::components::CanvasState;

/// This trait signifies an object that can be drawn onto the canvas.
pub trait Drawable {
    /// Draw the object to the given canvas, which has a grid with the given
    /// square size
    fn draw(&self, canvas: &CanvasRenderingContext2d, state: CanvasState);
}
