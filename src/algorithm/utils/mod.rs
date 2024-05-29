pub fn calc_canvas_loc(grid_pos: (i32, i32), square_size: u32) -> (f64, f64) {
    (
        (grid_pos.0 * square_size as i32) as f64,
        (grid_pos.1 * square_size as i32) as f64,
    )
}
