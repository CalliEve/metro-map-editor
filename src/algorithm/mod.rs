//! Contains all methods involving the map algorithm itself and drawing the map
//! to the canvas.

mod a_star;
mod calc_direction;
mod cost_calculation;
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
    /// The cost of moving from one node to another.
    pub move_cost: f64,
    /// The highest and lowest possible x values of the grid.
    pub grid_x_limits: (i32, i32),
    /// The highest and lowest possible y values of the grid.
    pub grid_y_limits: (i32, i32),
}

impl AlgorithmSettings {
    /// Set the highest and lowest possible x values of the grid.
    pub fn set_grid_x_limits(mut self, x_limits: (i32, i32)) -> Self {
        self.grid_x_limits = x_limits;
        self
    }

    /// Set the highest and lowest possible y values of the grid.
    pub fn set_grid_y_limits(mut self, y_limits: (i32, i32)) -> Self {
        self.grid_y_limits = y_limits;
        self
    }
}

impl Default for AlgorithmSettings {
    fn default() -> Self {
        Self {
            node_set_radius: 3,
            edge_routing_attempts: 3,
            move_cost: 1.0,
            grid_x_limits: (0, 0),
            grid_y_limits: (0, 0),
        }
    }
}
