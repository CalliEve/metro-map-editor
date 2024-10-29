use itertools::Itertools;
use rayon::prelude::*;

use super::models::{
    HeatmapData,
    NeighborData,
    StationHeatMap,
};
use crate::{
    algorithm::{
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
    (-2..=2)
        .into_iter()
        .cartesian_product((-2..=2).into_iter())
        .map(|(x, y)| GridNode(station_pos.0 + x, station_pos.1 + y))
        .collect()
}

/// Get data about the neighboring stations of the given station.
fn get_neighbor_data(map: &Map, station: &Station) -> Vec<NeighborData> {
    station
        .get_edges()
        .into_iter()
        .filter_map(|e_id| map.get_edge(*e_id))
        .filter_map(|e| e.opposite(station.get_id()))
        .filter_map(|s_id| map.get_station(s_id))
        .map(|neighbor| {
            NeighborData {
                station_id: neighbor.get_id(),
                distance: neighbor
                    .get_pos()
                    .diagonal_distance_to(station.get_pos()),
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
    occupied: OccupiedNodes,
) -> StationHeatMap {
    let possible_positions = get_possible_positions(station.get_pos());

    let heatmap = possible_positions
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

    StationHeatMap {
        station_id: station.get_id(),
        pos: station.get_pos(),
        original_pos: station.get_original_pos(),
        heatmap,
        neighbors: get_neighbor_data(map, station),
    }
}

/// Create the heatmap data for all stations in the map.
pub fn create_heatmap(
    settings: AlgorithmSettings,
    map: Map,
    occupied: OccupiedNodes,
) -> HeatmapData {
    let res = map
        .clone()
        .get_stations()
        .par_iter()
        .map(|station| {
            create_station_heatmap(
                settings,
                &map,
                station,
                occupied.clone(),
            )
        })
        .collect();

    HeatmapData {
        stations: res,
    }
}
