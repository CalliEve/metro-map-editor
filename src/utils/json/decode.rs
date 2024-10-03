//! Contains the functions used to decode a [`JSONMap`] and all its child
//! objects into a [`Map`].

use super::json_models::{
    JSONMap,
    JSONStation,
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
            parse_color,
            parse_id,
        },
        Error,
        Result,
    },
};

/// JSON data sometimes has maps/stations located in weird places (like all x
/// coordinates being negative or only difference being in the decimals), this
/// normalizes them so they fit within the canvas as it currently is.
fn normalize_stations(mut stations: Vec<JSONStation>, state: CanvasState) -> Vec<JSONStation> {
    let coords = stations
        .iter()
        .map(|s| (s.x, s.y))
        .collect();

    let normalized_coords = normalize_coords(coords, state);

    for (station, (x, y)) in stations
        .iter_mut()
        .zip(normalized_coords)
    {
        station.x = x;
        station.y = y;
    }

    stations
}

/// Translates the [`JSONMap`] to a [`Map`]
pub fn json_to_map(mut graph: JSONMap, state: CanvasState) -> Result<Map> {
    let mut map = Map::new();

    graph.stations = normalize_stations(graph.stations, state);

    // Add stations
    for json_station in graph
        .stations
        .drain(..)
    {
        let mut station = Station::new(
            GridNode::from_canvas_pos((json_station.x, json_station.y), state),
            Some(parse_id(&json_station.id).into()),
        );

        if let Some(name) = json_station.name {
            station.set_name(&name);
        }

        map.add_station(station);
    }

    // Check there is no station overlap
    for station in map.get_stations() {
        if map
            .get_stations()
            .iter()
            .filter(|s| s.get_id() != station.get_id())
            .any(|s| s.get_pos() == station.get_pos())
        {
            return Err(Error::decode_error(format!(
                "station {}({}) has the same position as another station",
                station.get_name(),
                station.get_id()
            )));
        }
    }

    // Add lines
    for json_line in graph
        .lines
        .drain(..)
    {
        let mut line = Line::new(Some(parse_id(&json_line.id).into()));

        if let Some(name) = json_line.name {
            line.set_name(&name);
        }

        if let Some(color) = json_line.color {
            line.set_color(parse_color(&color)?);
        }

        map.add_line(line);
    }

    // Add edges
    for json_edge in graph
        .edges
        .drain(..)
    {
        let edge_id = map.get_edge_id_between(
            parse_id(&json_edge.source).into(),
            parse_id(&json_edge.target).into(),
        );

        // Add edge to lines
        for line_id in &json_edge.lines {
            let mut line = map
                .get_line(parse_id(line_id).into())
                .ok_or(Error::decode_error(format!(
                    "edge references non-existent line {line_id}",
                )))?
                .clone();

            line.add_edge(edge_id, &mut map);
            map.add_line(line);
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::json::json_models::{
        JSONEdge,
        JSONLine,
    };

    #[test]
    fn test_normalize_stations() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((100, 100));

        let result = normalize_stations(
            vec![
                JSONStation {
                    id: "1".to_string(),
                    name: None,
                    x: -1.0,
                    y: -3.0,
                },
                JSONStation {
                    id: "2".to_string(),
                    name: None,
                    x: 1.0,
                    y: 5.0,
                },
                JSONStation {
                    id: "3".to_string(),
                    name: None,
                    x: 3.0,
                    y: 1.0,
                },
            ],
            canvas,
        );

        assert_eq!(
            result,
            vec![
                JSONStation {
                    id: "1".to_string(),
                    name: None,
                    x: 10.0,
                    y: 10.0,
                },
                JSONStation {
                    id: "2".to_string(),
                    name: None,
                    x: 50.0,
                    y: 90.0,
                },
                JSONStation {
                    id: "3".to_string(),
                    name: None,
                    x: 90.0,
                    y: 50.0,
                }
            ]
        );
    }

    #[test]
    fn test_json_to_map() {
        let mut canvas = CanvasState::new();
        canvas.set_square_size(5);
        canvas.set_size((100, 100));

        let result = json_to_map(
            JSONMap {
                stations: vec![
                    JSONStation {
                        id: "0".to_string(),
                        name: None,
                        x: -1.0,
                        y: -3.0,
                    },
                    JSONStation {
                        id: "1".to_string(),
                        name: Some("test 2".to_string()),
                        x: 1.0,
                        y: 5.0,
                    },
                    JSONStation {
                        id: "s3".to_string(),
                        name: None,
                        x: 3.0,
                        y: 1.0,
                    },
                ],
                lines: vec![JSONLine {
                    id: "0".to_string(),
                    name: Some("lineU1".to_string()),
                    color: Some("rgb(84, 167, 33)".to_string()),
                }],
                edges: vec![
                    JSONEdge {
                        source: "0".to_string(),
                        target: "1".to_string(),
                        lines: vec!["0".to_string()],
                    },
                    JSONEdge {
                        source: "1".to_string(),
                        target: "s3".to_string(),
                        lines: vec!["0".to_string()],
                    },
                ],
            },
            canvas,
        )
        .unwrap();

        assert_eq!(
            result
                .get_stations()
                .len(),
            3
        );
        assert_eq!(
            result
                .get_lines()
                .len(),
            1
        );
        assert_eq!(
            result
                .get_edges()
                .len(),
            2
        );

        let mut edges = result
            .get_edges()
            .iter()
            .map(|e| e.get_id())
            .collect::<Vec<_>>();
        edges.sort();

        let result_line = result
            .get_line(0.into())
            .expect("no line with id 0");
        let mut line_edges = result_line
            .get_edges()
            .to_vec();
        line_edges.sort();

        assert_eq!(result_line.get_color(), (84, 167, 33));
        assert_eq!(result_line.get_name(), "lineU1");
        assert_eq!(line_edges, edges);

        let result_station = result
            .get_station(1.into())
            .expect("no station with id 1");
        assert_eq!(result_station.get_pos(), (10, 18));
        assert_eq!(result_station.get_name(), "test 2");
    }
}
