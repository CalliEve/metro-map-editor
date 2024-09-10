use web_sys::CanvasRenderingContext2d;

use crate::{
    algorithm::{
        calc_direction::{
            calc_direction,
            EdgeDirection,
        },
        closest_corner::calc_closest_corner,
    },
    components::CanvasState,
    models::GridNode,
};

pub fn add_offset(offset: f64, x: f64, y: f64, direction: EdgeDirection) -> (f64, f64) {
    match direction {
        EdgeDirection::Up => (x + offset, y),
        EdgeDirection::DiagUpRight => (x + offset, y + offset),
        EdgeDirection::Right => (x, y + offset),
        EdgeDirection::DiagDownRight => (x - offset, y + offset),
        EdgeDirection::Down => (x - offset, y),
        EdgeDirection::DiagDownLeft => (x - offset, y - offset),
        EdgeDirection::Left => (x, y - offset),
        EdgeDirection::DiagUpLeft => (x + offset, y - offset),
        EdgeDirection::Equal => (x, y),
    }
}

pub fn draw_edge(
    from: GridNode,
    to: GridNode,
    steps: &[GridNode],
    canvas: &CanvasRenderingContext2d,
    state: CanvasState,
    height_offset: f64,
) {
    let from_pos = from.to_canvas_pos(state);
    let to_pos = to.to_canvas_pos(state);

    let (from_x, from_y) = calc_closest_corner(
        from_pos,
        steps
            .first()
            .map_or(to_pos, |s| s.to_canvas_pos(state)),
        state,
        height_offset,
    );
    canvas.move_to(from_x, from_y);

    let mut last_is = state.is_on_canvas(from);
    let mut last_pos = (from_x, from_y);
    for step in steps {
        let (mut step_x, mut step_y) = step.to_canvas_pos(state);
        let direction = calc_direction(last_pos.0, last_pos.1, step_x, step_y);
        last_pos = (step_x, step_y);
        (step_x, step_y) = add_offset(height_offset, step_x, step_y, direction);

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
        -height_offset,
    );
    canvas.line_to(to_x, to_y);
}
