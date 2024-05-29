pub fn calc_canvas_loc(grid_pos: (u32, u32), square_size: u32) -> (f64, f64) {
    (
        (grid_pos.0 * square_size) as f64,
        (grid_pos.1 * square_size) as f64,
    )
}
