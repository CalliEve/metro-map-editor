use std::collections::HashMap;

use itertools::Itertools;

use super::{
    Error,
    Result,
};
use crate::models::{
    Edge,
    EdgeID,
    Map,
    Station,
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

/// Get the line section from the given stations and edges. The stations should
/// all be part of one continuous line section.
pub fn to_line_section(stations: &[Station], edges: &[Edge]) -> Result<LineSection> {
    let mut section = LineSection {
        edges: Vec::new(),
        ends: Vec::new(),
        middles: Vec::new(),
    };

    let edge_map = edges
        .iter()
        .map(|e| (e.get_id(), e.clone()))
        .collect::<HashMap<_, _>>();
    let station_map = stations
        .iter()
        .map(|s| (s.get_id(), s.clone()))
        .collect::<HashMap<_, _>>();

    for station in stations {
        let mut count = 0;
        for edge in station.get_edges() {
            if edge_map.contains_key(edge) {
                count += 1;
            }
        }

        if count == 1 {
            section
                .ends
                .push(station.get_id());
        }
    }

    if section
        .ends
        .len()
        != 2
    {
        return Err(Error::other(
            "Given selection is not one line section or contains a cycle.",
        ));
    }

    let mut current_station = station_map
        .get(&section.ends[0])
        .expect("Start station not found.")
        .clone();
    let edge_id = *current_station
        .get_edges()
        .iter()
        .find(|e| edge_map.contains_key(e))
        .expect("Start edge not found.");
    let mut current_edge = edge_map
        .get(&edge_id)
        .expect("Start edge not found.")
        .clone();

    section
        .edges
        .push(current_edge.clone());
    section
        .middles
        .clear();

    while let Some((next_station, next_edge)) = get_next_edge_station(
        &station_map,
        &edge_map,
        current_station.get_id(),
        &current_edge,
    ) {
        current_station = next_station;
        current_edge = next_edge;

        section
            .edges
            .push(current_edge.clone());

        if section.ends[1] == current_station.get_id() {
            break;
        }

        section
            .middles
            .push(current_station.get_id());
    }

    section.edges = section
        .edges
        .into_iter()
        .unique_by(Edge::get_id)
        .collect();

    Ok(section)
}

/// Get the next station and edge in the line section.
/// Used for traversing a line section.
fn get_next_edge_station(
    station_map: &HashMap<StationID, Station>,
    edge_map: &HashMap<EdgeID, Edge>,
    station_id: StationID,
    edge: &Edge,
) -> Option<(Station, Edge)> {
    let opposite = edge.opposite(station_id)?;

    let next_station = station_map.get(&opposite)?;

    let next_id = *next_station
        .get_edges()
        .iter()
        .find(|e| **e != edge.get_id() && edge_map.contains_key(*e))?;

    let next_edge = edge_map.get(&next_id)?;

    Some((next_station.clone(), next_edge.clone()))
}
