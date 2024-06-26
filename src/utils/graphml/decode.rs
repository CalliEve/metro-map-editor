//! Contains the functions used to decode a [`GraphMlMap`] and all its child
//! objects into a [`Map`].

use super::graphml_map::{
    GraphItem,
    GraphMlMap,
    Key,
    Node,
};
use crate::{
    models::{
        Line,
        Map,
        Station,
    },
    utils::calc_grid_loc,
};

/// Transforms an edge represented by a [`Key`] to a [`Line`].
fn edge_to_line(edge: &Key) -> Line {
    let mut line = Line::new(
        Vec::new(),
        Some(
            edge.id
                .clone(),
        ),
    );
    line.set_name(&edge.name);
    line.set_color((
        edge.r
            .as_ref()
            .map_or(0, |r| {
                r.parse()
                    .expect("invalid r color value")
            }),
        edge.g
            .as_ref()
            .map_or(0, |g| {
                g.parse()
                    .expect("invalid g color value")
            }),
        edge.b
            .as_ref()
            .map_or(0, |b| {
                b.parse()
                    .expect("invalid b color value")
            }),
    ));

    line
}

/// Transforms a [`Node`] into a [`Station`].
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

    let mut station = Station::new(
        station_loc,
        Some(
            node.id
                .clone(),
        ),
    );
    station.set_name(
        &node
            .data
            .iter()
            .find(|d| d.key == "label")
            .expect("no station name provided")
            .value
            .clone(),
    );

    station
}

/// GraphML sometimes has maps/stations located in weird places (like all x
/// coordinates being negative), this normalizes that to the lowest x and y
/// coordinates being 2.
fn normalize_stations(map: &mut Map) {
    let mut low_x = i32::MAX;
    let mut low_y = i32::MAX;

    for station in map.get_stations() {
        if station
            .get_pos()
            .0
            < low_x
        {
            low_x = station
                .get_pos()
                .0;
        }
        if station
            .get_pos()
            .1
            < low_y
        {
            low_y = station
                .get_pos()
                .1;
        }
    }

    for station in map.get_mut_stations() {
        let (x, y) = station.get_pos();
        station.set_pos((x - low_x + 2, y - low_y + 2));
    }
}

/// Translates the [`GraphMlMap`] to a [`Map`]
pub fn graphml_to_map(graph: &GraphMlMap, square_size: u32) -> Map {
    let mut map = Map::new();

    // First add a Line for every edge defined
    for key in &graph.key {
        if key.for_item == "edge" {
            map.add_line(edge_to_line(key));
        }
    }

    // Load in all the stations
    for item in &graph
        .graph
        .content
    {
        if let GraphItem::Node(n) = item {
            map.add_station(node_to_station(n, square_size));
        }
    }

    // Ensure the location of the stations is correct
    normalize_stations(&mut map);

    // Only load all the lines once we have loaded the stations they reference
    for item in &graph
        .graph
        .content
    {
        if let GraphItem::Edge(e) = item {
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
                line.add_station(target.clone(), Some(&source));
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::graphml::graphml_map::{
        Data,
        Edge,
        Graph,
    };

    #[test]
    fn test_edge_to_line() {
        let input = Key {
            id: "test".to_owned(),
            for_item: "edge".to_owned(),
            r: Some("30".to_owned()),
            g: Some("0".to_owned()),
            b: Some("235".to_owned()),
            name: "test line".to_owned(),
        };

        let result = edge_to_line(&input);

        let mut example = Line::new(Vec::new(), Some("test".to_owned()));
        example.set_color((30, 0, 235));
        example.set_name(&"test line");

        assert_eq!(result.get_id(), example.get_id());
        assert_eq!(result.get_color(), example.get_color());
        assert_eq!(result.get_name(), example.get_name());
    }

    #[test]
    fn test_node_to_station() {
        let node = Node {
            id: "test".to_owned(),
            data: vec![
                Data {
                    key: "x".to_owned(),
                    value: "120.0".to_owned(),
                },
                Data {
                    key: "label".to_owned(),
                    value: "test station".to_owned(),
                },
                Data {
                    key: "y".to_owned(),
                    value: "155".to_owned(),
                },
            ],
        };

        let result = node_to_station(&node, 30);

        let mut example = Station::new((4, 5), Some("test".to_owned()));
        example.set_name(&"test station");

        assert_eq!(result.get_id(), example.get_id());
        assert_eq!(result.get_pos(), example.get_pos());
        assert_eq!(result.get_name(), example.get_name());
    }

    #[test]
    fn test_normalize_stations() {
        let mut map = Map::new();
        map.add_station(Station::new((-3, 4), None));
        map.add_station(Station::new((10, 10), None));
        map.add_station(Station::new((0, 7), None));

        normalize_stations(&mut map);

        let edited_stations = map.get_stations();

        assert_eq!(edited_stations[0].get_pos(), (2, 2));
        assert_eq!(edited_stations[1].get_pos(), (15, 8));
        assert_eq!(edited_stations[2].get_pos(), (5, 5));
    }

    #[test]
    fn test_graphml_to_map() {
        let graphml = GraphMlMap {
            key: vec![
                Key {
                    id: "x".to_owned(),
                    for_item: "node".to_owned(),
                    name: "x coordinate".to_owned(),
                    r: None,
                    g: None,
                    b: None,
                },
                Key {
                    id: "x".to_owned(),
                    for_item: "node".to_owned(),
                    name: "x coordinate".to_owned(),
                    r: None,
                    g: None,
                    b: None,
                },
                Key {
                    id: "x".to_owned(),
                    for_item: "node".to_owned(),
                    name: "x coordinate".to_owned(),
                    r: None,
                    g: None,
                    b: None,
                },
                Key {
                    id: "l0".to_owned(),
                    for_item: "edge".to_owned(),
                    name: "lineU1".to_owned(),
                    r: Some("84".to_owned()),
                    g: Some("167".to_owned()),
                    b: Some("33".to_owned()),
                },
            ],
            graph: Graph {
                content: vec![
                    GraphItem::Node(Node {
                        id: "n0".to_owned(),
                        data: vec![
                            Data {
                                key: "x".to_owned(),
                                value: "90".to_owned(),
                            },
                            Data {
                                key: "label".to_owned(),
                                value: "test 1".to_owned(),
                            },
                            Data {
                                key: "y".to_owned(),
                                value: "155".to_owned(),
                            },
                        ],
                    }),
                    GraphItem::Node(Node {
                        id: "n1".to_owned(),
                        data: vec![
                            Data {
                                key: "x".to_owned(),
                                value: "150".to_owned(),
                            },
                            Data {
                                key: "label".to_owned(),
                                value: "test 2".to_owned(),
                            },
                            Data {
                                key: "y".to_owned(),
                                value: "126".to_owned(),
                            },
                        ],
                    }),
                    GraphItem::Node(Node {
                        id: "n2".to_owned(),
                        data: vec![
                            Data {
                                key: "x".to_owned(),
                                value: "210".to_owned(),
                            },
                            Data {
                                key: "label".to_owned(),
                                value: "test 3".to_owned(),
                            },
                            Data {
                                key: "y".to_owned(),
                                value: "100".to_owned(),
                            },
                        ],
                    }),
                    GraphItem::Edge(Edge {
                        id: "e0".to_owned(),
                        source: "n0".to_owned(),
                        target: "n1".to_owned(),
                        data: vec![Data {
                            key: "l0".to_owned(),
                            value: "true".to_owned(),
                        }],
                    }),
                    GraphItem::Edge(Edge {
                        id: "e1".to_owned(),
                        source: "n1".to_owned(),
                        target: "n2".to_owned(),
                        data: vec![Data {
                            key: "l0".to_owned(),
                            value: "true".to_owned(),
                        }],
                    }),
                ],
            },
        };

        let map = graphml_to_map(&graphml, 30);

        let result_line = map
            .get_line("l0")
            .expect("no line with id l0");
        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");

        let result_station = map
            .get_station("n1")
            .expect("no station with id n1");
        assert_eq!(result_station.get_pos(), (4, 3));
        assert_eq!(result_station.get_name(), "test 2");
    }
}
