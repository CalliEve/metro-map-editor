use crate::models::{
    Edge,
    EdgeID,
    Map,
    StationID,
};

pub struct LineSection {
    pub edges: Vec<Edge>,
    pub ends: Vec<StationID>,
    pub middles: Vec<StationID>,
}

/// Get all the edges between two intersections where the given edge is
/// connected to. If `stop_at_locked` is true, the function will treat a locked
/// station as a line section end.
pub fn trace_line_section(map: &Map, edge_id: EdgeID, stop_at_locked: bool) -> LineSection {
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
        .edges
        .last()
        .unwrap();

    follow_line_section(
        map,
        backwards.ends[1],
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
) -> LineSection {
    let mut section = LineSection {
        edges: Vec::new(),
        ends: vec![start_station],
        middles: Vec::new(),
    };
    let mut next = Some((start_station, start_edge));
    let lines = map
        .get_edge(start_edge)
        .expect("edge start of line section not found")
        .get_lines();

    while let Some((station_id, edge_id)) = next {
        let edge = map
            .get_edge(edge_id)
            .expect("edge of line section not found");
        section
            .edges
            .push(edge.clone());

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
            section
                .ends
                .push(opposite);
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

        if section
            .edges
            .contains(next_edge)
            || next_edge.get_lines() != lines
        {
            section
                .ends
                .push(opposite);
            break;
        }

        section
            .middles
            .push(opposite);
        next = Some((station.get_id(), next_edge.get_id()));
    }

    section
}
