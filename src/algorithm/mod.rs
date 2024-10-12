//! Contains all methods involving the map algorithm itself and drawing the map
//! to the canvas.
use serde::{
    Deserialize,
    Serialize,
};

mod a_star;
mod cost_calculation;
pub mod drawing;
mod edge_dijkstra;
mod local_search;
mod occupation;
mod order_edges;
mod recalculate_map;
mod route_edges;
mod station_contraction;
mod utils;

pub use a_star::run_a_star;
pub use recalculate_map::recalculate_map;
use utils::*;

/// Stores the settings for the algorithm.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
    /// Whether to print debug information.
    pub debug: bool,
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

    /// Toggle the debug mode.
    pub fn toggle_debug(mut self) -> Self {
        self.debug = !self.debug;
        self
    }
}

impl Default for AlgorithmSettings {
    fn default() -> Self {
        Self {
            node_set_radius: 3,
            edge_routing_attempts: 3,
            move_cost: 1.0,
            debug: false,
            grid_x_limits: (i32::MIN, i32::MAX),
            grid_y_limits: (i32::MIN, i32::MAX),
        }
    }
}
