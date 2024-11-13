//! Contains utility functions and structs that can be useful
//! everywhere else in the project, but may not fit in other modules.

mod error;
pub mod graphml;
mod id_manager;
pub mod json;
pub mod line_sections;
mod parsing;

#[cfg(feature = "heatmap")]
pub mod heatmap_data;

pub use error::{
    Error,
    Result,
};
pub use id_manager::{
    IDData,
    IDManager,
};

use crate::models::GridNode;

/// Compares two floats to determine if they do not differ more than 1.0.
/// This can be used to see if two coordinates are for the same pixel on the
/// canvas.
pub fn equal_pixel(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0
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

/// Calculates the offset of the grid node from the canvas offset.
pub fn canvas_offset_to_grid_offset(offset: (f64, f64), square_size: f64) -> (i32, i32) {
    (
        (offset.0 / square_size).round() as i32,
        (offset.1 / square_size).round() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_pixel() {
        assert!(equal_pixel(3.253, 3.0));
        assert!(equal_pixel(3.0, 3.253));
        assert!(equal_pixel(3.0, 3.0));
        assert!(!equal_pixel(4.0, 3.0));
        assert!(!equal_pixel(4.7, 3.6));
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
        let first = GridNode::from((-1, -1));
        let second = GridNode::from((1, 1));
        let third = GridNode::from((2, 0));
        assert_eq!(
            calculate_angle(first, second, third),
            90.0
        );
    }
}
