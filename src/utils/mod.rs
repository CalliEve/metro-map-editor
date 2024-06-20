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
