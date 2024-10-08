//! This module contains the Route Edges algorithm and all it needs.

use std::collections::HashMap;

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

/// Get a set of nodes in the radius around the given station.
fn get_node_set(
    settings: AlgorithmSettings,
    station: &Station,
    occupied: &OccupiedNodes,
) -> Vec<(GridNode, f64)> {
    if station.is_settled() {
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

            let distance = node.diagonal_distance_to(original_station_pos);
            if distance.ceil() as i32 <= radius {
                nodes.push((node, distance * settings.move_cost));
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
    from: GridNode,
    mut to_set: Vec<(GridNode, f64)>,
    to: GridNode,
) -> (
    Vec<(GridNode, f64)>,
    Vec<(GridNode, f64)>,
) {
    for (node, _) in &from_set.clone() {
        if *node == to {
            from_set.retain(|(n, _)| n != node);
            continue;
        } else if *node == from {
            to_set.retain(|(n, _)| n != node);
            continue;
        }

        if to_set
            .iter()
            .any(|(n, _)| n == node)
        {
            if node.diagonal_distance_to(from) > node.diagonal_distance_to(to) {
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
pub fn route_edges(
    settings: AlgorithmSettings,
    map: &mut Map,
    mut edges: Vec<Edge>,
) -> Result<OccupiedNodes> {
    let mut occupied = HashMap::new();

    for edge in &mut edges {
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

        let mut from_nodes = get_node_set(settings, from_station, &occupied);
        let mut to_nodes = get_node_set(settings, to_station, &occupied);

        if have_overlap(&from_nodes, &to_nodes) {
            (from_nodes, to_nodes) = split_overlap(
                from_nodes,
                from_station.get_pos(),
                to_nodes,
                to_station.get_pos(),
            );
        }
        // if from_nodes.is_empty() || to_nodes.is_empty() {
        //     debug_print(
        //         settings,
        //         &format!(
        //             "no nodes to route edge between stations {}{} and {}{}: from:
        // {:?}, to: {:?}",             from_station.get_id(),
        //             from_station.get_pos(),
        //             to_station.get_id(),
        //             to_station.get_pos(),
        //             from_nodes,
        //             to_nodes
        //         ),
        //         true,
        //     );
        //     return Err(Error::other(
        //         "no nodes to route edge between stations",
        //     ));
        // }

        if from_nodes.is_empty() {
            from_nodes.push((from_station.get_pos(), 0.0));
        }
        if to_nodes.is_empty() {
            to_nodes.push((to_station.get_pos(), 0.0));
        }

        debug_print(
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
            false,
        );

        let (start, nodes, end, _) = edge_dijkstra(
            settings,
            map,
            edge,
            &from_nodes,
            from_station,
            &to_nodes,
            to_station,
            &occupied,
        )?;

        debug_print(
            settings,
            &format!("routed edge from {start} to {end}",),
            false,
        );

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
        map.get_mut_station(edge.get_from())
            .ok_or(Error::other(
                "edge from-station not found",
            ))?
            .settle(start);
        map.get_mut_station(edge.get_to())
            .ok_or(Error::other(
                "edge to-station not found",
            ))?
            .settle(end);
        map.add_edge(edge.clone());
    }
    Ok(occupied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Station;

    #[test]
    fn test_get_node_set() {
        let mut map = Map::new();
        let station = Station::new((0, 0).into(), None);
        map.add_station(station.clone());

        let result = get_node_set(
            AlgorithmSettings::default(),
            &station,
            &mut HashMap::new(),
        );

        assert_eq!(
            result,
            vec![
                (GridNode::from((-3, 0)), 3.0),
                (GridNode::from((-2, 0)), 2.0),
                (
                    GridNode::from((-1, -1)),
                    GridNode::from((1, -1)).diagonal_distance_to(GridNode::from((0, 0)))
                ),
                (GridNode::from((-1, 0)), 1.0),
                (
                    GridNode::from((-1, 1)),
                    GridNode::from((1, -1)).diagonal_distance_to(GridNode::from((0, 0)))
                ),
                (GridNode::from((0, -3)), 3.0),
                (GridNode::from((0, -2)), 2.0),
                (GridNode::from((0, -1)), 1.0),
                (GridNode::from((0, 0)), 0.0),
                (GridNode::from((0, 1)), 1.0),
                (GridNode::from((0, 2)), 2.0),
                (GridNode::from((0, 3)), 3.0),
                (
                    GridNode::from((1, -1)),
                    GridNode::from((1, -1)).diagonal_distance_to(GridNode::from((0, 0)))
                ),
                (GridNode::from((1, 0)), 1.0),
                (
                    GridNode::from((1, 1)),
                    GridNode::from((1, -1)).diagonal_distance_to(GridNode::from((0, 0)))
                ),
                (GridNode::from((2, 0)), 2.0),
                (GridNode::from((3, 0)), 3.0),
            ]
        );
    }

    #[test]
    fn test_split_overlap() {
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

        let (from_set, to_set) = split_overlap(
            from_set,
            GridNode::from((0, 0)),
            to_set,
            GridNode::from((5, 5)),
        );

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
    fn test_route_edges() {
        let mut map = Map::new();
        let edges = vec![];

        let result = route_edges(
            AlgorithmSettings::default(),
            &mut map,
            edges,
        )
        .unwrap();

        assert_eq!(result, HashMap::new());
    }
}
