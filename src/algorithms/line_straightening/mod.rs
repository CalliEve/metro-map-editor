//! This module contains the line straightening algorithm.

use crate::{
    models::{
        Edge,
        Map,
        Station,
    },
    utils::{
        line_sections::to_line_section,
        Result,
    },
    Error,
};

mod fit_edges;
mod line_creation;
mod update_map;

use fit_edges::{
    attach_stations,
    calculate_cost,
};
use line_creation::create_edge_candidates;
use update_map::{
    deoccupy_section,
    recalculate_adjacent_edges,
    update_map,
};

use super::AlgorithmSettings;

/// This algorithm will try to find a straight line between the selected
/// stations. For this, it requires them to be part of one line section.
pub fn straighten_line(
    settings: AlgorithmSettings,
    map: &mut Map,
    selected_edges: &[Edge],
    selected_stations: &[Station],
) -> Result<()> {
    let mut occupied = map.get_occupied_nodes();
    let line_section = to_line_section(selected_stations, selected_edges)?;

    deoccupy_section(map, &mut occupied, &line_section);

    let start_station = map
        .get_station(line_section.ends[0])
        .expect("start station not found");
    let end_station = map
        .get_station(line_section.ends[1])
        .expect("end station not found");

    leptos::logging::log!("Getting edge candidates");
    let edge_candidates = create_edge_candidates(
        map,
        &occupied,
        start_station.get_pos(),
        end_station.get_pos(),
        &line_section.edges,
    )?;

    let result = edge_candidates
        .into_iter()
        .filter_map(|(start, mut nodes, end)| {
            leptos::logging::log!(
                "Trying straight line from {:?} to {:?}",
                start,
                end
            );

            let Ok(mut attached_stations) = attach_stations(map, &line_section, &nodes) else {
                return None;
            };

            attached_stations.insert(line_section.ends[0], start);
            attached_stations.insert(line_section.ends[1], end);
            nodes.insert(0, start);
            nodes.push(end);

            let cost = calculate_cost(map, &line_section, &attached_stations);

            Some((attached_stations, nodes, cost))
        })
        .min_by_key(|(_, _, cost)| *cost)
        .map(|(section, nodes, _)| (section, nodes))
        .ok_or(Error::other(
            "No straight line possible",
        ))?;

    leptos::logging::log!("Updating map");
    update_map(
        map,
        &result.0,
        &mut occupied,
        &line_section.edges,
    )?;

    leptos::logging::log!("Recalculating adjacent edges");
    recalculate_adjacent_edges(
        settings,
        map,
        &result.0,
        occupied,
        &line_section.edges,
    )?;

    leptos::logging::log!("Straightening done");

    Ok(())
}
