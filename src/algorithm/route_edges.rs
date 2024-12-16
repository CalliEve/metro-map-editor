//! This module contains the Route Edges algorithm and the functions for
//! determining the to and from node-sets that it needs.

use super::{
    edge_dijkstra::edge_dijkstra,
    log_print,
    occupation::OccupiedNodes,
    AlgorithmSettings,
    Updater,
};
use crate::{
    models::{
        Edge,
        GridNode,
        Map,
        Station,
    },
    utils::{
        IDManager,
        Result,
    },
    Error,
};

/// Get a set of nodes in the radius around the given station.
fn get_node_set(
    map: &Map,
    settings: AlgorithmSettings,
    station: &Station,
    occupied: &OccupiedNodes,
) -> Vec<(GridNode, f64)> {
    if station.is_settled() || station.has_locked_edge(map) {
        return vec![(station.get_pos(), 0.0)];
    }

    let radius = settings.node_set_radius;
    let original_station_pos = station.get_original_pos();
    let station_pos = station.get_pos();
    let mut nodes = Vec::new();

    // Add all nodes in the radius around the station
    for x in (station_pos.0 - radius)..=(station_pos.0 + radius) {
        for y in (station_pos.1 - radius)..=(station_pos.1 + radius) {
            let node = GridNode::from((x, y));
            if occupied.contains_key(&node) {
                continue;
            }

            let distance = node.manhattan_distance_to(original_station_pos);
            if distance <= radius {
                nodes.push((
                    node,
                    f64::from(distance) * settings.move_cost,
                ));
            }
        }
    }

    nodes
}

/// Check if two slices of grid nodes have any overlap.
fn have_overlap(left: &[(GridNode, f64)], right: &[(GridNode, f64)]) -> bool {
    for (node, _) in left {
        if right
            .iter()
            .any(|(n, _)| n == node)
        {
            return true;
        }
    }
    false
}

/// Split the overlap between the two node sets based on the distance to their
/// source.
#[allow(clippy::type_complexity)] // the return type is complex but makes sense here
fn split_overlap(
    mut from_set: Vec<(GridNode, f64)>,
    from: &Station,
    mut to_set: Vec<(GridNode, f64)>,
    to: &Station,
) -> (
    Vec<(GridNode, f64)>,
    Vec<(GridNode, f64)>,
) {
    for (node, _) in &from_set.clone() {
        // Ensure the station is always in their own set
        if *node == to.get_pos() {
            from_set.retain(|(n, _)| n != node);
            continue;
        } else if *node == from.get_pos() {
            to_set.retain(|(n, _)| n != node);
            continue;
        }

        if to_set
            .iter()
            .any(|(n, _)| n == node)
        {
            // If the node is in both sets, remove it from the set that it's furthest from
            // the station from.
            if node.diagonal_distance_to(from.get_original_pos())
                > node.diagonal_distance_to(to.get_original_pos())
            {
                from_set.retain(|(n, _)| n != node);
            } else {
                to_set.retain(|(n, _)| n != node);
            }
        }
    }

    (from_set, to_set)
}

/// Route all the edges on the map (as given by the input list of edges) and
/// return them. This is the Route Edges algorithm in the paper.
#[allow(clippy::too_many_lines)] // mostly due to large calls like debug prints
pub async fn route_edges(
    settings: AlgorithmSettings,
    map: &mut Map,
    mut edges: Vec<Edge>,
    mut occupied: OccupiedNodes,
    midway_updater: Updater,
) -> Result<OccupiedNodes> {
    for edge in &mut edges {
        if edge.is_locked() {
            continue;
        }

        let from_station = map
            .get_station(edge.get_from())
            .ok_or(Error::other(
                "from station on edge not found",
            ))?;
        let to_station = map
            .get_station(edge.get_to())
            .ok_or(Error::other(
                "to station on edge not found",
            ))?;

        let mut from_nodes = if settings.allow_station_relocation {
            get_node_set(map, settings, from_station, &occupied)
        } else {
            vec![(from_station.get_pos(), 0.0)]
        };
        let mut to_nodes = if settings.allow_station_relocation {
            get_node_set(map, settings, to_station, &occupied)
        } else {
            vec![(to_station.get_pos(), 0.0)]
        };

        if settings.allow_station_relocation {
            if have_overlap(&from_nodes, &to_nodes) {
                (from_nodes, to_nodes) = split_overlap(
                    from_nodes,
                    from_station,
                    to_nodes,
                    to_station,
                );
            }

            if from_nodes.is_empty() {
                from_nodes.push((from_station.get_pos(), 0.0));
            }
            if to_nodes.is_empty() {
                to_nodes.push((to_station.get_pos(), 0.0));
            }
        }

        log_print(
            settings,
            &format!(
                "routing edge from {}{} to {}{}, sets:\nfrom: {:?}\nto: {:?}",
                from_station.get_id(),
                from_station.get_pos(),
                to_station.get_id(),
                to_station.get_pos(),
                from_nodes,
                to_nodes
            ),
            super::LogType::Debug,
        );

        let (start, nodes, end, cost) = edge_dijkstra(
            settings,
            map,
            edge,
            &from_nodes,
            from_station,
            &to_nodes,
            to_station,
            &occupied,
        )?;

        log_print(
            settings,
            &format!(
                "routed edge {} ({} -> {}) from {start} to {end} at cost {cost}\nOriginally from {} to {}",
                edge.get_id(),
                from_station.get_id(),
                to_station.get_id(),
                from_station.get_pos(),
                to_station.get_pos(),
            ),
            super::LogType::Debug,
        );

        if nodes
            .iter()
            .any(|n| occupied.contains_key(n))
        {
            return Err(Error::other(format!(
                "Nodes of edge {} already occupied: {:?}",
                edge.get_id(),
                nodes
                    .iter()
                    .filter(|n| occupied.contains_key(n))
                    .collect::<Vec<_>>()
            )));
        }

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
        edge.settle();

        occupied.insert(
            start,
            from_station
                .get_id()
                .into(),
        );
        occupied.insert(
            end,
            to_station
                .get_id()
                .into(),
        );

        if let Some(start_station) = map.get_mut_station(edge.get_from()) {
            start_station.settle(start);
            start_station.add_cost(*cost);
        }
        if let Some(end_station) = map.get_mut_station(edge.get_to()) {
            end_station.settle(end);
            end_station.add_cost(*cost);
        }
        map.add_edge(edge.clone());

        if let Updater::Updater(updater) = midway_updater.clone() {
            updater(map.clone(), IDManager::to_data()).await;
        }
    }
    Ok(occupied)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use futures_test::test;

    use super::*;
    use crate::models::Station;

    #[test]
    async fn test_get_node_set() {
        let mut map = Map::new();
        let station = Station::new((0, 0).into(), None);
        map.add_station(station.clone());

        let result = get_node_set(
            &map,
            AlgorithmSettings::default(),
            &station,
            &mut HashMap::new(),
        );

        assert_eq!(
            result,
            vec![
                (GridNode::from((-3, 0)), 3.0),
                (GridNode::from((-2, -1)), 3.0),
                (GridNode::from((-2, 0)), 2.0),
                (GridNode::from((-2, 1)), 3.0),
                (GridNode::from((-1, -2)), 3.0),
                (GridNode::from((-1, -1)), 2.0),
                (GridNode::from((-1, 0)), 1.0),
                (GridNode::from((-1, 1)), 2.0),
                (GridNode::from((-1, 2)), 3.0),
                (GridNode::from((0, -3)), 3.0),
                (GridNode::from((0, -2)), 2.0),
                (GridNode::from((0, -1)), 1.0),
                (GridNode::from((0, 0)), 0.0),
                (GridNode::from((0, 1)), 1.0),
                (GridNode::from((0, 2)), 2.0),
                (GridNode::from((0, 3)), 3.0),
                (GridNode::from((1, -2)), 3.0),
                (GridNode::from((1, -1)), 2.0),
                (GridNode::from((1, 0)), 1.0),
                (GridNode::from((1, 1)), 2.0),
                (GridNode::from((1, 2)), 3.0),
                (GridNode::from((2, -1)), 3.0),
                (GridNode::from((2, 0)), 2.0),
                (GridNode::from((2, 1)), 3.0),
                (GridNode::from((3, 0)), 3.0),
            ]
        );
    }

    #[test]
    async fn test_split_overlap() {
        let from = Station::new((0, 0).into(), None);
        let from_set = vec![
            (GridNode::from((0, 0)), 0.0),
            (GridNode::from((1, 1)), 0.0),
            (GridNode::from((1, 2)), 0.0),
            (GridNode::from((2, 2)), 0.0),
            (GridNode::from((3, 3)), 0.0),
            (GridNode::from((3, 4)), 0.0),
            (GridNode::from((4, 4)), 0.0),
            (GridNode::from((4, 5)), 0.0),
        ];
        let to = Station::new((5, 5).into(), None);
        let to_set = vec![
            (GridNode::from((1, 1)), 0.0),
            (GridNode::from((1, 2)), 0.0),
            (GridNode::from((2, 2)), 0.0),
            (GridNode::from((3, 3)), 0.0),
            (GridNode::from((3, 4)), 0.0),
            (GridNode::from((4, 4)), 0.0),
            (GridNode::from((4, 5)), 0.0),
            (GridNode::from((5, 5)), 0.0),
        ];

        let (from_set, to_set) = split_overlap(from_set, &from, to_set, &to);

        assert_eq!(
            from_set,
            vec![
                (GridNode::from((0, 0)), 0.0),
                (GridNode::from((1, 1)), 0.0),
                (GridNode::from((1, 2)), 0.0),
                (GridNode::from((2, 2)), 0.0),
            ]
        );
        assert_eq!(
            to_set,
            vec![
                (GridNode::from((3, 3)), 0.0),
                (GridNode::from((3, 4)), 0.0),
                (GridNode::from((4, 4)), 0.0),
                (GridNode::from((4, 5)), 0.0),
                (GridNode::from((5, 5)), 0.0),
            ]
        );
    }

    #[test]
    async fn test_route_edges() {
        let mut map = Map::new();
        let edges = vec![];

        let result = route_edges(
            AlgorithmSettings::default(),
            &mut map,
            edges,
            HashMap::new(),
            Updater::NoUpdates,
        )
        .await
        .unwrap();

        assert_eq!(result, HashMap::new());
    }
}
