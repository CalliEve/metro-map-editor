//! Contains the local search algorithm for optimising the location of a
//! station.

use ordered_float::NotNan;

use super::{
    debug_print,
    edge_dijkstra::edge_dijkstra,
    occupation::OccupiedNodes,
    AlgorithmSettings,
};
use crate::{
    models::{
        Edge,
        GridNode,
        Map,
        Station,
    },
    utils::Result,
    Error,
};

/// Represents a station position with its edges and cost.
struct StationPos {
    /// The station at this position.
    station: Station,
    /// The edges connected to the station.
    edges: Vec<Edge>,
    /// The nodes occupied by map once this station and its edges have been
    /// taken into account.
    occupied: OccupiedNodes,
    /// The total cost of the station and its edges.
    cost: NotNan<f64>,
}

impl StationPos {
    /// Create a new [`StationPos`] with the given data.
    fn new(station: Station, edges: Vec<Edge>, occupied: OccupiedNodes, cost: NotNan<f64>) -> Self {
        Self {
            station,
            edges,
            occupied,
            cost,
        }
    }
}

/// Try a new position for the given station and return data on the result.
fn try_station_pos(
    settings: AlgorithmSettings,
    map: &Map,
    mut target_station: Station,
    station_pos: GridNode,
    mut occupied: OccupiedNodes,
) -> Result<StationPos> {
    let mut map = map.clone();

    target_station.set_pos(station_pos);

    let mut total_cost = NotNan::new(0.0).unwrap();
    let mut edges_before = Vec::new();
    let mut edges_after = Vec::new();

    for edge_id in target_station.get_edges() {
        let edge = map
            .get_mut_edge(*edge_id)
            .ok_or(Error::other(
                "edge of station not found",
            ))?;
        for node in edge.get_nodes() {
            occupied.remove(node);
        }
        edge.unsettle();
        edges_before.push(edge.clone());
    }

    for mut edge in edges_before {
        let to_station = map
            .get_station(
                edge.opposite(edge.get_to())
                    .unwrap(),
            )
            .ok_or(Error::other(
                "to-station of edge not found",
            ))?;
        let from_station = map
            .get_station(
                edge.opposite(edge.get_from())
                    .unwrap(),
            )
            .ok_or(Error::other(
                "from-station of edge not found",
            ))?;

        let from = vec![(from_station.get_pos(), 0.0)];
        let to = vec![(to_station.get_pos(), 0.0)];

        let (_, nodes, _, cost) = edge_dijkstra(
            settings,
            &map,
            &edge,
            &from,
            from_station,
            &to,
            to_station,
            &occupied,
        )?;

        occupied.extend(
            nodes
                .iter()
                .map(|n| {
                    (
                        *n,
                        edge.get_id()
                            .into(),
                    )
                }),
        );
        edge.set_nodes(nodes);
        map.add_edge(edge.clone());
        edges_after.push(edge);

        total_cost += cost;
    }

    Ok(StationPos::new(
        target_station,
        edges_after,
        occupied,
        total_cost,
    ))
}

/// Perform a local search on the map.
/// This will try to find a better position for each station.
/// This is the Local Search algorithm in the paper.
/// FIXME: This implementation is very slow
pub fn local_search(
    settings: AlgorithmSettings,
    map: &mut Map,
    occupied: &mut OccupiedNodes,
) -> Result<()> {
    let all_stations = map
        .get_stations()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    for station in all_stations {
        if station
            .get_edges()
            .is_empty()
        {
            continue;
        }

        let mut neighborhood = station
            .get_pos()
            .get_neighbors();
        neighborhood.push(station.get_pos());

        let mut best: Option<StationPos> = None;

        for node in neighborhood {
            if let Ok(station_pos) = try_station_pos(
                settings,
                map,
                station.clone(),
                node,
                occupied.clone(),
            ) {
                if best.is_none()
                    || station_pos.cost
                        < best
                            .as_ref()
                            .unwrap()
                            .cost
                {
                    best = Some(station_pos);
                }
            }
        }

        if best.is_none() {
            return Err(Error::other(format!(
                "no valid station positions found for station {}",
                station.get_id()
            )));
        }

        if station.get_pos()
            == best
                .as_ref()
                .unwrap()
                .station
                .get_pos()
        {
            debug_print(
                settings,
                &format!(
                    "Station {} stays at {}",
                    station.get_id(),
                    station.get_pos()
                ),
                false,
            );
        } else {
            debug_print(
                settings,
                &format!(
                    "Moving station {} from {} to {}",
                    station.get_id(),
                    station.get_pos(),
                    best.as_ref()
                        .unwrap()
                        .station
                        .get_pos()
                ),
                false,
            );
        }

        let best = best.unwrap();
        map.add_station(best.station);
        for edge in best.edges {
            map.add_edge(edge);
        }
        *occupied = best.occupied;
    }

    Ok(())
}
