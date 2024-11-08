//! Module containing the functions to create the heatmap data for all stations
//! in the given [`Map`].

use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

use super::models::{
    HeatmapData,
    NeighborData,
    StationHeatMap,
};
use crate::{
    algorithm::{
        total_distance,
        try_station_pos,
        AlgorithmSettings,
        OccupiedNodes,
    },
    models::{
        GridNode,
        Map,
        Station,
    },
    utils::calculate_angle,
};

/// Get the 5x5 grid of possible positions for the station around the given
/// station location.
fn get_possible_positions(station_pos: GridNode) -> Vec<GridNode> {
    (-3..=3)
        .cartesian_product(-3..=3)
        .map(|(x, y)| GridNode(station_pos.0 + x, station_pos.1 + y))
        .collect()
}

/// Get data about the neighboring stations of the given station.
fn get_neighbor_data(map: &Map, station: &Station) -> Vec<NeighborData> {
    station
        .get_edges()
        .iter()
        .filter_map(|e_id| map.get_edge(*e_id))
        .filter_map(|e| e.opposite(station.get_id()))
        .filter_map(|s_id| map.get_station(s_id))
        .map(|neighbor| {
            NeighborData {
                station_id: neighbor.get_id(),
                distance: neighbor
                    .get_pos()
                    .diagonal_distance_to(station.get_pos()),
                position: neighbor.get_pos(),
                angle: calculate_angle(
                    GridNode(
                        station
                            .get_pos()
                            .0,
                        station
                            .get_pos()
                            .1
                            - 10,
                    ),
                    station.get_pos(),
                    neighbor.get_pos(),
                ),
            }
        })
        .collect()
}

/// Create a heatmap around the given station by calling the `try_station_pos`
/// function from the local search algorithm.
fn create_station_heatmap(
    settings: AlgorithmSettings,
    map: &Map,
    station: &Station,
    occupied: &OccupiedNodes,
) -> StationHeatMap {
    let possible_positions = get_possible_positions(station.get_pos());

    let heatmap: HashMap<String, f64> = possible_positions
        .into_iter()
        .filter_map(|pos| {
            try_station_pos(
                settings,
                map,
                station.clone(),
                pos,
                occupied.clone(),
            )
            .map(|res| (pos.to_string(), *res.cost))
            .ok()
        })
        .collect();

    let mut neighbor_nodes = station
        .get_pos()
        .get_neighbors();

    neighbor_nodes.sort_by_key(|a| total_distance(map, *a, station));

    StationHeatMap {
        station_id: station.get_id(),
        pos: station.get_pos(),
        original_pos: station.get_original_pos(),
        heatmap,
        neighbors: get_neighbor_data(map, station),
        current_og_pos: occupied
            .get(&station.get_original_pos())
            .copied(),
        average_closest_coords: neighbor_nodes
            .into_iter()
            .take(4)
            .collect(),
    }
}

/// Create the heatmap data for all stations in the map.
pub fn create_heatmap(
    settings: AlgorithmSettings,
    map: &Map,
    occupied: &OccupiedNodes,
) -> HeatmapData {
    let res = map
        .clone()
        .get_stations()
        .par_iter()
        .map(|station| create_station_heatmap(settings, map, station, occupied))
        .collect();

    HeatmapData {
        stations: res,
    }
}
