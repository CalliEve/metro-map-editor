//! This module contains the Recalculate Map algorithm, which is the main
//! function to run the map algorithm.

use leptos::logging;

use super::{
    order_edges::order_edges,
    randomize_edges,
    route_edges::route_edges,
    unsettle_map,
    AlgorithmSettings,
};
use crate::{
    models::Map,
    utils::Result,
    Error,
};

/// Recalculate the map, all the positions of the stations and the edges between
/// them, as a whole. This is the Recalculate Map algorithm in the paper.
pub fn recalculate_map(settings: AlgorithmSettings, map: &mut Map) -> Result<()> {
    if map
        .get_edges()
        .is_empty()
    {
        logging::warn!("Recalculate map called on an empty map");
        return Ok(());
    }

    logging::log!(
        "Recalculating map with {} edges",
        map.get_edges()
            .len()
    );

    map.quickcalc_edges();
    unsettle_map(map);

    let mut edges = order_edges(map)?;
    let mut attempt = 0;
    let mut found = false;

    // logging::log!("Ordered {} edges", edges.len());

    while !found {
        if attempt >= settings.edge_routing_attempts {
            return Err(Error::other(
                "Reached max amount of retries when routing edges.",
            ));
        }

        let mut alg_map = map.clone();

        attempt += 1;
        let res = route_edges(settings, &mut alg_map, edges.clone());

        if let Err(e) = res {
            logging::warn!("Failed to route edges: {e}");
            randomize_edges(&mut edges);
        } else {
            found = true;
            *map = alg_map;
        }
    }

    // TODO: Implement the rest of the algorithm

    logging::log!("Recalculated map");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Map;

    #[test]
    fn test_recalculate_map() {
        let mut map = Map::new();
        recalculate_map(AlgorithmSettings::default(), &mut map).unwrap();
    }
}
