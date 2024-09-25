//! Contains all methods involving the map algorithm itself and drawing the map
//! to the canvas.

mod a_star;
mod calc_direction;
pub mod drawing;
mod edge_dijkstra;
mod order_edges;
mod recalculate_map;
mod route_edges;
mod utils;

pub use a_star::run_a_star;
pub use recalculate_map::recalculate_map;
use utils::*;

/// Stores the settings for the algorithm.
#[derive(Clone, Copy, Debug)]
pub struct AlgorithmSettings {
    /// The size of the radius around a station to possibly route edges to for
    /// the possible new station location.
    /// Default: 3
    pub node_set_radius: i32,
    /// Max amount of attempts allowed of routing edges before erroring out.
    /// Default: 5
    pub edge_routing_attempts: usize,
}

impl Default for AlgorithmSettings {
    fn default() -> Self {
        Self {
            node_set_radius: 3,
            edge_routing_attempts: 5,
        }
    }
}
