//! Parsing utilities for the application.

use std::hash::{
    DefaultHasher,
    Hash,
    Hasher,
};

use super::Result;
use crate::components::CanvasState;

/// Settings used to normalize coordinates.
#[derive(Debug, Clone, Copy)]
pub(super) struct NormalizationSettings {
    /// The minimum x coordinate.
    pub(super) min_x: f64,
    /// The maximum x coordinate.
    pub(super) max_x: f64,
    /// The minimum y coordinate.
    pub(super) min_y: f64,
    /// The maximum y coordinate.
    pub(super) max_y: f64,
    /// The width of the canvas in the x-coordinate.
    pub(super) size_x: f64,
    /// The height of the canvas in the y-coordinate.
    pub(super) size_y: f64,
    /// The offset of the canvas.
    pub(super) offset: (f64, f64),
    /// The size of a square on the canvas.
    pub(super) square_size: f64,
}

/// Saved data sometimes has maps/stations located in weird places (like all x
/// coordinates being negative or only difference being in the decimals), this
/// normalizes them so they fit within the canvas as it currently is. Returns
/// the generated [`NormalizationSettings`] and the normalized coordinates.
pub(super) fn normalize_coords(
    mut items: Vec<(f64, f64)>,
    state: CanvasState,
) -> (Vec<(f64, f64)>, NormalizationSettings) {
    let square_size = state.drawn_square_size();

    let size_x = f64::from(
        state
            .get_visible_size()
            .0,
    ) * square_size
        - 4.0 * square_size;
    let size_y = f64::from(
        state
            .get_visible_size()
            .1,
    ) * square_size
        - 4.0 * square_size;

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

    let settings = NormalizationSettings {
        min_x,
        max_x,
        min_y,
        max_y,
        size_x,
        size_y,
        square_size,
        offset: (
            f64::from(
                state
                    .get_offset()
                    .0,
            ),
            f64::from(
                state
                    .get_offset()
                    .1,
            ),
        ),
    };

    for (x, y) in &mut items {
        (*x, *y) = normalize_coordinate(*x, *y, settings);
    }

    (items, settings)
}

/// Normalize the given coordinate using the given settings.
pub(super) fn normalize_coordinate(x: f64, y: f64, settings: NormalizationSettings) -> (f64, f64) {
    let x = (x - settings.min_x) / (settings.max_x - settings.min_x) * settings.size_x
        + (settings
            .offset
            .0
            / settings.square_size)
        + 2.0 * settings.square_size;
    let y = (y - settings.min_y) / (settings.max_y - settings.min_y) * settings.size_y
        + (settings
            .offset
            .1
            / settings.square_size)
        + 2.0 * settings.square_size;
    (x, y)
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

/// Parses the given string into a rgb color.
pub fn parse_color(given: &str) -> Result<(u8, u8, u8)> {
    let color = csscolorparser::parse(given)?.to_rgba8();

    Ok((color[0], color[1], color[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_coords() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((100.0, 100.0));

        let result = normalize_coords(
            vec![(-1.0, -3.0), (1.0, 1.0), (3.0, 5.0)],
            canvas,
        );

        assert_eq!(
            result.0,
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

    #[test]
    fn test_parse_color() {
        assert_eq!(
            parse_color("rgb(255, 0, 0)").unwrap(),
            (255, 0, 0)
        );
        assert_eq!(
            parse_color("rgb(0, 40, 0)").unwrap(),
            (0, 40, 0)
        );
        assert_eq!(
            parse_color("rgb(0, 0, 255)").unwrap(),
            (0, 0, 255)
        );
        assert_eq!(
            parse_color("rgb(255, 255, 255)").unwrap(),
            (255, 255, 255)
        );
        assert_eq!(
            parse_color("rgb(0, 0, 0)").unwrap(),
            (0, 0, 0)
        );
        assert_eq!(
            parse_color("#ff0000").unwrap(),
            (255, 0, 0)
        );
        assert_eq!(
            parse_color("#00ff00").unwrap(),
            (0, 255, 0)
        );
        assert_eq!(
            parse_color("#0000FF").unwrap(),
            (0, 0, 255)
        );
    }
}
