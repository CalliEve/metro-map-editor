//! Contains functions to update the map with new station positions and edges as
//! a result of the straightening algorithm.

use std::collections::{
    HashMap,
    HashSet,
};

use super::line_creation::create_straight_edge;
use crate::{
    algorithms::{
        calc_direction::node_direction,
        edge_dijkstra,
        AlgorithmSettings,
        OccupiedNode,
        OccupiedNodes,
    },
    models::{
        Edge,
        GridNode,
        Map,
        StationID,
    },
    utils::{
        line_sections::LineSection,
        Result,
    },
};

/// Update the map with the new station positions and edges.
pub fn update_map(
    map: &mut Map,
    updated_stations: &HashMap<StationID, GridNode>,
    occupied: &mut OccupiedNodes,
    edges: &[Edge],
) -> Result<()> {
    for (id, new_pos) in updated_stations {
        map.get_mut_station(*id)
            .expect("station not found")
            .set_pos(*new_pos);
        occupied.insert(*new_pos, OccupiedNode::Station(*id));
    }

    for edge in edges {
        let direction = node_direction(
            updated_stations[&edge.get_from()],
            updated_stations[&edge.get_to()],
        );
        let nodes = create_straight_edge(
            map,
            occupied,
            updated_stations[&edge.get_from()],
            updated_stations[&edge.get_to()],
            direction,
        )?;

        for node in &nodes {
            occupied.insert(*node, OccupiedNode::Edge(edge.get_id()));
        }

        map.get_mut_edge(edge.get_id())
            .expect("edge not found")
            .set_nodes(nodes);
    }

    Ok(())
}

/// Recalculate the nodes of the edges adjacent to the updated stations that are
/// not part of the straightened line.
pub fn recalculate_adjacent_edges(
    settings: AlgorithmSettings,
    map: &mut Map,
    updated_stations: &HashMap<StationID, GridNode>,
    mut occupied: OccupiedNodes,
    edges: &[Edge],
) -> Result<()> {
    let mut updated_edges = edges
        .iter()
        .map(Edge::get_id)
        .collect::<HashSet<_>>();

    for id in updated_stations.keys() {
        let station = map
            .get_station(*id)
            .cloned()
            .expect("station not found");

        for edge_id in station.get_edges() {
            if updated_edges.contains(edge_id) {
                continue;
            }

            let edge = map
                .get_edge(*edge_id)
                .expect("edge not found");
            let from_station = map
                .get_station(edge.get_from())
                .expect("from-station of edge not found");
            let to_station = map
                .get_station(edge.get_to())
                .expect("to-station of edge not found");

            occupied.remove(&from_station.get_pos());
            occupied.remove(&to_station.get_pos());
            for node in edge.get_nodes() {
                occupied.remove(node);
            }

            let (_, nodes, ..) = edge_dijkstra(
                settings,
                map,
                edge,
                &[(from_station.get_pos(), 0.0)],
                from_station,
                &[(to_station.get_pos(), 0.0)],
                to_station,
                &occupied,
            )?;

            for node in &nodes {
                occupied.insert(*node, OccupiedNode::Edge(*edge_id));
            }
            occupied.insert(
                from_station.get_pos(),
                OccupiedNode::Station(from_station.get_id()),
            );
            occupied.insert(
                to_station.get_pos(),
                OccupiedNode::Station(to_station.get_id()),
            );

            map.get_mut_edge(*edge_id)
                .expect("edge not found")
                .set_nodes(nodes);

            updated_edges.insert(*edge_id);
        }
    }

    Ok(())
}

/// Remove all stations and edges from the selection from the occupied nodes
pub fn deoccupy_section(map: &Map, occupied: &mut OccupiedNodes, section: &LineSection) {
    for station_id in section
        .middles
        .iter()
        .chain(
            section
                .ends
                .iter(),
        )
    {
        let station = map
            .get_station(*station_id)
            .expect("station not found");
        occupied.remove(&station.get_pos());
    }

    for edge in &section.edges {
        let edge = map
            .get_edge(edge.get_id())
            .expect("edge not found");
        for node in edge.get_nodes() {
            occupied.remove(node);
        }
    }
}
