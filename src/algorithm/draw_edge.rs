use web_sys::CanvasRenderingContext2d;

use super::closest_corner::calc_closest_corner;
use crate::{
    components::CanvasState,
    models::GridNode,
};

pub fn draw_edge(
    from: GridNode,
    to: GridNode,
    steps: &[GridNode],
    canvas: &CanvasRenderingContext2d,
    state: CanvasState,
) {
    let from_pos = from.to_canvas_pos(state);
    let to_pos = to.to_canvas_pos(state);

    let (from_x, from_y) = calc_closest_corner(
        from_pos,
        steps
            .first()
            .map_or(to_pos, |s| s.to_canvas_pos(state)),
        state,
    );
    canvas.move_to(from_x, from_y);

    let mut last_is = state.is_on_canvas(from);
    for step in steps {
        let (step_x, step_y) = step.to_canvas_pos(state);

        let step_is = state.is_on_canvas(*step);
        if !last_is && !step_is {
            canvas.move_to(step_x, step_y);
            continue;
        }
        last_is = step_is;

        canvas.line_to(step_x, step_y);
    }

    let (to_x, to_y) = calc_closest_corner(
        to_pos,
        steps
            .last()
            .map_or(from_pos, |s| s.to_canvas_pos(state)),
        state,
    );
    canvas.line_to(to_x, to_y);
}
