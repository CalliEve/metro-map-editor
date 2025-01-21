//! This module contains all the complex algorithms used in the project.

mod drawing;
mod line_straightening;
mod map_layout;

mod a_star;
mod calc_direction;
mod occupation;
mod utils;

pub use a_star::run_a_star;
use calc_direction::EdgeDirection;
pub use drawing::*;
pub use line_straightening::*;
pub use map_layout::*;
use occupation::diagonal_occupied;
pub use occupation::{
    OccupiedNode,
    OccupiedNodes,
};
pub use utils::{
    log_print,
    LogType,
};
use utils::{
    node_outside_grid,
    overlap_amount,
    randomize_edges,
    unsettle_map,
};
