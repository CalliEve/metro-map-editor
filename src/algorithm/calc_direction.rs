//! Contains tools to determine the direction of an edge.

use crate::utils::equal_pixel;

/// Represents the direction the edge is moving.
#[derive(Debug, Clone, Copy, PartialEq)]
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
