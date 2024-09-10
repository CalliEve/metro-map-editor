//! Parsing utilities for the application.

use std::hash::{
    DefaultHasher,
    Hash,
    Hasher,
};

use crate::components::CanvasState;

/// Saved data sometimes has maps/stations located in weird places (like all x
/// coordinates being negative or only difference being in the decimals), this
/// normalizes them so they fit within the canvas as it currently is.
pub(super) fn normalize_coords(mut items: Vec<(f64, f64)>, state: CanvasState) -> Vec<(f64, f64)> {
    let square_size = state.drawn_square_size();

    let size_x = f64::from(
        state
            .get_size()
            .1,
    ) - 4.0 * square_size;
    let size_y = f64::from(
        state
            .get_size()
            .0,
    ) - 4.0 * square_size;

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for (x, y) in items
        .iter()
        .copied()
    {
        if min_x > x {
            min_x = x;
        }
        if max_x < x {
            max_x = x;
        }
        if min_y > y {
            min_y = y;
        }
        if max_y < y {
            max_y = y;
        }
    }

    for (x, y) in &mut items {
        *x = (*x - min_x) / (max_x - min_x) * size_x + 2.0 * square_size;
        *y = (*y - min_y) / (max_y - min_y) * size_y + 2.0 * square_size;
    }

    items
}

/// Parse the given string into an u64 to create an ID from.
pub(super) fn parse_id(given: &str) -> u64 {
    given
        .parse()
        .ok()
        .or_else(|| {
            given
                .get(1..)
                .and_then(|i| {
                    i.parse()
                        .ok()
                })
        })
        .unwrap_or_else(|| {
            let mut hasher = DefaultHasher::new();
            given.hash(&mut hasher);
            hasher.finish()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_coords() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((100, 100));

        let result = normalize_coords(
            vec![(-1.0, -3.0), (1.0, 1.0), (3.0, 5.0)],
            canvas,
        );

        assert_eq!(
            result,
            vec![(10.0, 10.0), (50.0, 50.0), (90.0, 90.0)]
        );
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id("test"), 14402189752926126668);
        assert_eq!(parse_id("1"), 1);
        assert_eq!(parse_id("a"), 8186225505942432243);
        assert_eq!(parse_id("a1"), 1);
        assert_eq!(
            parse_id("test") as f64,
            14402189752926126668.0
        );
    }
}