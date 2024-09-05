#[derive(Debug, Clone, Copy)]
pub enum EdgeDirection {
    Up,
    DiagUpRight,
    Right,
    DiagDownRight,
    Down,
    DiagDownLeft,
    Left,
    DiagUpLeft,
    Equal,
}

/// Calculates the direction the edge is moving.
pub fn calc_direction(from_x: f64, from_y: f64, to_x: f64, to_y: f64) -> EdgeDirection {
    if from_x == to_x && from_y > to_y {
        EdgeDirection::Up
    } else if from_x < to_x && from_y > to_y {
        EdgeDirection::DiagUpRight
    } else if from_x < to_x && from_y == to_y {
        EdgeDirection::Right
    } else if from_x < to_x && from_y < to_y {
        EdgeDirection::DiagDownRight
    } else if from_x == to_x && from_y < to_y {
        EdgeDirection::Down
    } else if from_x > to_x && from_y < to_y {
        EdgeDirection::DiagDownLeft
    } else if from_x > to_x && from_y == to_y {
        EdgeDirection::Left
    } else if from_x > to_x && from_y > to_y {
        EdgeDirection::DiagUpLeft
    } else {
        EdgeDirection::Equal
    }
}
