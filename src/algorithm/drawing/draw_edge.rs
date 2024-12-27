//! Contains the function for drawing an edge onto the map.

use super::{
    calc_closest_corner,
    calc_direction::{
        calc_direction,
        EdgeDirection,
    },
    canvas_context::CanvasContext,
};
use crate::{
    components::CanvasState,
    models::GridNode,
};

/// Adds an offset to the given coordinates based on the given direction of the
/// edge.
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

/// Draws an edge between two nodes with, optionally, the given step nodes in
/// between. An offset can be given to draw the edge higher or lower.
pub fn draw_edge(
    from: GridNode,
    to: GridNode,
    steps: &[GridNode],
    canvas: &CanvasContext<'_>,
    state: CanvasState,
    height_offset: f64,
) {
    let mut steps = steps;
    let from_pos = from.to_canvas_pos(state);
    let to_pos = to.to_canvas_pos(state);
    let has_offset = height_offset.abs() > f64::EPSILON;

    #[allow(unused_assignments)] // It is used to keep the borrow going
    let mut steps_vec = Vec::new();
    if let Some(start) = steps.first() {
        if !from.is_neighbor_of(start) {
            steps_vec = steps
                .iter()
                .rev()
                .copied()
                .collect::<Vec<GridNode>>();
            steps = steps_vec.as_ref();
        }
    }

    // The position of the start node on the canvas, based on the direction it is
    // leaving the station from
    let (from_x, from_y) = calc_closest_corner(
        from_pos,
        steps
            .first()
            .map_or(to_pos, |s| s.to_canvas_pos(state)),
        state,
        height_offset,
    );
    canvas.move_to(from_x, from_y);

    // The position of the last node on the canvas and if it is on the canvas
    let mut last_pos = (from_x, from_y);
    let mut last_is = state.is_on_canvas(from);

    for step in steps {
        let (mut step_x, mut step_y) = step.to_canvas_pos(state);

        if has_offset {
            // Add a potential offset to the step based on the direction the edge is going
            let direction = calc_direction(last_pos.0, last_pos.1, step_x, step_y);
            last_pos = (step_x, step_y);
            (step_x, step_y) = add_offset(height_offset, step_x, step_y, direction);
        }

        // If the last step was off the canvas and the current step is off the canvas,
        // then don't draw this edge
        let step_is = state.is_on_canvas(*step);
        if !last_is && !step_is {
            canvas.move_to(step_x, step_y);
            continue;
        }
        last_is = step_is;

        canvas.line_to(step_x, step_y);
    }

    // The position of the target node on the canvas, based on the direction it is
    // coming into the station from
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_edge_diag() {
        let canvas = CanvasContext::new();
        let mut state = CanvasState::new();
        state.set_square_size(5);
        state.set_size((100.0, 100.0));

        let from = GridNode::from((0, 0));
        let to = GridNode::from((3, 3));
        let steps = vec![
            GridNode::from((1, 1)),
            GridNode::from((2, 2)),
        ];

        draw_edge(from, to, &steps, &canvas, state, 0.0);

        assert_eq!(
            canvas.get_record("move_to"),
            Some(vec!["1.3,1.3".to_owned(),])
        );

        assert_eq!(
            canvas.get_record("line_to"),
            Some(vec![
                "5.0,5.0".to_owned(),
                "10.0,10.0".to_owned(),
                "13.7,13.7".to_owned()
            ])
        );
    }

    #[test]
    fn test_draw_edge_wiggly() {
        let canvas = CanvasContext::new();
        let mut state = CanvasState::new();
        state.set_square_size(5);
        state.set_size((100.0, 100.0));

        let from = GridNode::from((0, 0));
        let to = GridNode::from((3, 3));
        let steps = vec![
            GridNode::from((1, 1)),
            GridNode::from((1, 2)),
            GridNode::from((2, 2)),
            GridNode::from((2, 3)),
            GridNode::from((2, 4)),
            GridNode::from((3, 4)),
        ];

        draw_edge(from, to, &steps, &canvas, state, 0.0);

        assert_eq!(
            canvas.get_record("move_to"),
            Some(vec!["1.3,1.3".to_owned(),])
        );

        assert_eq!(
            canvas.get_record("line_to"),
            Some(vec![
                "5.0,5.0".to_owned(),
                "5.0,10.0".to_owned(),
                "10.0,10.0".to_owned(),
                "10.0,15.0".to_owned(),
                "10.0,20.0".to_owned(),
                "15.0,20.0".to_owned(),
                "15.0,16.6".to_owned()
            ])
        );
    }

    #[test]
    fn test_draw_edge_on_off_canvas() {
        let canvas = CanvasContext::new();
        let mut state = CanvasState::new();
        state.set_square_size(5);
        state.set_size((16.0, 16.0));

        let from = GridNode::from((0, 0));
        let to = GridNode::from((3, 3));
        let steps = vec![
            GridNode::from((1, 1)),
            GridNode::from((1, 2)),
            GridNode::from((2, 2)),
            GridNode::from((2, 3)),
            GridNode::from((2, 4)),
            GridNode::from((3, 4)),
            GridNode::from((4, 4)),
            GridNode::from((4, 5)),
            GridNode::from((3, 4)),
        ];

        draw_edge(from, to, &steps, &canvas, state, 0.0);

        assert_eq!(
            canvas.get_record("move_to"),
            Some(vec![
                "1.3,1.3".to_owned(),
                "15.0,20.0".to_owned(),
                "20.0,20.0".to_owned(),
                "20.0,25.0".to_owned(),
                "15.0,20.0".to_owned(),
            ])
        );

        assert_eq!(
            canvas.get_record("line_to"),
            Some(vec![
                "5.0,5.0".to_owned(),
                "5.0,10.0".to_owned(),
                "10.0,10.0".to_owned(),
                "10.0,15.0".to_owned(),
                "10.0,20.0".to_owned(),
                "15.0,16.6".to_owned()
            ])
        );
    }

    #[test]
    fn test_draw_edge_with_offset() {
        let canvas = CanvasContext::new();
        let mut state = CanvasState::new();
        state.set_square_size(5);
        state.set_size((100.0, 100.0));

        let from = GridNode::from((0, 0));
        let to = GridNode::from((3, 3));
        let steps = vec![
            GridNode::from((1, 1)),
            GridNode::from((2, 2)),
        ];

        draw_edge(from, to, &steps, &canvas, state, 1.0);

        assert_eq!(
            canvas.get_record("move_to"),
            Some(vec!["0.3,2.3".to_owned(),])
        );

        assert_eq!(
            canvas.get_record("line_to"),
            Some(vec![
                "4.0,6.0".to_owned(),
                "9.0,11.0".to_owned(),
                "12.7,14.7".to_owned()
            ])
        );
    }
}
