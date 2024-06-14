mod graphml;

pub use graphml::decode_map;

pub fn calc_canvas_loc(grid_pos: (i32, i32), square_size: u32) -> (f64, f64) {
    (
        (grid_pos.0 * square_size as i32) as f64,
        (grid_pos.1 * square_size as i32) as f64,
    )
}

pub fn calc_grid_loc(canvas_pos: (f64, f64), square_size: u32) -> (i32, i32) {
    (
        (canvas_pos.0 / (square_size as f64)).round() as i32,
        (canvas_pos.1 / (square_size as f64)).round() as i32,
    )
}
