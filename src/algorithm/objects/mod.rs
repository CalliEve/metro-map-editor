use web_sys::CanvasRenderingContext2d;

mod line;
mod map;
mod station;

pub use line::Line;
pub use map::Map;
pub use station::Station;

pub trait Drawable {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32);
}
