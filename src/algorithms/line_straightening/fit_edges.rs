//! Contains functions to attach stations to a straightened line and calculate
//! the cost of the new line.

use std::{
    collections::HashMap,
    mem,
};

use itertools::Itertools;
use ordered_float::NotNan;

use crate::{
    models::{
        GridNode,
        Map,
        Station,
        StationID,
    },
    utils::{
        line_sections::LineSection,
        Result,
    },
    Error,
};

/// Returns a hashmap with the updated station positions after distributing the
/// stations over the given nodes.
pub fn attach_stations(
    map: &Map,
    original_section: &LineSection,
    new_nodes: &[GridNode],
) -> Result<HashMap<StationID, GridNode>> {
    //FIXME: Cannot handle intersections yet
    if original_section
        .middles
        .len()
        > new_nodes.len()
    {
        return Err(Error::EarlyAbort);
    }

    let mut updated = HashMap::new();
    let mut degree_twos: Vec<(Vec<StationID>, Vec<GridNode>)> = Vec::new();
    let mut between_stations: Vec<StationID> = Vec::new();
    let mut new_nodes = new_nodes.to_vec();
    let station_count = original_section
        .middles
        .len();

    for (i, station_id) in original_section
        .middles
        .iter()
        .enumerate()
    {
        let station = map
            .get_station(*station_id)
            .expect("station in section not found");

        if station
            .get_edges()
            .len()
            == 2
        {
            between_stations.push(*station_id);
        } else {
            let mut index = get_nearest_node(station, &new_nodes);

            let before_count = between_stations.len();
            let after_count = station_count - i - 1;

            if before_count >= index {
                index = before_count + 1;
            }
            if index + after_count >= new_nodes.len() {
                index -= index - (station_count - after_count) + 1;
            }
            if before_count >= index {
                return Err(Error::EarlyAbort);
            }

            degree_twos.push((
                mem::take(&mut between_stations),
                new_nodes[..index].to_vec(),
            ));
            updated.insert(*station_id, new_nodes[index]);
            new_nodes = new_nodes
                .into_iter()
                .skip(index + 1)
                .collect();
        }
    }

    degree_twos.push((
        mem::take(&mut between_stations),
        new_nodes,
    ));

    for (stations, nodes) in degree_twos {
        let updated_degree_twos = spread_evenly(&stations, &nodes);
        updated.extend(updated_degree_twos);
    }

    Ok(updated)
}

/// Returns the index of the node in the given list that is closest to the given
/// station.
fn get_nearest_node(station: &Station, nodes: &[GridNode]) -> usize {
    nodes
        .iter()
        .enumerate()
        .min_by_key(|(_, node)| {
            NotNan::new(
                station
                    .get_pos()
                    .diagonal_distance_to(**node),
            )
            .expect("Encountered NaN")
        })
        .map(|(index, _)| index)
        .expect("no nodes")
}

/// Spreads the stations equidistantly over the given nodes.
fn spread_evenly(stations: &[StationID], nodes: &[GridNode]) -> HashMap<StationID, GridNode> {
    let mut updated = HashMap::new();

    let mut between_count = (nodes.len() as f64 / (stations.len() as f64 + 1.0)).round() as usize;
    between_count = between_count.saturating_sub(1);

    let mut station_iter = stations.iter();
    let mut node_iter = nodes
        .iter()
        .dropping(between_count);

    while let Some(node) = node_iter.next() {
        let Some(station_id) = station_iter.next() else {
            break;
        };

        updated.insert(*station_id, *node);
        node_iter = node_iter.dropping(between_count);
    }

    updated
}

/// Calculates the cost of the new line.
/// The cost is the sum of the manhattan distances between the stations and
/// their new positions.
pub fn calculate_cost(
    map: &Map,
    section: &LineSection,
    updated: &HashMap<StationID, GridNode>,
) -> i32 {
    section
        .ends
        .iter()
        .chain(&section.middles)
        .filter_map(|id| map.get_station(*id))
        .map(|station| (station.get_id(), station.get_pos()))
        .fold(0, |cost, (id, node)| {
            node.manhattan_distance_to(
                *updated
                    .get(&id)
                    .expect("station in section not in updated"),
            ) + cost
        })
}
