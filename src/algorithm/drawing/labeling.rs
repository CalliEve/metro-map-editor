//! Contains functions to help with placing labels on the canvas.

use crate::{
    algorithm::drawing::calc_direction::{
        calc_direction,
        EdgeDirection,
    },
    CanvasState,
};

/// Calculate the position of the label based on the given coordinates of the
/// node that should be labeled. In addition the next node can be given to
/// determine the direction of the label and make sure it doesn't cross it. Also
/// can be provided an offset to move the label further away from the node in
/// case of multiple lines. Returns a vector of possible coordinates where the
/// label can be placed in order of preference.
pub fn calc_label_pos(
    state: CanvasState,
    coord: (f64, f64),
    next_coord: Option<(f64, f64)>,
    offset: Option<f64>,
) -> Vec<(f64, f64)> {
    let offset = (state.drawn_square_size() / 4.0) + offset.unwrap_or(0.0);

    if next_coord.is_none() {
        return vec![
            (coord.0 + offset, coord.1 - offset),
            (coord.0 + offset, coord.1 + offset),
            (coord.0 - offset, coord.1 - offset),
            (coord.0 - offset, coord.1 + offset),
        ];
    }

    let next_coord = next_coord.unwrap();
    match calc_direction(
        coord.0,
        coord.1,
        next_coord.0,
        next_coord.1,
    ) {
        EdgeDirection::Equal => {
            vec![
                (coord.0 + offset, coord.1 - offset),
                (coord.0 + offset, coord.1 + offset),
                (coord.0 - offset, coord.1 - offset),
                (coord.0 - offset, coord.1 + offset),
            ]
        },
        EdgeDirection::Up => {
            vec![
                (coord.0 - offset, coord.1),
                (coord.0 + offset, coord.1),
                (coord.0 - offset, coord.1 - offset),
                (coord.0 + offset, coord.1 - offset),
            ]
        },
        EdgeDirection::DiagUpRight => {
            vec![
                (coord.0 + offset, coord.1),
                (coord.0 - offset, coord.1 - offset),
                (coord.0 - offset, coord.1),
            ]
        },
        EdgeDirection::Right => {
            vec![
                (coord.0, coord.1 - offset),
                (coord.0, coord.1 + offset),
                (coord.0 + offset, coord.1 - offset),
                (coord.0 + offset, coord.1 + offset),
            ]
        },
        EdgeDirection::DiagDownRight => {
            vec![
                (coord.0, coord.1 - offset),
                (coord.0 + offset, coord.1),
                (coord.0, coord.1 + offset),
            ]
        },
        EdgeDirection::Down => {
            vec![
                (coord.0 + offset, coord.1),
                (coord.0 - offset, coord.1),
                (coord.0 + offset, coord.1 + offset),
                (coord.0 - offset, coord.1 + offset),
            ]
        },
        EdgeDirection::DiagDownLeft => {
            vec![
                (coord.0 + offset, coord.1),
                (coord.0, coord.1 + offset),
                (coord.0 - offset, coord.1),
            ]
        },
        EdgeDirection::Left => {
            vec![
                (coord.0, coord.1 - offset),
                (coord.0, coord.1 + offset),
                (coord.0 - offset, coord.1 - offset),
                (coord.0 - offset, coord.1 + offset),
            ]
        },
        EdgeDirection::DiagUpLeft => {
            vec![
                (coord.0, coord.1 - offset),
                (coord.0 - offset, coord.1),
                (coord.0, coord.1 + offset),
            ]
        },
    }
}
