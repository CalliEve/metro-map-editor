//! Contains utility functions and structs that can be useful
//! everywhere else in the project, but may not fit in other modules.

mod graphml;

pub use graphml::decode_map;

/// Calculate the location on the canvas based on the given grid location and
/// grid square size.
pub fn calc_canvas_loc(grid_pos: (i32, i32), square_size: u32) -> (f64, f64) {
    (
        f64::from(grid_pos.0 * square_size as i32),
        f64::from(grid_pos.1 * square_size as i32),
    )
}

/// Calculate the location on the grid based on the given canvas location and
/// grid square size.
pub fn calc_grid_loc(canvas_pos: (f64, f64), square_size: u32) -> (i32, i32) {
    (
        (canvas_pos.0 / f64::from(square_size)).round() as i32,
        (canvas_pos.1 / f64::from(square_size)).round() as i32,
    )
}

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
    fn test_calc_canvas_loc() {
        let result = calc_canvas_loc((4, 5), 30);

        assert_eq!(result, (120.0, 150.0));
    }

    #[test]
    fn test_calc_grid_loc() {
        let result = calc_grid_loc((120.0, 157.5), 30);

        assert_eq!(result, (4, 5));
    }

    #[test]
    fn test_equal_pixel() {
        assert!(equal_pixel(3.253, 3.0));
        assert!(equal_pixel(3.0, 3.253));
        assert!(equal_pixel(3.0, 3.0));
        assert!(!equal_pixel(4.0, 3.0));
        assert!(!equal_pixel(4.7, 3.6));
    }
}
