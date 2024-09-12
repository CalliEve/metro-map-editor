use std::f64::consts::PI;

use crate::{
    components::CanvasState,
    utils::equal_pixel,
};

/// Calculates the coordinate of the corner (on an octilinear grid) of a station
/// closest to the given neigbor. An offset is provided for, if the corner is
/// further from the middle of the station coordinate.
pub fn calc_closest_corner(
    from: (f64, f64),
    to: (f64, f64),
    state: CanvasState,
    height_offset: f64,
) -> (f64, f64) {
    let cardinal_offset = state.drawn_square_size() / PI;
    let corner_offset = state.drawn_square_size() / PI * 0.8;

    let (from_x, from_y) = from;
    let (to_x, to_y) = to;

    if equal_pixel(from_x, to_x) {
        if from_y > to_y {
            (
                from_x + height_offset,
                from_y - cardinal_offset,
            ) // below
        } else {
            (
                from_x - height_offset,
                from_y + cardinal_offset,
            ) // above
        }
    } else if from_x > to_x {
        if equal_pixel(from_y, to_y) {
            (
                from_x - cardinal_offset,
                from_y - height_offset,
            ) // left
        } else if from_y > to_y {
            (
                from_x - corner_offset + height_offset,
                from_y - corner_offset - height_offset,
            ) // below left
        } else {
            (
                from_x - corner_offset - height_offset,
                from_y + corner_offset - height_offset,
            ) // above left
        }
    } else if equal_pixel(from_y, to_y) {
        (
            from_x + cardinal_offset,
            from_y + height_offset,
        ) // right
    } else if from_y > to_y {
        (
            from_x + corner_offset + height_offset,
            from_y - corner_offset + height_offset,
        ) // below right
    } else {
        (
            from_x + corner_offset - height_offset,
            from_y + corner_offset + height_offset,
        ) // above right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_closest_corner_test(from: (f64, f64), to: (f64, f64), expected: (f64, f64)) {
        let mut state = CanvasState::new();
        state.set_square_size(3);

        let result = calc_closest_corner(from, to, state, 0.0);
        let (result_x, result_y) = result;
        let (expected_x, expected_y) = expected;

        assert!(
            equal_pixel(result_x, expected_x),
            "expected {expected:?} got ({}, {})",
            result_x.round(),
            result_y.round()
        );
        assert!(
            equal_pixel(result_y, expected_y),
            "expected {expected:?} got ({}, {})",
            result_x.round(),
            result_y.round()
        );
    }

    #[test]
    fn test_calc_closest_corner() {
        // Bottom right
        run_closest_corner_test((15.0, 15.0), (20.0, 20.0), (16.0, 16.0));
        run_closest_corner_test((15.0, 15.0), (16.0, 20.0), (16.0, 16.0));
        run_closest_corner_test((15.0, 15.0), (20.0, 16.0), (16.0, 16.0));

        // Top right
        run_closest_corner_test((15.0, 15.0), (20.0, 10.0), (16.0, 14.0));
        run_closest_corner_test((15.0, 15.0), (16.0, 10.0), (16.0, 14.0));
        run_closest_corner_test((15.0, 15.0), (20.0, 14.0), (16.0, 14.0));

        // Top left
        run_closest_corner_test((15.0, 15.0), (10.0, 10.0), (14.0, 14.0));
        run_closest_corner_test((15.0, 15.0), (14.0, 10.0), (14.0, 14.0));
        run_closest_corner_test((15.0, 15.0), (10.0, 14.0), (14.0, 14.0));

        // Bottom left
        run_closest_corner_test((15.0, 15.0), (10.0, 20.0), (14.0, 16.0));
        run_closest_corner_test((15.0, 15.0), (14.0, 20.0), (14.0, 16.0));
        run_closest_corner_test((15.0, 15.0), (10.0, 16.0), (14.0, 16.0));

        // same x-axis
        run_closest_corner_test((15.0, 15.0), (15.0, 20.0), (15.0, 16.0));
        run_closest_corner_test((15.0, 15.0), (15.0, 10.0), (15.0, 14.0));

        // same y-axis
        run_closest_corner_test((15.0, 15.0), (20.0, 15.0), (16.0, 15.0));
        run_closest_corner_test((15.0, 15.0), (10.0, 15.0), (14.0, 15.0));
    }
}
