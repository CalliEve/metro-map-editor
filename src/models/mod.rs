//! Contains all structs used to represent the metro map within the algorithm.

use web_sys::CanvasRenderingContext2d;

mod line;
mod map;
mod selected_line;
mod station;

pub use line::Line;
pub use map::Map;
pub use selected_line::SelectedLine;
pub use station::Station;

/// This trait signifies an object that can be drawn onto the canvas.
pub trait Drawable {
    /// Draw the object to the given canvas, which has a grid with the given
    /// square size
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32);
}
