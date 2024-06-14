use std::i32;

use crate::{
    algorithm::{Line, Map, Station},
    utils::calc_grid_loc,
};

use super::graphml_map::{GraphItem, GraphMlMap, Key, Node};

fn edge_to_line(edge: &Key) -> Line {
    let mut line = Line::new(Vec::new(), &edge.id);
    line.set_name(&edge.name);
    line.set_color((
        edge.r
            .as_ref()
            .map_or(0, |r| r.parse().expect("invalid r color value")),
        edge.g
            .as_ref()
            .map_or(0, |g| g.parse().expect("invalid g color value")),
        edge.b
            .as_ref()
            .map_or(0, |b| b.parse().expect("invalid b color value")),
    ));

    line
}

fn node_to_station(node: &Node, square_size: u32) -> Station {
    let station_loc = calc_grid_loc(
        (
            node.data
                .iter()
                .find(|d| d.key == "x")
                .expect("no x coordinate provided")
                .value
                .parse()
                .expect("no valid x coordinate provided"),
            node.data
                .iter()
                .find(|d| d.key == "y")
                .expect("no y coordinate provided")
                .value
                .parse()
                .expect("no valid y coordinate provided"),
        ),
        square_size,
    );

    let mut station = Station::new(station_loc, Some(node.id.clone()));
    station.set_name(
        node.data
            .iter()
            .find(|d| d.key == "label")
            .expect("no station name provided")
            .value
            .clone(),
    );

    station
}

fn normalize_stations(map: &mut Map) {
    let mut low_x = i32::MAX;
    let mut low_y = i32::MAX;

    for station in map.get_stations() {
        if station.get_pos().0 < low_x {
            low_x = station.get_pos().0;
        }
        if station.get_pos().1 < low_y {
            low_y = station.get_pos().1
        }
    }

    for station in map.get_mut_stations() {
        let (x, y) = station.get_pos();
        station.set_pos((x - low_x + 2, y - low_y + 2));
    }
}

pub fn graphml_to_map(graph: GraphMlMap, square_size: u32) -> Map {
    let mut map = Map::new();

    for key in &graph.key {
        if key.for_item == "edge" {
            map.add_line(edge_to_line(key));
        }
    }

    for item in &graph.graph.content {
        match item {
            GraphItem::Node(n) => map.add_station(node_to_station(n, square_size)),
            _ => {}
        }
    }

    normalize_stations(&mut map);

    // Only load all the lines once we have loaded the stations they reference
    for item in &graph.graph.content {
        match item {
            GraphItem::Edge(e) => {
                let source = map
                    .get_station(&e.source)
                    .expect("edge source referenced non-existant station")
                    .clone();
                let target = map
                    .get_station(&e.target)
                    .expect("edge target referenced non-existant station")
                    .clone();

                for data in &e.data {
                    let line = map
                        .get_mut_line(&data.key)
                        .expect("edge referenced non-existant line");
                    line.add_station(source.clone());
                    line.add_station(target.clone());
                }
            }
            _ => {}
        }
    }

    map
}
