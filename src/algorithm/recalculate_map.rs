//! This module contains the Recalculate Map algorithm, which is the main
//! function to run the map algorithm.

use std::collections::HashMap;

use leptos::logging;

use super::{
    local_search::local_search,
    order_edges::order_edges,
    randomize_edges,
    route_edges::route_edges,
    station_contraction::{
        contract_stations,
        expand_stations,
    },
    unsettle_map,
    AlgorithmSettings,
};
use crate::{
    algorithm::debug_print,
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

    debug_print(
        settings,
        &format!(
            "Recalculating map with {} edges and {} stations",
            map.get_edges()
                .len(),
            map.get_stations()
                .len()
        ),
        false,
    );

    let contracted_stations = contract_stations(settings, map);

    debug_print(
        settings,
        &format!(
            "Contracted stations, {} edges and {} stations left",
            map.get_edges()
                .len(),
            map.get_stations()
                .len()
        ),
        false,
    );

    map.quickcalc_edges();
    unsettle_map(map);

    let mut edges = order_edges(map)?;
    let mut attempt = 0;
    let mut found = false;
    let mut occupied = HashMap::new();

    debug_print(
        settings,
        &format!("Ordered {} edges", edges.len()),
        false,
    );

    while !found {
        let mut alg_map = map.clone();

        attempt += 1;
        let res = route_edges(settings, &mut alg_map, edges.clone());

        if let Err(e) = res {
            logging::error!("Failed to route edges: {e}");

            if attempt >= settings.edge_routing_attempts {
                *map = alg_map;
                return Err(Error::other(
                    "Reached max amount of retries when routing edges.",
                ));
            }

            randomize_edges(&mut edges);
        } else {
            found = true;
            *map = alg_map;
            occupied = res.unwrap();
        }
    }

    debug_print(
        settings,
        "Routed edges, commencing local search",
        false,
    );

    // local_search(settings, map, &mut occupied);

    debug_print(
        settings,
        "Finished local search, re-adding contracted stations",
        false,
    );

    expand_stations(settings, map, &contracted_stations)?;

    logging::log!("Recalculated map");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        utils::{
            graphml,
            json,
        },
        CanvasState,
        MapState,
    };

    #[test]
    fn test_recalculate_map() {
        let map_files = vec![
            "existing_maps/disjointed_test.json",
            "existing_maps/routing_test.json",
            "existing_maps/montreal.graphml",
            "existing_maps/wien.graphml",
            "existing_maps/washington.graphml",
            "existing_maps/karlsruhe.graphml",
            "existing_maps/sydney.graphml",
            "existing_maps/berlin.graphml",
        ];

        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((800, 1700));

        for map_file in &map_files {
            let test_file_content = std::fs::read_to_string(map_file).expect(&format!(
                "test data file {map_file} does not exist"
            ));

            let mut map = if map_file.ends_with(".json") {
                json::decode_map(&test_file_content, canvas).expect(&format!(
                    "failed to decode json of {map_file}"
                ))
            } else {
                graphml::decode_map(&test_file_content, canvas).expect(&format!(
                    "failed to decode graphml of {map_file}"
                ))
            };

            let mut state = MapState::new(map.clone());
            state.calculate_algorithm_settings();
            let settings = state.get_algorithm_settings();

            println!(
                "testing on map {map_file} with {} stations and {} edges",
                map.get_stations()
                    .len(),
                map.get_edges()
                    .len()
            );

            recalculate_map(settings, &mut map).expect(&format!(
                "failed to recalculate map {map_file}"
            ));
        }
    }
}
