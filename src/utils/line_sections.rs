use crate::models::{
    Edge,
    EdgeID,
    Map,
    StationID,
};

/// Get all the edges between two intersections where the given edge is
/// connected to. If `stop_at_locked` is true, the function will treat a locked
/// station as a line section end.
pub fn trace_line_section(map: &Map, edge_id: EdgeID, stop_at_locked: bool) -> Vec<Edge> {
    let edge = map
        .get_edge(edge_id)
        .expect("edge start of line section not found");

    let backwards = follow_line_section(
        map,
        edge.get_to(),
        edge_id,
        stop_at_locked,
    );

    let backwards_end = backwards
        .last()
        .unwrap();

    follow_line_section(
        map,
        backwards_end.get_from(),
        backwards_end.get_id(),
        stop_at_locked,
    )
}

/// Follow a line section from the given edge until an intersection or a locked
/// station is found.
fn follow_line_section(
    map: &Map,
    start_station: StationID,
    start_edge: EdgeID,
    stop_at_locked: bool,
) -> Vec<Edge> {
    let mut edges = Vec::new();
    let mut next = Some((start_station, start_edge));
    let lines = map
        .get_edge(start_edge)
        .expect("edge start of line section not found")
        .get_lines();

    while let Some((station_id, edge_id)) = next {
        let edge = map
            .get_edge(edge_id)
            .expect("edge of line section not found");
        edges.push(edge.clone());

        let opposite = edge
            .opposite(station_id)
            .expect("station in edge section does not have opposite");

        let station = map
            .get_station(opposite)
            .expect("station of line section not found");

        if station
            .get_edges()
            .len()
            != 2
            || (stop_at_locked && station.is_locked())
        {
            break;
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
            break;
        }

        next = Some((station.get_id(), next_edge.get_id()));
    }

    edges
}

/// Get the end stations of the given line section as defined by a vec of edges
/// and a list of all stations in between.
pub fn get_line_section_parts(line_section: &[Edge]) -> (Vec<StationID>, Vec<StationID>) {
    let mut ends = Vec::new();
    let mut middles = Vec::new();

    for edge in line_section {
        if ends.contains(&edge.get_from()) {
            ends.retain(|&id| id != edge.get_from());
            middles.push(edge.get_from());
        } else if middles.contains(&edge.get_from()) {
            continue;
        } else {
            ends.insert(0, edge.get_from());
        }

        if ends.contains(&edge.get_to()) {
            ends.retain(|&id| id != edge.get_to());
            middles.push(edge.get_to());
        } else if middles.contains(&edge.get_to()) {
            continue;
        } else {
            ends.push(edge.get_to());
        }
    }

    (ends, middles)
}
