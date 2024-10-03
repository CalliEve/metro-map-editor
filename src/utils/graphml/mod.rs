//! This module provides the capability of decoding valid GraphML data into the
//! [`Map`] struct used in this project.

use quick_xml::de::from_str;

mod decode;
mod graphml_map;

use decode::graphml_to_map;
use graphml_map::GraphMlMap;

use super::Result;
use crate::{
    components::CanvasState,
    models::Map,
};

/// Decode the given GraphML string into a [`Map`] struct.
/// This decoder also requires the target grid square size to know which station
/// goes onto which grid node.
pub fn decode_map(input: &str, state: CanvasState) -> Result<Map> {
    let decoded: GraphMlMap = from_str(input)?;

    graphml_to_map(decoded, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_map() {
        let test_file_content = std::fs::read_to_string("existing_maps/small_test.graphml")
            .expect("test data file does not exist");
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let result = decode_map(&test_file_content, canvas).expect("failed to decode graphml");

        let result_line = result
            .get_line(0.into())
            .expect("no line with id 0");
        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");

        let result_station = result
            .get_station(1.into())
            .expect("no station with id 1");
        assert_eq!(result_station.get_pos(), (30, 28));
        assert_eq!(result_station.get_name(), "test 2");
    }
}
