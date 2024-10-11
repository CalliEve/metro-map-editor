//! Contains the [`GridNode`] struct and its methods.

use std::{
    fmt::Display,
    ops::{
        Add,
        Mul,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::components::CanvasState;

/// Represents a node on the grid.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridNode(
    /// The x coordinate.
    pub i32,
    /// The y coordinate.
    pub i32,
);

impl GridNode {
    /// Create the [`GridNode`] from the given canvas coordinate and the state
    /// of the canvas.
    pub fn from_canvas_pos(canvas_pos: (f64, f64), state: CanvasState) -> Self {
        Self(
            (canvas_pos.0 / state.drawn_square_size()).round() as i32
                + state
                    .get_offset()
                    .0,
            (canvas_pos.1 / state.drawn_square_size()).round() as i32
                + state
                    .get_offset()
                    .1,
        )
    }

    /// Translate the [`GridNode`] to a canvas coordinate, given the state of
    /// the canvas.
    pub fn to_canvas_pos(self, state: CanvasState) -> (f64, f64) {
        let square_size = state.drawn_square_size();
        (
            f64::from(
                self.0
                    - state
                        .get_offset()
                        .0,
            ) * square_size,
            f64::from(
                self.1
                    - state
                        .get_offset()
                        .1,
            ) * square_size,
        )
    }

    /// Get the diagonal distance to a target node.
    pub fn diagonal_distance_to(self, target: GridNode) -> f64 {
        let dx = (self.0 - target.0).abs();
        let dy = (self.1 - target.1).abs();

        f64::from(dx + dy) - (2f64.sqrt() - 2.0) * f64::from(dx.min(dy))
    }

    /// Get the Manhattan distance to a target node.
    pub fn manhattan_distance_to(self, target: GridNode) -> i32 {
        let dx = (self.0 - target.0).abs();
        let dy = (self.1 - target.1).abs();

        dx + dy
    }

    /// Get a list of all the neighbors of this grid node.
    pub fn get_neighbors(self) -> Vec<GridNode> {
        vec![
            Self(self.0 - 1, self.1 - 1),
            Self(self.0, self.1 - 1),
            Self(self.0 + 1, self.1 - 1),
            Self(self.0 + 1, self.1),
            Self(self.0 + 1, self.1 + 1),
            Self(self.0, self.1 + 1),
            Self(self.0 - 1, self.1 + 1),
            Self(self.0 - 1, self.1),
        ]
    }
}

impl From<(i32, i32)> for GridNode {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl Add for GridNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for GridNode {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<GridNode> for GridNode {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl PartialEq<(i32, i32)> for GridNode {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Display for GridNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_canvas_pos() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        let result = GridNode::from((4, 5)).to_canvas_pos(canvas);

        assert_eq!(result, (20.0, 25.0));
    }

    #[test]
    fn test_from_canvas_pos() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        let result = GridNode::from_canvas_pos((20.0, 24.6), canvas);

        assert_eq!(result, (4, 5));
    }

    #[test]
    fn test_diagonal_distance() {
        let dist = GridNode::from((4, 5)).diagonal_distance_to(GridNode::from((10, 7)));

        assert_eq!(dist.round(), 9.0);
    }

    #[test]
    fn test_get_neighbors() {
        let neighbors = GridNode::from((4, 5)).get_neighbors();

        assert_eq!(
            neighbors,
            vec![
                (3, 4),
                (4, 4),
                (5, 4),
                (5, 5),
                (5, 6),
                (4, 6),
                (3, 6),
                (3, 5)
            ]
        );
    }
}
