//! This module provides the capability of decoding JSON data into the
//! [`Map`] struct used in this project and vice versa.

use serde_json::from_str;

mod decode;
mod encode;
mod json_models;

use decode::json_to_map;
use encode::map_to_json;
use json_models::JSONMap;

use super::Result;
use crate::{
    components::CanvasState,
    models::Map,
};

/// Decode the given JSON string into a [`Map`] struct.
/// This decoder also requires the target grid square size to know which station
/// goes onto which grid node.
pub fn decode_map(input: &str, state: CanvasState) -> Result<Map> {
    let decoded: JSONMap = from_str(input)?;

    json_to_map(decoded, state)
}

/// Encode the given [`Map`] into a JSON string.
pub fn encode_map(map: &Map, state: CanvasState) -> Result<String> {
    let json_map = map_to_json(map, state);

    serde_json::to_string(&json_map).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_map() {
        let test_file_content = std::fs::read_to_string("exisiting_maps/small_test.json")
            .expect("test data file does not exist");
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let result = decode_map(&test_file_content, canvas).expect("failed to decode json");

        let result_line = result
            .get_line(0.into())
            .expect("no line with id 0");
        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");

        let result_station = result
            .get_station(1.into())
            .expect("no station with id 1");
        assert_eq!(result_station.get_pos(), (30, 58));
        assert_eq!(result_station.get_name(), "test-2");
    }

    #[test]
    fn test_encode_map() {
        let test_file_content = std::fs::read_to_string("exisiting_maps/small_test.json")
            .expect("test data file does not exist");
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((100, 100));

        let map = decode_map(&test_file_content, canvas).expect("failed to decode json");
        let result = encode_map(&map, canvas).expect("failed to encode map");

        assert_eq!(
            result,
            test_file_content
                .replace("\n", "")
                .replace(" ", "")
        );
    }
}
