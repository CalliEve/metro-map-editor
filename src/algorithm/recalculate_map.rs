//! This module contains the Recalculate Map algorithm, which is the main
//! function to run the map algorithm.

use std::collections::HashMap;

use leptos::logging;

use super::{
    local_search::local_search,
    occupation::OccupiedNodes,
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
    algorithm::log_print,
    models::Map,
    utils::Result,
    Error,
};

/// Recalculate the map, all the positions of the stations and the edges between
/// them, as a whole. This is the Recalculate Map algorithm in the paper.
pub fn recalculate_map(settings: AlgorithmSettings, map: &mut Map) -> Result<OccupiedNodes> {
    if map
        .get_edges()
        .is_empty()
    {
        logging::warn!("Recalculate map called on an empty map");
        return Ok(HashMap::new());
    }

    log_print(
        settings,
        &format!(
            "Recalculating map with {} edges and {} stations",
            map.get_edges()
                .len(),
            map.get_stations()
                .len()
        ),
        super::LogType::Debug,
    );

    let contracted_stations = contract_stations(settings, map);

    log_print(
        settings,
        &format!(
            "Contracted stations, {} edges and {} stations left",
            map.get_edges()
                .len(),
            map.get_stations()
                .len()
        ),
        super::LogType::Debug,
    );

    map.quickcalc_edges();
    unsettle_map(map);

    let mut edges = order_edges(map)?;
    let mut attempt = 0;
    let mut found = false;
    let mut occupied = HashMap::new();

    log_print(
        settings,
        &format!("Ordered {} edges", edges.len()),
        super::LogType::Debug,
    );

    while !found {
        let mut alg_map = map.clone();

        attempt += 1;
        let res = route_edges(settings, &mut alg_map, edges.clone());

        if let Err(e) = res {
            log_print(
                settings,
                &format!("Failed to route edges: {e}"),
                super::LogType::Error,
            );

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

    log_print(
        settings,
        "Routed edges, commencing local search",
        super::LogType::Debug,
    );

    if settings.local_search {
        local_search(settings, map, &mut occupied);
    }

    log_print(
        settings,
        "Finished local search, re-adding contracted stations",
        super::LogType::Debug,
    );

    // Skip this step if heatmap is enabled as we need to keep the contracted
    // stations
    #[cfg(not(feature = "heatmap"))]
    expand_stations(settings, map, &contracted_stations)?;

    #[cfg(all(not(test), not(feature = "benchmarking")))]
    logging::log!("Recalculated map");

    Ok(occupied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorithm::{
            occupation::OccupiedNodes,
            LogType,
        },
        utils::{
            graphml,
            json,
        },
        CanvasState,
        MapState,
    };

    #[test]
    fn test_recalculate_map_no_overlap_check() {
        let map_file = "existing_maps/wien.graphml";

        let mut canvas = CanvasState::new();
        canvas.set_square_size(7);
        canvas.set_size((674, 1648));

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
        let mut settings = state.get_algorithm_settings();
        settings.edge_routing_attempts = 1;

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

        let mut occupied: OccupiedNodes = HashMap::new();
        for station in map.get_stations() {
            if let Some(existing) = occupied.insert(
                station.get_pos(),
                station
                    .get_id()
                    .into(),
            ) {
                panic!(
                    "station {:?} and {} have the same position",
                    existing,
                    station.get_id()
                );
            }
        }
        for edge in map.get_edges() {
            for node in edge.get_nodes() {
                if let Some(existing) = occupied.insert(
                    *node,
                    edge.get_id()
                        .into(),
                ) {
                    panic!(
                        "edge node {} of edge {} is already occupied by {:?}",
                        node,
                        edge.get_id(),
                        existing
                    );
                }
            }
        }
    }

    #[test]
    fn test_recalculate_map() {
        let map_files = vec![
            "existing_maps/disjointed_test.json",
            "existing_maps/routing_test.json",
            "existing_maps/montreal.graphml",
            "existing_maps/wien.graphml",
            "existing_maps/washington.graphml",
            "existing_maps/karlsruhe.graphml",
        ];
        let mut failed = Vec::new();

        for map_file in &map_files {
            let mut canvas = CanvasState::new();
            canvas.set_square_size(7);
            canvas.set_size((800, 1648));

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
            let mut settings = state.get_algorithm_settings();
            settings.edge_routing_attempts = 1;
            settings.log_level = LogType::Error;

            println!(
                "testing on map {map_file} with {} stations and {} edges",
                map.get_stations()
                    .len(),
                map.get_edges()
                    .len()
            );

            if let Err(e) = recalculate_map(settings, &mut map) {
                failed.push((map_file, e));
            }
        }

        if !failed.is_empty() {
            for (map_file, e) in failed {
                eprintln!("Failed on map {map_file}: {e}");
            }
            panic!("Some maps failed to recalculate");
        }
    }
}
