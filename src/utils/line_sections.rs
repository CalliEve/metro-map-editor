use std::collections::VecDeque;

use crate::models::{
    Edge,
    EdgeID,
    Map,
};

/// Get all the edges between two intersections where the given edge is
/// connected to. If `stop_at_locked` is true, the function will treat a locked
/// station as a line section end.
pub fn trace_line_section(map: &Map, edge_id: EdgeID, stop_at_locked: bool) -> Vec<Edge> {
    let edge = map
        .get_edge(edge_id)
        .expect("edge start of line section not found");
    let lines = edge.get_lines();
    let mut edges = vec![edge.clone()];

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
            || (stop_at_locked && station.is_locked())
        {
            continue;
        }

        let next_id = *station
            .get_edges()
            .iter()
            .find(|e| **e != edge.get_id())
            .expect("Should have one edge of two not equal to current");

        let next_edge = map
            .get_edge(next_id)
            .expect("Invalid next edge.");

        if edges.contains(next_edge) || next_edge.get_lines() != lines {
            continue;
        }

        queue.push_back((station.get_id(), next_edge.clone()));
        edges.push(next_edge.clone());
    }

    edges
}
