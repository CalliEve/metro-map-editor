//! Contains all methods involving the map algorithm itself and drawing the map
//! to the canvas.

mod a_star;
mod calc_direction;
mod closest_corner;
pub mod drawing;

pub use a_star::run_a_star;
