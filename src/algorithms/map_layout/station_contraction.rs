//! Contains the functions for contracting all degree two stations into edges
//! and expanding those edges back out again.

use std::collections::HashMap;

use super::AlgorithmSettings;
use crate::{
    algorithms::{
        log_print,
        LogType,
    },
    models::{
        Edge,
        Map,
        Station,
        StationID,
    },
    utils::{
        line_sections::{
            trace_line_section,
            LineSection,
        },
        Result,
    },
    Error,
};

/// Resolves a cycle of two stations in a line section by taking out the
/// starting edge from the line section.
fn resolve_two_station_cycle(mut line_section: LineSection) -> LineSection {
    for edge in &line_section
        .edges
        .clone()
    {
        if edge.get_from() == line_section.ends[0] {
            line_section
                .edges
                .retain(|e| e != edge);
            line_section.ends[0] = edge.get_to();
            break;
        } else if edge.get_to() == line_section.ends[0] {
            line_section
                .edges
                .retain(|e| e != edge);
            line_section.ends[0] = edge.get_from();
            break;
        }
    }

    line_section
        .middles
        .retain(|id| *id != line_section.ends[0]);
    line_section
}

/// Resolves a line section that is a cycle by taking the station with the most
/// edges connected out of the cycle together with the edges it is connected
/// to.
fn resolve_cycle(map: &Map, mut line_section: LineSection) -> LineSection {
    // Find the station with the most edges connected to it.
    let mut biggest_station = (line_section.middles[0], 0);
    for edge in &line_section.edges {
        let start = edge.get_from();
        let end = edge.get_to();
        let start_station = map
            .get_station(start)
            .unwrap();
        let end_station = map
            .get_station(end)
            .unwrap();

        if start_station
            .get_edges()
            .len()
            > biggest_station.1
        {
            biggest_station = (
                start,
                start_station
                    .get_edges()
                    .len(),
            );
        }

        if end_station
            .get_edges()
            .len()
            > biggest_station.1
        {
            biggest_station = (
                end,
                end_station
                    .get_edges()
                    .len(),
            );
        }
    }

    // Take out the biggest station and the edges it is connected to, this will
    // ensure the cycle will now have at least 3 stations in it after contraction.
    let mut ends = Vec::new();
    line_section
        .middles
        .retain(|id| *id != biggest_station.0);
    for edge in &line_section
        .edges
        .clone()
    {
        if edge.get_from() == biggest_station.0 {
            line_section
                .edges
                .retain(|e| e != edge);
            ends.push(edge.get_to());
            line_section
                .middles
                .retain(|id| *id != edge.get_to());
        } else if edge.get_to() == biggest_station.0 {
            line_section
                .edges
                .retain(|e| e != edge);
            ends.insert(0, edge.get_from());
            line_section
                .middles
                .retain(|id| *id != edge.get_from());
        }
    }
    line_section.ends = ends;

    line_section
}

/// Check if the station can be contracted into an edge between its neighboring
/// stations given by start and end.
fn can_contract_into(
    settings: AlgorithmSettings,
    map: &Map,
    start: StationID,
    end: StationID,
    station_count: usize,
) -> bool {
    let start_station = map
        .get_station(start)
        .unwrap();
    let end_station = map
        .get_station(end)
        .unwrap();

    let radius_mult = 2 - i32::from(start_station.is_locked()) - i32::from(end_station.is_locked());

    // Check if the stations are far enough apart. If they are too close, the
    // stations might become too close for the contracted station to be re-inserted
    // after the algorithm has ran its course.
    start_station
        .get_pos()
        .manhattan_distance_to(end_station.get_pos())
        > settings.node_set_radius * radius_mult + station_count as i32
}

/// Contract all stations with degree two into an edge between their neighboring
/// stations. Skips if there is already an edge between the neighboring
/// stations. Returns a hashmap of the contracted stations.
pub fn contract_stations(
    settings: AlgorithmSettings,
    map: &mut Map,
) -> HashMap<StationID, Station> {
    let mut contracted_stations = HashMap::new();

    let mut unchecked_edges = map
        .get_edges()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter();

    while let Some(edge) = unchecked_edges.next() {
        let mut line_section = trace_line_section(map, edge.get_id(), true);

        if line_section
            .ends
            .first()
            == line_section
                .ends
                .last()
        {
            if line_section
                .middles
                .len()
                <= 3
            {
                // Just skip, we need at least 4 stations to contract part of a cycle.
                continue;
            }
            line_section = resolve_cycle(map, line_section);
        } else if line_section
            .ends
            .len()
            != 2
        {
            panic!(
                "Line section does not have two ends, but instead {}",
                line_section
                    .ends
                    .len()
            );
        }

        let mut start = line_section.ends[0];
        let end = line_section.ends[1];
        if map
            .get_edge_id_between_if_exists(start, end)
            .is_some()
        {
            // Edge already exists, so we have to resolve the two station cycle this would
            // form.
            line_section = resolve_two_station_cycle(line_section);
            start = line_section.ends[0];
        }

        // Check for other edge cases preventing contraction.
        if !can_contract_into(
            settings,
            map,
            start,
            end,
            line_section
                .middles
                .len(),
        ) {
            continue;
        }

        // Create the new edge and retrieve it so we have a mutable reference to the
        // station object.
        let new_edge_id = map.get_edge_id_between(start, end);
        let new_edge = map
            .get_mut_edge(new_edge_id)
            .unwrap();

        new_edge.extend_contracted_stations(&line_section.middles);

        let middle_stations = line_section
            .middles
            .iter()
            .map(|id| {
                map.get_station(*id)
                    .unwrap()
            })
            .cloned()
            .collect::<Vec<_>>();

        for station in middle_stations {
            contracted_stations.insert(station.get_id(), station.clone());
            map.remove_station(station.get_id());
        }

        // Remove the edges that we contracted from our list of unchecked edges, as we
        // checked them by contracting them.
        unchecked_edges = unchecked_edges
            .filter(|e| {
                !line_section
                    .edges
                    .contains(e)
            })
            .collect::<Vec<_>>()
            .into_iter();
    }

    contracted_stations
}

/// Reinsert all contracted stations into the map.
/// The stations in `expand_stations` are reinserted into the map at the
/// locations given by `station_locs`.
fn reinsert_stations(
    map: &mut Map,
    edge: &Edge,
    to_expand: &mut [Station],
    station_locs: &[usize],
) {
    // Reinsert the contracted stations into the map at the given locations.
    for (station, loc) in to_expand
        .iter_mut()
        .zip(station_locs)
    {
        let node = edge
            .get_nodes()
            .get(*loc)
            .unwrap();

        station.set_pos(*node);
        station.clear_edges();
        map.add_station(station.clone());
    }

    // To get a vec of all station IDs that we need to add a new edge between, also
    // add the start and end station IDs of the edge to expand.
    let mut expand_station_ids = to_expand
        .iter()
        .map(Station::get_id)
        .collect::<Vec<_>>();
    expand_station_ids.insert(0, edge.get_from());
    expand_station_ids.push(edge.get_to());

    // Add in an edge for every pair of station ids. We also keep track of the
    // location of the end station and the index of that location.
    for ((start, end), (i, loc)) in expand_station_ids
        .iter()
        .zip(&expand_station_ids[1..])
        .zip(
            station_locs
                .iter()
                .enumerate(),
        )
    {
        let new_edge_id = map.get_edge_id_between(*start, *end);

        // Add the edge to the lines of the old edge.
        for line_id in edge.get_lines() {
            let mut line = map
                .get_line(*line_id)
                .unwrap()
                .clone();
            line.add_edge(new_edge_id, map);
            map.add_line(line);
        }

        if *loc == 0 {
            // end station location is 0, the amount of nodes on the new edge is thus 0 and
            // we can skip the rest.
            continue;
        }

        // Calculate the nodes to take from the old edge and set them on the new edge.
        let to_skip = if i == 0 { 0 } else { station_locs[i - 1] + 1 };
        let to_take = if *loc
            >= edge
                .get_nodes()
                .len()
        {
            // If the end station is the last contracted station, can take all nodes from
            // the start station up to the end of the nodes list.
            loc - station_locs[i - 1]
        } else if i == 0 {
            // If the end station is the first contracted station, can take all nodes up to
            // the location of this station.
            *loc
        } else {
            // Otherwise, take all nodes between the previous contracted station and the
            // current one.
            loc - station_locs[i - 1] - 1
        };

        map.get_mut_edge(new_edge_id)
            .unwrap()
            .set_nodes(
                edge.get_nodes()
                    .iter()
                    .copied()
                    .skip(to_skip)
                    .take(to_take)
                    .collect(),
            );
    }
}

/// Expand all contracted stations into new stations and edges.
/// The contracted stations are returned equidistantly between the two ends of
/// the edge they were contracted into.
pub fn expand_stations(
    settings: AlgorithmSettings,
    map: &mut Map,
    contracted_stations: &HashMap<StationID, Station>,
) -> Result<()> {
    let edges = map
        .get_edges()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    for edge in edges {
        // Get all stations that were contracted into the edge.
        let mut to_expand = edge
            .get_contracted_stations()
            .iter()
            .map(|id| {
                contracted_stations
                    .get(id)
                    .unwrap()
            })
            .cloned()
            .collect::<Vec<_>>();

        if to_expand.is_empty() {
            continue;
        }

        if to_expand.len()
            > edge
                .get_nodes()
                .len()
        {
            return Err(Error::other(format!(
                "Contracted edge {} has {} nodes while having {} contracted stations",
                edge.get_id(),
                edge.get_nodes()
                    .len(),
                to_expand.len()
            )));
        }

        // Calculate the new locations of the contracted stations on the edge, these are
        // equi-distance between the start and end stations of the edge they were
        // contracted into.
        let step = (edge
            .get_nodes()
            .len() as f64)
            / (to_expand.len() as f64 + 1.0);
        let station_locs = (0..(to_expand.len() + 2))
            .map(|i| ((i as f64) * step) as usize)
            .collect::<Vec<_>>()[1..]
            .to_vec();

        log_print(
            settings,
            &format!(
                "expand_len: {}, nodes_len: {}, station_locs: {:?}",
                to_expand.len(),
                edge.get_nodes()
                    .len(),
                station_locs
            ),
            LogType::Debug,
        );

        // FIXME: the order of stations can still flip!

        reinsert_stations(
            map,
            &edge,
            &mut to_expand,
            &station_locs,
        );

        map.remove_edge(edge.get_id());
    }

    Ok(())
}
