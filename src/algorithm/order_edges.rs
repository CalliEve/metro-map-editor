use std::collections::{
    BinaryHeap,
    HashMap,
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

/// Calculate the line degree of the given station.
/// This is the Line Degree algorithm in the paper.
fn line_degree(map: &Map, station_id: StationID) -> Result<usize> {
    let mut degree = 0;

    let station = map
        .get_station(station_id)
        .ok_or(Error::other(
            "station not found when calculating line degree",
        ))?;

    for edge_id in station.get_edges() {
        let edge = map
            .get_edge(*edge_id)
            .ok_or(Error::other(
                "edge connected to station not found",
            ))?;

        degree += edge
            .get_lines()
            .len();
    }

    Ok(degree)
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct HeapStation {
    station: StationID,
    degree: usize,
}

impl HeapStation {
    fn new(station: StationID, degree: usize) -> Self {
        Self {
            station,
            degree,
        }
    }
}

impl PartialOrd for HeapStation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.degree
                .cmp(&other.degree),
        )
    }
}

impl Ord for HeapStation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .degree
            .cmp(&self.degree)
    }
}

/// Calculate the line degree of all stations and return them in a hashmap, also
/// return the station with the highest degree.
fn calc_all_degrees(
    map: &Map,
) -> Result<(
    HashMap<StationID, HeapStation>,
    HeapStation,
)> {
    let mut stations = HashMap::new();
    let mut highest = HeapStation::new(0.into(), usize::MIN);

    for station_id in map
        .get_stations()
        .into_iter()
        .map(Station::get_id)
    {
        let degree = line_degree(map, station_id)?;
        stations.insert(
            station_id,
            HeapStation::new(station_id, degree),
        );

        if degree > highest.degree {
            highest = HeapStation::new(station_id, degree);
        }
    }

    Ok((stations, highest))
}

/// Order the edges in the map by the line degree of the stations they are
/// connected to. This is the Order Edges algorithm in the paper.
pub fn order_edges(map: &Map) -> Result<Vec<Edge>> {
    let mut edges = Vec::new();
    let mut queue = BinaryHeap::new();

    let (line_degree_map, highest) = calc_all_degrees(map)?;
    queue.push(highest);

    while let Some(station_with_degree) = queue.pop() {
        let station = map
            .get_station(station_with_degree.station)
            .ok_or(Error::other(
                "station not found when ordering edges",
            ))?;

        let mut station_edges = Vec::new();

        for edge_id in station.get_edges() {
            let edge = map
                .get_edge(*edge_id)
                .ok_or(Error::other(
                    "edge connected to station not found",
                ))?;

            if !edges.contains(edge) {
                let opposite_id = edge
                    .opposite(station.get_id())
                    .ok_or(Error::other(
                        "station not found on edge that it thought it was on",
                    ))?;

                let neighbor = line_degree_map
                    .get(&opposite_id)
                    .ok_or(Error::other(
                        "station not found in line degree map",
                    ))?;

                station_edges.push((
                    edge.clone(),
                    line_degree_map
                        .get(&opposite_id)
                        .ok_or(Error::other(
                            "station not found in line degree map",
                        ))?,
                ));
                queue.push(*neighbor);
            }
        }

        // Sort the edges by the degree of the opposite in descending order
        station_edges.sort_unstable_by(|a, b| {
            b.1.degree
                .cmp(&a.1.degree)
        });

        edges.append(
            &mut station_edges
                .into_iter()
                .map(|(edge, _)| edge)
                .collect::<Vec<_>>(),
        );
    }

    Ok(edges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::EdgeID,
        utils::json,
        CanvasState,
    };

    #[test]
    fn test_order_edges() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let test_file_content = std::fs::read_to_string("exisiting_maps/routing_test.json")
            .expect("test data file does not exist");
        let map = json::decode_map(&test_file_content, canvas).expect("failed to decode graphml");

        let edge1_id = map
            .get_edge_id_between_if_exists(5.into(), 4.into())
            .unwrap();
        let edge2_id = map
            .get_edge_id_between_if_exists(5.into(), 9.into())
            .unwrap();
        let edge3_id = map
            .get_edge_id_between_if_exists(5.into(), 3.into())
            .unwrap();
        let edge4_id = map
            .get_edge_id_between_if_exists(5.into(), 6.into())
            .unwrap();
        let edge5_id = map
            .get_edge_id_between_if_exists(4.into(), 8.into())
            .unwrap();
        let edge6_id = map
            .get_edge_id_between_if_exists(4.into(), 3.into())
            .unwrap();
        let edge7_id = map
            .get_edge_id_between_if_exists(4.into(), 2.into())
            .unwrap();
        let edge8_id = map
            .get_edge_id_between_if_exists(8.into(), 9.into())
            .unwrap();
        let edge9_id = map
            .get_edge_id_between_if_exists(9.into(), 10.into())
            .unwrap();
        let edge10_id = map
            .get_edge_id_between_if_exists(8.into(), 7.into())
            .unwrap();
        let edge11_id = map
            .get_edge_id_between_if_exists(3.into(), 1.into())
            .unwrap();

        let sorted = order_edges(&map).unwrap();
        let sorted_ids: Vec<EdgeID> = sorted
            .iter()
            .map(|edge| edge.get_id())
            .collect();

        assert_eq!(
            sorted_ids,
            vec![
                edge1_id, edge2_id, edge3_id, edge4_id, edge5_id, edge6_id, edge7_id, edge8_id,
                edge9_id, edge10_id, edge11_id
            ]
        );
    }
}
