use web_sys::CanvasRenderingContext2d;

use super::closest_corner::calc_closest_corner;
use crate::models::GridNode;

pub fn draw_edge(
    from: GridNode,
    to: GridNode,
    steps: &[GridNode],
    canvas: &CanvasRenderingContext2d,
    square_size: u32,
) {
    let from_pos = from.to_canvas_pos(square_size);
    let to_pos = to.to_canvas_pos(square_size);

    let (from_x, from_y) = calc_closest_corner(
        from_pos,
        steps
            .first()
            .map_or(to_pos, |s| s.to_canvas_pos(square_size)),
        square_size,
    );
    canvas.move_to(from_x, from_y);

    for step in steps {
        let (step_x, step_y) = step.to_canvas_pos(square_size);
        canvas.line_to(step_x, step_y);
    }

    let (to_x, to_y) = calc_closest_corner(
        to_pos,
        steps
            .last()
            .map_or(from_pos, |s| {
                s.to_canvas_pos(square_size)
            }),
        square_size,
    );
    canvas.line_to(to_x, to_y);
}
