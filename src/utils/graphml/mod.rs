//! This module provides the capability of decoding valid GraphML data into the
//! [`Map`] struct used in this project.

use quick_xml::de::{
    from_str,
    DeError,
};

mod decode;
mod graphml_map;

use decode::graphml_to_map;
use graphml_map::GraphMlMap;

use crate::models::Map;

/// Decode the given GraphML string into a [`Map`] struct.
/// This decoder also requires the target grid square size to know which station
/// goes onto which grid node.
pub fn decode_map(input: &str, square_size: u32) -> Result<Map, DeError> {
    let decoded: GraphMlMap = from_str(input)?;

    Ok(graphml_to_map(&decoded, square_size))
}
