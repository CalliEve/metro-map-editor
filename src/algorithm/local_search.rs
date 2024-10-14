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

/// Calculate the total manhattan distance of a point to all neighboring
/// stations of the given station.
fn total_distance(map: &Map, node: GridNode, station: &Station) -> i32 {
    station
        .get_edges()
        .iter()
        .map(|id| {
            map.get_edge(*id)
                .expect("edge attached to station does not exist")
                .opposite(station.get_id())
                .expect("station does not have opposite")
        })
        .map(|id| {
            let neighbor = map
                .get_station(id)
                .expect("opposite station does not exist");
            node.manhattan_distance_to(neighbor.get_pos())
        })
        .sum()
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

    occupied.remove(&target_station.get_pos());
    target_station.set_pos(station_pos);
    occupied.insert(
        station_pos,
        target_station
            .get_id()
            .into(),
    );
    map.add_station(target_station.clone());

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
        let from_station = map
            .get_station(edge.get_from())
            .ok_or(Error::other(
                "from-station of edge not found",
            ))?;
        let to_station = map
            .get_station(edge.get_to())
            .ok_or(Error::other(
                "to-station of edge not found",
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
        if *total_cost >= target_station.get_cost() {
            return Err(Error::EarlyAbort);
        }
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
pub fn local_search(settings: AlgorithmSettings, map: &mut Map, occupied: &mut OccupiedNodes) {
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
        if station.is_locked() {
            continue;
        }

        let mut neighborhood = station
            .get_pos()
            .get_neighbors();

        neighborhood.sort_by(|a, b| {
            total_distance(map, *a, &station).cmp(&total_distance(map, *b, &station))
        });

        let mut best = None;

        'neighborhood: for node in neighborhood
            .into_iter()
            .take(4)
        {
            if let Ok(station_pos) = try_station_pos(
                settings,
                map,
                station.clone(),
                node,
                occupied.clone(),
            ) {
                if *station_pos.cost < station.get_cost() {
                    best = Some(station_pos);
                    break 'neighborhood;
                }
            }
        }

        if best.is_none() {
            continue; // CHECKME: we should implement an iterative checking
                      // maybe
        }

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

        let best = best.unwrap();
        map.add_station(best.station);
        for edge in best.edges {
            map.add_edge(edge);
        }
        *occupied = best.occupied;
    }
}
