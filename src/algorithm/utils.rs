//! Utility functions specifically for the algorithm module.

use leptos::logging;
use rand::{
    seq::SliceRandom,
    thread_rng,
};

use super::AlgorithmSettings;
use crate::models::{
    Edge,
    GridNode,
    Map,
};

/// Marks all stations and edges on the map as unsettled, freeing their location
/// for moving in the algorithm.
pub fn unsettle_map(map: &mut Map) {
    for station in map.get_mut_stations() {
        station.unsettle();
    }
    for edge in map.get_mut_edges() {
        edge.unsettle();
    }
}

/// Randomizes the order of the edges in the given vector.
pub fn randomize_edges(edges: &mut Vec<Edge>) {
    let mut rng = thread_rng();
    edges.shuffle(&mut rng);
}

/// Returns true if the given node is outside the grid limits.
pub fn node_outside_grid(settings: AlgorithmSettings, node: GridNode) -> bool {
    node.0
        < settings
            .grid_x_limits
            .0
        || node.0
            > settings
                .grid_x_limits
                .1
        || node.1
            < settings
                .grid_y_limits
                .0
        || node.1
            > settings
                .grid_y_limits
                .1
}

/// Returns the amount of overlap between two slices.
pub fn overlap_amount<T: PartialEq>(left: &[T], right: &[T]) -> usize {
    left.iter()
        .filter(|&l| right.contains(l))
        .count()
}

/// Calculates the angle formed by three grid nodes and returns it in rounded
/// degrees. The second point is assumed to be the middle node where the angle
/// is located.
pub fn calculate_angle(first: GridNode, second: GridNode, third: GridNode) -> f64 {
    let l = (f64::from(first.1 - second.1)).atan2(f64::from(first.0 - second.0));
    let r = (f64::from(third.1 - second.1)).atan2(f64::from(third.0 - second.0));
    (l - r)
        .abs()
        .to_degrees()
        .round()
}

/// Prints a debug message if the settings allow it.
pub fn debug_print(settings: AlgorithmSettings, msg: &str, warn: bool) {
    if settings.debug && warn {
        logging::debug_warn!("{}", msg);
    } else if settings.debug {
        logging::log!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_outside_grid() {
        let mut settings = AlgorithmSettings::default();
        settings.grid_x_limits = (0, 10);
        settings.grid_y_limits = (0, 10);

        assert!(node_outside_grid(
            settings,
            GridNode::from((-1, 0))
        ));
        assert!(node_outside_grid(
            settings,
            GridNode::from((0, -1))
        ));
        assert!(node_outside_grid(
            settings,
            GridNode::from((11, 0))
        ));
        assert!(node_outside_grid(
            settings,
            GridNode::from((0, 11))
        ));
        assert!(!node_outside_grid(
            settings,
            GridNode::from((0, 0))
        ));
        assert!(!node_outside_grid(
            settings,
            GridNode::from((10, 10))
        ));
    }

    #[test]
    fn test_overlap_amount() {
        let left = vec![1, 2, 3, 0, 4, 5, 2];
        let right = vec![8, 3, 4, 10, 5, 6, 7];
        assert_eq!(overlap_amount(&left, &right), 3);
    }

    #[test]
    fn test_calculate_angle() {
        let first = GridNode::from((0, 0));
        let second = GridNode::from((1, 1));
        let third = GridNode::from((2, 0));
        assert_eq!(
            calculate_angle(first, second, third),
            90.0
        );
    }
}
