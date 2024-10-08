use std::collections::HashMap;

use leptos::logging;

use super::{
    debug_print,
    AlgorithmSettings,
};
use crate::{
    models::{
        Edge,
        Map,
        Station,
        StationID,
    },
    utils::Result,
    Error,
};

/// Check if the station has degree two and the edges are part of the same
/// lines.
fn station_has_degree_two(map: &Map, station: &Station) -> bool {
    if station
        .get_edges()
        .len()
        != 2
    {
        return false;
    }

    let edges = station
        .get_edges()
        .iter()
        .map(|id| {
            map.get_edge(*id)
                .unwrap()
        })
        .collect::<Vec<_>>();

    edges[0].get_lines() == edges[1].get_lines()
}

/// Check if the station can be contracted into an edge between its neighboring
/// stations given by start and end.
fn can_contract_into(
    settings: AlgorithmSettings,
    map: &Map,
    start: StationID,
    end: StationID,
) -> bool {
    if map
        .get_edge_id_between_if_exists(start, end)
        .is_some()
    {
        // Edge already exists, so we can't contract into it, skip.
        return false;
    }

    let min_distance = settings.node_set_radius * 2 + 1;

    let start_station = map
        .get_station(start)
        .unwrap();
    let end_station = map
        .get_station(end)
        .unwrap();

    start_station
        .get_pos()
        .manhattan_distance_to(end_station.get_pos())
        <= min_distance
}

/// Contract all stations with degree two into an edge between their neighboring
/// stations. Skips if there is already an edge between the neighboring
/// stations. Returns a hashmap of the contracted stations.
pub fn contract_stations(
    settings: AlgorithmSettings,
    map: &mut Map,
) -> HashMap<StationID, Station> {
    let mut contracted_stations = HashMap::new();

    let station_ids = map
        .get_stations()
        .into_iter()
        .map(Station::get_id)
        .collect::<Vec<_>>();

    for station_id in station_ids {
        let station = map
            .get_station(station_id)
            .unwrap()
            .clone();
        if !station_has_degree_two(map, &station) {
            continue;
        }

        let edges = station
            .get_edges()
            .iter()
            .map(|id| {
                map.get_edge(*id)
                    .unwrap()
            })
            .cloned()
            .collect::<Vec<_>>();

        let start = edges[0]
            .opposite(station_id)
            .unwrap();
        let end = edges[1]
            .opposite(station_id)
            .unwrap();

        if !can_contract_into(settings, map, start, end) {
            continue;
        }

        let new_edge_id = map.get_edge_id_between(start, end);

        let new_edge = map
            .get_mut_edge(new_edge_id)
            .unwrap();

        new_edge.extend_contracted_stations(edges[0].get_contracted_stations());
        new_edge.extend_contracted_stations(edges[1].get_contracted_stations());
        new_edge.add_contracted_station(station_id);

        map.remove_station(station_id);

        contracted_stations.insert(station.get_id(), station);
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

    let mut expand_station_ids = to_expand
        .iter()
        .map(Station::get_id)
        .collect::<Vec<_>>();
    expand_station_ids.insert(0, edge.get_from());
    expand_station_ids.push(edge.get_to());

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

        for line_id in edge.get_lines() {
            let mut line = map
                .get_line(*line_id)
                .unwrap()
                .clone();
            line.add_edge(new_edge_id, map);
            map.add_line(line);
        }

        let to_skip = if i == 0 { 0 } else { station_locs[i - 1] + 1 };
        let to_take = if *loc
            >= edge
                .get_nodes()
                .len()
        {
            loc - station_locs[i - 1]
        } else if *loc == 0 {
            *loc
        } else if i == 0 {
            loc - 1
        } else {
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

        let mut ref_station = map
            .get_station(edge.get_from())
            .ok_or(Error::other(
                "Edge with contracted stations has no start station",
            ))?
            .clone();
        let mut end_station = map
            .get_station(edge.get_to())
            .ok_or(Error::other(
                "Edge with contracted stations has no end station",
            ))?
            .clone();
        let first_node = edge
            .get_nodes()
            .first()
            .unwrap();
        if first_node.diagonal_distance_to(ref_station.get_pos())
            > first_node.diagonal_distance_to(end_station.get_pos())
        {
            std::mem::swap(&mut ref_station, &mut end_station);
        }

        // Sort by distance to reference station
        to_expand.sort_by(|a, b| {
            a.get_pos()
                .diagonal_distance_to(ref_station.get_pos())
                .partial_cmp(
                    &b.get_pos()
                        .diagonal_distance_to(ref_station.get_pos()),
                )
                .unwrap()
        });

        let step = (edge
            .get_nodes()
            .len() as f64)
            / (to_expand.len() as f64 + 1.0);
        let station_locs = (0..(to_expand.len() + 2))
            .map(|i| ((i as f64) * step) as usize)
            .collect::<Vec<_>>()[1..]
            .to_vec();

        debug_print(
            settings,
            &format!(
                "expand_len: {}, nodes_len: {}, station_locs: {:?}",
                to_expand.len(),
                edge.get_nodes()
                    .len(),
                station_locs
            ),
            false,
        );

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
