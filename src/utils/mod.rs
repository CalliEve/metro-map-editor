//! Contains utility functions and structs that can be useful
//! everywhere else in the project, but may not fit in other modules.

mod error;
pub mod graphml;
pub mod json;
mod parsing;

pub use error::{
    Error,
    Result,
};

/// Compares two floats to determine if they do not differ more than 1.0.
/// This can be used to see if two coordinates are for the same pixel on the
/// canvas.
pub fn equal_pixel(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0
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
}
