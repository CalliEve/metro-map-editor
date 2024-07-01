//! Contains the functions used to decode a [`GraphMlMap`] and all its child
//! objects into a [`Map`].

use super::graphml_map::{
    GraphItem,
    GraphMlMap,
    Key,
    Node,
};
use crate::{
    components::CanvasState,
    models::{
        GridNode,
        Line,
        Map,
        Station,
    },
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

/// Get the coordinates of a node from its data.
fn get_node_coords(node: &Node) -> (f64, f64) {
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
    )
}

/// Transforms a [`Node`] into a [`Station`].
fn node_to_station(node: &Node, state: CanvasState) -> Station {
    let station_loc = GridNode::from_canvas_pos(get_node_coords(node), state);

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
/// coordinates being negative or only difference being in the decimals), this
/// normalizes them so they fit within the canvas as it currently is.
fn normalize_stations(mut items: Vec<GraphItem>, state: CanvasState) -> Vec<GraphItem> {
    let square_size = state.drawn_square_size();

    let size_x = f64::from(
        state
            .get_size()
            .1,
    ) - 4.0 * square_size;
    let size_y = f64::from(
        state
            .get_size()
            .0,
    ) - 4.0 * square_size;

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for item in &items {
        if let GraphItem::Node(node) = item {
            let (x, y) = get_node_coords(node);
            if min_x > x {
                min_x = x;
            }
            if max_x < x {
                max_x = x;
            }
            if min_y > y {
                min_y = y;
            }
            if max_y < y {
                max_y = y;
            }
        }
    }

    for item in &mut items {
        if let GraphItem::Node(node) = item {
            let (x, y) = get_node_coords(node);

            let new_x = (x - min_x) / (max_x - min_x) * size_x + 2.0 * square_size;
            let new_y = (y - min_y) / (max_y - min_y) * size_y + 2.0 * square_size;

            node.data
                .iter_mut()
                .find(|d| d.key == "x")
                .expect("no x coordinate provided")
                .value = format!("{new_x}");
            node.data
                .iter_mut()
                .find(|d| d.key == "y")
                .expect("no y coordinate provided")
                .value = format!("{new_y}");
        }
    }

    items
}

/// Translates the [`GraphMlMap`] to a [`Map`]
pub fn graphml_to_map(mut graph: GraphMlMap, mut state: CanvasState) -> Map {
    state.set_zoom_factor(1.0);
    let mut map = Map::new();

    // First add a Line for every edge defined
    for key in &graph.key {
        if key.for_item == "edge" {
            map.add_line(edge_to_line(key));
        }
    }

    // Ensure the location of the stations is correct
    graph
        .graph
        .content = normalize_stations(
        graph
            .graph
            .content,
        state,
    );

    // Load in all the stations
    for item in &graph
        .graph
        .content
    {
        if let GraphItem::Node(n) = item {
            map.add_station(node_to_station(n, state));
        }
    }

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

        let result = node_to_station(&node, CanvasState::new());

        let mut example = Station::new((24, 31).into(), Some("test".to_owned()));
        example.set_name(&"test station");

        assert_eq!(result.get_id(), example.get_id());
        assert_eq!(result.get_pos(), example.get_pos());
        assert_eq!(result.get_name(), example.get_name());
    }

    #[test]
    fn test_normalize_stations() {
        let items = vec![
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
                        value: "200".to_owned(),
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
                        value: "120".to_owned(),
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
        ];

        let result = normalize_stations(items, CanvasState::new());

        if let GraphItem::Node(node) = &result[0] {
            assert_eq!(get_node_coords(&node), (10.0, 290.0));
        }
        if let GraphItem::Node(node) = &result[1] {
            assert_eq!(get_node_coords(&node), (150.0, 66.0));
        }
        if let GraphItem::Node(node) = &result[2] {
            assert_eq!(get_node_coords(&node), (290.0, 10.0));
        }
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

        let map = graphml_to_map(graphml, CanvasState::new());

        let result_line = map
            .get_line("l0")
            .expect("no line with id l0");
        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");

        let result_station = map
            .get_station("n1")
            .expect("no station with id n1");
        assert_eq!(result_station.get_pos(), (30, 28));
        assert_eq!(result_station.get_name(), "test 2");
    }
}
