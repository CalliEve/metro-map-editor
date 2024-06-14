use leptos::logging::log;
use quick_xml::de::{from_str, DeError};

mod decode;
mod graphml_map;

use crate::algorithm::Map;
use decode::graphml_to_map;
use graphml_map::GraphMlMap;

pub fn decode_map(input: String, square_size: u32) -> Result<Map, DeError> {
    let decoded: GraphMlMap = from_str(&input)?;

    Ok(graphml_to_map(decoded, square_size))
}
