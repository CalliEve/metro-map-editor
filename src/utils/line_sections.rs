use std::collections::VecDeque;

use crate::models::{
    EdgeID,
    Map,
};

/// Get all the edges between two intersections where the given edge is
/// connected to.
pub fn trace_line_section(map: &Map, edge_id: EdgeID) -> Vec<EdgeID> {
    let edge = map
        .get_edge(edge_id)
        .expect("edge start of line section not found");
    let lines = edge.get_lines();
    let mut edges = vec![edge_id];

    let mut queue = VecDeque::new();
    queue.push_back((edge.get_to(), edge.clone()));
    queue.push_back((edge.get_from(), edge.clone()));

    while let Some((prev_station, edge)) = queue.pop_front() {
        let station_id = edge
            .opposite(prev_station)
            .unwrap();
        let station = map
            .get_station(station_id)
            .unwrap();
        if station
            .get_edges()
            .len()
            != 2
        {
            continue;
        }

        let next_id = *station
            .get_edges()
            .iter()
            .find(|e| **e != edge.get_id())
            .expect("Should have one edge of two not equal to current");
        if edges.contains(&next_id) {
            continue;
        }
        edges.push(next_id);

        let next_edge = map
            .get_edge(next_id)
            .expect("Invalid next edge.");
        queue.push_back((station.get_id(), next_edge.clone()));
    }

    edges
}
