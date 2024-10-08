//! Contains the functions used to decode a [`GraphMlMap`] and all its child
//! objects into a [`Map`].

use std::num::ParseIntError;

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
    utils::{
        parsing::{
            normalize_coords,
            parse_id,
        },
        Error,
        Result,
    },
};

/// Transforms an edge represented by a [`Key`] to a [`Line`].
fn edge_to_line(edge: &Key) -> Result<Line> {
    let mut line = Line::new(Some(parse_id(&edge.id).into()));
    line.set_name(&edge.name);
    line.set_color((
        edge.r
            .as_ref()
            .ok_or(Error::decode_error(
                "missing r color value",
            ))
            .and_then(|r| {
                r.parse::<u8>()
                    .map_err(|e: ParseIntError| {
                        Error::decode_error(format!(
                            "Invalid value for r color value: {e}"
                        ))
                    })
            })?,
        edge.g
            .as_ref()
            .ok_or(Error::decode_error(
                "missing g color value",
            ))
            .and_then(|r| {
                r.parse::<u8>()
                    .map_err(|e: ParseIntError| {
                        Error::decode_error(format!(
                            "Invalid value for g color value: {e}"
                        ))
                    })
            })?,
        edge.b
            .as_ref()
            .ok_or(Error::decode_error(
                "missing b color value",
            ))
            .and_then(|r| {
                r.parse::<u8>()
                    .map_err(|e: ParseIntError| {
                        Error::decode_error(format!(
                            "Invalid value for b color value: {e}"
                        ))
                    })
            })?,
    ));

    Ok(line)
}

/// Get the coordinates of a node from its data.
fn get_node_coords(node: &Node) -> Result<(f64, f64)> {
    Ok((
        node.data
            .iter()
            .find(|d| d.key == "x")
            .ok_or(Error::decode_error(
                "no x coordinate provided",
            ))?
            .value
            .parse()
            .map_err(|_| Error::decode_error("x coordinate is invalid"))?,
        node.data
            .iter()
            .find(|d| d.key == "y")
            .ok_or(Error::decode_error(
                "no x coordinate provided",
            ))?
            .value
            .parse()
            .map_err(|_| Error::decode_error("y coordinate is invalid"))?,
    ))
}

/// Transforms a [`Node`] into a [`Station`].
fn node_to_station(node: &Node, state: CanvasState) -> Result<Station> {
    let coords = get_node_coords(node)?;
    let station_loc = GridNode::from_canvas_pos(coords, state);

    let mut station = Station::new(
        station_loc,
        Some(parse_id(&node.id).into()),
    );
    station.set_name(
        &node
            .data
            .iter()
            .find(|d| d.key == "label")
            .ok_or(Error::decode_error(
                "no station name provided",
            ))?
            .value
            .clone(),
    );

    Ok(station)
}

/// GraphML sometimes has maps/stations located in weird places (like all x
/// coordinates being negative or only difference being in the decimals), this
/// normalizes them so they fit within the canvas as it currently is.
fn normalize_stations(mut items: Vec<GraphItem>, state: CanvasState) -> Result<Vec<GraphItem>> {
    let mut coords = items
        .iter()
        .filter_map(|item| {
            if let GraphItem::Node(node) = item {
                Some(get_node_coords(node))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>>>()?;

    coords = normalize_coords(coords, state);

    for (item, (x, y)) in items
        .iter_mut()
        .filter(|item| matches!(item, GraphItem::Node(_)))
        .zip(coords)
    {
        if let GraphItem::Node(node) = item {
            node.data
                .iter_mut()
                .find(|d| d.key == "x")
                .ok_or(Error::decode_error(
                    "no x coordinate provided",
                ))?
                .value = format!("{x}");
            node.data
                .iter_mut()
                .find(|d| d.key == "y")
                .ok_or(Error::decode_error(
                    "no y coordinate provided",
                ))?
                .value = format!("{y}");
        }
    }

    Ok(items)
}

/// Translates the [`GraphMlMap`] to a [`Map`]
pub fn graphml_to_map(mut graph: GraphMlMap, mut state: CanvasState) -> Result<Map> {
    state.set_zoom_factor(1.0);
    let mut map = Map::new();

    // First add a Line for every edge defined
    for key in &graph.key {
        if key.for_item == "edge" {
            map.add_line(edge_to_line(key)?);
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
    )?;

    // Load in all the stations
    for item in &graph
        .graph
        .content
    {
        if let GraphItem::Node(n) = item {
            map.add_station(node_to_station(n, state)?);
        }
    }

    // Check there is no station overlap
    // FIXME: instead of erroring, it should look for a free spot in its neighbors
    for station in map.get_stations() {
        if let Some(other) = map
            .get_stations()
            .iter()
            .filter(|s| s.get_id() != station.get_id())
            .find(|s| s.get_pos() == station.get_pos())
        {
            return Err(Error::decode_error(format!(
                "station {}({}) has the same position as another station {}({}) on this map of size {:?} with squares of size {}",
                station.get_name(),
                station.get_id(),
                other.get_name(),
                other.get_id(),
                state.get_size(),
                state.get_square_size()
            )));
        }
    }

    // Only load all the lines once we have loaded the stations they reference
    for item in &graph
        .graph
        .content
    {
        if let GraphItem::Edge(e) = item {
            for data in &e.data {
                let mut line = map
                    .get_mut_line(parse_id(&data.key).into())
                    .ok_or(Error::decode_error(format!(
                        "edge {} referenced non-existant line {}",
                        e.id, data.key
                    )))?
                    .clone();
                line.add_station(
                    &mut map,
                    parse_id(&e.source).into(),
                    Some(parse_id(&e.target).into()),
                    None,
                );
                map.add_line(line);
            }
        }
    }

    Ok(map)
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
            id: "2".to_owned(),
            for_item: "edge".to_owned(),
            r: Some("30".to_owned()),
            g: Some("0".to_owned()),
            b: Some("235".to_owned()),
            name: "test line".to_owned(),
        };

        let result = edge_to_line(&input).unwrap();

        let mut example = Line::new(Some(2.into()));
        example.set_color((30, 0, 235));
        example.set_name(&"test line");

        assert_eq!(result.get_id(), example.get_id());
        assert_eq!(result.get_color(), example.get_color());
        assert_eq!(result.get_name(), example.get_name());
    }

    #[test]
    fn test_node_to_station() {
        let node = Node {
            id: "2".to_owned(),
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
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let result = node_to_station(&node, canvas).unwrap();

        let mut example = Station::new((24, 31).into(), Some(2.into()));
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
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let result = normalize_stations(items, canvas).unwrap();

        if let GraphItem::Node(node) = &result[0] {
            assert_eq!(
                get_node_coords(&node),
                Ok((10.0, 290.0))
            );
        }
        if let GraphItem::Node(node) = &result[1] {
            assert_eq!(
                get_node_coords(&node),
                Ok((150.0, 66.0))
            );
        }
        if let GraphItem::Node(node) = &result[2] {
            assert_eq!(
                get_node_coords(&node),
                Ok((290.0, 10.0))
            );
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
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);

        let map = graphml_to_map(graphml, canvas).unwrap();

        let result_line = map
            .get_line(0.into())
            .expect("no line with id 0");
        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");

        let result_station = map
            .get_station(1.into())
            .expect("no station with id 1");
        assert_eq!(result_station.get_pos(), (30, 28));
        assert_eq!(result_station.get_name(), "test 2");
    }
}
