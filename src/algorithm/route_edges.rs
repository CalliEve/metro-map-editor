//! This module contains the Route Edges algorithm and all it needs.

use super::{
    edge_dijkstra::edge_dijkstra,
    have_overlap,
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
    _map: &Map, // FIXME: unused parameter
    station: &Station,
) -> Result<Vec<GridNode>> {
    if station.is_settled() {
        return Ok(vec![station.get_pos()]);
    }

    let radius = settings.node_set_radius;
    let station_pos = station.get_pos();
    let mut nodes = Vec::new();

    // Add all nodes in the radius around the station
    for x in (station_pos.0 - radius)..=(station_pos.0 + radius) {
        for y in (station_pos.1 - radius)..=(station_pos.1 + radius) {
            let node = GridNode::from((x, y));
            if node
                .diagonal_distance_to(station_pos)
                .ceil() as i32
                <= radius
            {
                nodes.push(node);
            }
        }
    }

    // TODO: include distance cost on the nodes
    Ok(nodes)
}

/// Split the overlap between the two node sets based on the distance to their
/// source.
fn split_overlap(
    mut from_set: Vec<GridNode>,
    from: GridNode,
    mut to_set: Vec<GridNode>,
    to: GridNode,
) -> (Vec<GridNode>, Vec<GridNode>) {
    for node in from_set
        .clone()
        .iter()
    {
        if node == &from {
            continue;
        }

        if *node == to || to_set.contains(node) {
            if node.diagonal_distance_to(from) > node.diagonal_distance_to(to) {
                from_set.retain(|n| n != node);
            } else {
                to_set.retain(|n| n != node);
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
) -> Result<Vec<Edge>> {
    for edge in edges.iter_mut() {
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

        let mut from_nodes = get_node_set(settings, map, from_station)?;
        let mut to_nodes = get_node_set(settings, map, to_station)?;

        if have_overlap(&from_nodes, &to_nodes) {
            (from_nodes, to_nodes) = split_overlap(
                from_nodes,
                from_station.get_pos(),
                to_nodes,
                to_station.get_pos(),
            );
        }

        let (start, nodes, end) = edge_dijkstra(map, from_nodes, to_nodes)?;

        edge.set_nodes(nodes);

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
    }
    Ok(edges)
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
            &map,
            &station,
        );

        assert_eq!(
            result,
            Ok(vec![
                GridNode::from((-3, 0)),
                GridNode::from((-2, 0)),
                GridNode::from((-1, -1)),
                GridNode::from((-1, 0)),
                GridNode::from((-1, 1)),
                GridNode::from((0, -3)),
                GridNode::from((0, -2)),
                GridNode::from((0, -1)),
                GridNode::from((0, 0)),
                GridNode::from((0, 1)),
                GridNode::from((0, 2)),
                GridNode::from((0, 3)),
                GridNode::from((1, -1)),
                GridNode::from((1, 0)),
                GridNode::from((1, 1)),
                GridNode::from((2, 0)),
                GridNode::from((3, 0)),
            ])
        );
    }

    #[test]
    fn test_split_overlap() {
        let from_set = vec![
            GridNode::from((0, 0)),
            GridNode::from((1, 1)),
            GridNode::from((1, 2)),
            GridNode::from((2, 2)),
            GridNode::from((3, 3)),
            GridNode::from((3, 4)),
            GridNode::from((4, 4)),
            GridNode::from((4, 5)),
        ];
        let to_set = vec![
            GridNode::from((1, 1)),
            GridNode::from((1, 2)),
            GridNode::from((2, 2)),
            GridNode::from((3, 3)),
            GridNode::from((3, 4)),
            GridNode::from((4, 4)),
            GridNode::from((4, 5)),
            GridNode::from((5, 5)),
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
                GridNode::from((0, 0)),
                GridNode::from((1, 1)),
                GridNode::from((1, 2)),
                GridNode::from((2, 2)),
            ]
        );
        assert_eq!(
            to_set,
            vec![
                GridNode::from((3, 3)),
                GridNode::from((3, 4)),
                GridNode::from((4, 4)),
                GridNode::from((4, 5)),
                GridNode::from((5, 5)),
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

        assert_eq!(result, vec![]);
    }
}
