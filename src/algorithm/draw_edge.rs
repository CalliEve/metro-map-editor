use web_sys::CanvasRenderingContext2d;

use super::closest_corner::calc_closest_corner;
use crate::models::Station;

pub fn draw_edge(
    from: &Station,
    to: &Station,
    canvas: &CanvasRenderingContext2d,
    square_size: u32,
) {
    // TODO: calculate path between stations

    let from_pos = from.get_canvas_pos(square_size);
    let to_pos = to.get_canvas_pos(square_size);

    let (from_x, from_y) = calc_closest_corner(from_pos, to_pos, square_size);
    canvas.move_to(from_x, from_y);

    let (to_x, to_y) = calc_closest_corner(to_pos, from_pos, square_size);
    canvas.line_to(to_x, to_y);
}
