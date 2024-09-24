//! Contains all methods involving the map algorithm itself and drawing the map
//! to the canvas.

mod a_star;
mod calc_direction;
pub mod drawing;
mod order_edges;
mod recalculate_map;

pub use a_star::run_a_star;
pub use recalculate_map::recalculate_map;
