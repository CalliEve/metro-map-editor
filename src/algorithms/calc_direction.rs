//! Contains tools to determine the direction of an edge.

use crate::{
    models::GridNode,
    utils::equal_pixel,
};

/// Represents the direction the edge is moving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    /// The edge is moving up.
    Up,
    /// The edge is moving diagonally up and to the right.
    DiagUpRight,
    /// The edge is moving right.
    Right,
    /// The edge is moving diagonally down and to the right.
    DiagDownRight,
    /// The edge is moving down.
    Down,
    /// The edge is moving diagonally down and to the left.
    DiagDownLeft,
    /// The edge is moving left.
    Left,
    /// The edge is moving diagonally up and to the left.
    DiagUpLeft,
    /// The edge is not moving.
    Equal,
}

impl EdgeDirection {
    /// Returns the opposite direction.
    pub fn flip(self) -> EdgeDirection {
        match self {
            EdgeDirection::Up => EdgeDirection::Down,
            EdgeDirection::DiagUpRight => EdgeDirection::DiagDownLeft,
            EdgeDirection::Right => EdgeDirection::Left,
            EdgeDirection::DiagDownRight => EdgeDirection::DiagUpLeft,
            EdgeDirection::Down => EdgeDirection::Up,
            EdgeDirection::DiagDownLeft => EdgeDirection::DiagUpRight,
            EdgeDirection::Left => EdgeDirection::Right,
            EdgeDirection::DiagUpLeft => EdgeDirection::DiagDownRight,
            EdgeDirection::Equal => EdgeDirection::Equal,
        }
    }
}

/// Calculates the direction the edge is moving.
pub fn calc_direction(from_x: f64, from_y: f64, to_x: f64, to_y: f64) -> EdgeDirection {
    if equal_pixel(from_x, to_x) && from_y > to_y {
        EdgeDirection::Up
    } else if from_x < to_x && from_y > to_y {
        EdgeDirection::DiagUpRight
    } else if from_x < to_x && equal_pixel(from_y, to_y) {
        EdgeDirection::Right
    } else if from_x < to_x && from_y < to_y {
        EdgeDirection::DiagDownRight
    } else if equal_pixel(from_x, to_x) && from_y < to_y {
        EdgeDirection::Down
    } else if from_x > to_x && from_y < to_y {
        EdgeDirection::DiagDownLeft
    } else if from_x > to_x && equal_pixel(from_y, to_y) {
        EdgeDirection::Left
    } else if from_x > to_x && from_y > to_y {
        EdgeDirection::DiagUpLeft
    } else {
        EdgeDirection::Equal
    }
}

/// Calculates the direction the edge is moving based on the start and end node.
pub fn node_direction(start: GridNode, end: GridNode) -> EdgeDirection {
    to_direction(end.0 - start.0, end.1 - start.1)
}

/// Converts the given horizontal and vertical difference values to a direction.
fn to_direction(horizontal: i32, vertical: i32) -> EdgeDirection {
    match (horizontal, vertical) {
        (0, ..=-1) => EdgeDirection::Up,
        (1.., ..=-1) => EdgeDirection::DiagUpRight,
        (1.., 0) => EdgeDirection::Right,
        (1.., 1..) => EdgeDirection::DiagDownRight,
        (0, 1..) => EdgeDirection::Down,
        (..=-1, 1..) => EdgeDirection::DiagDownLeft,
        (..=-1, 0) => EdgeDirection::Left,
        (..=-1, ..=-1) => EdgeDirection::DiagUpLeft,
        (0, 0) => EdgeDirection::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_direction() {
        assert_eq!(
            calc_direction(0.0, 0.0, 0.0, -1.0),
            EdgeDirection::Up
        );
        assert_eq!(
            calc_direction(0.0, 0.0, 1.0, -1.0),
            EdgeDirection::DiagUpRight
        );
        assert_eq!(
            calc_direction(0.0, 0.0, 1.0, 0.0),
            EdgeDirection::Right
        );
        assert_eq!(
            calc_direction(0.0, 0.0, 1.0, 1.0),
            EdgeDirection::DiagDownRight
        );
        assert_eq!(
            calc_direction(0.0, 0.0, 0.0, 1.0),
            EdgeDirection::Down
        );
        assert_eq!(
            calc_direction(0.0, 0.0, -1.0, 1.0),
            EdgeDirection::DiagDownLeft
        );
        assert_eq!(
            calc_direction(0.0, 0.0, -1.0, 0.0),
            EdgeDirection::Left
        );
        assert_eq!(
            calc_direction(0.0, 0.0, -1.0, -1.0),
            EdgeDirection::DiagUpLeft
        );
        assert_eq!(
            calc_direction(0.0, 0.0, 0.0, 0.0),
            EdgeDirection::Equal
        );
    }
}
