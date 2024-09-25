//! Utility functions specifically for the algorithm module.

use rand::{
    seq::SliceRandom,
    thread_rng,
};

use crate::models::{
    Edge,
    GridNode,
    Map,
};

/// Marks all stations on the map as unsettled, freeing their location for
/// moving in the algorithm.
pub fn unsettle_stations(map: &mut Map) {
    for station in map.get_mut_stations() {
        station.unsettle();
    }
}

/// Randomizes the order of the edges in the given vector.
pub fn randomize_edges(edges: &mut Vec<Edge>) {
    let mut rng = thread_rng();
    edges.shuffle(&mut rng);
}

/// Check if two slices of grid nodes have any overlap.
pub fn have_overlap(left: &[GridNode], right: &[GridNode]) -> bool {
    for node in left {
        if right.contains(node) {
            return true;
        }
    }
    false
}
