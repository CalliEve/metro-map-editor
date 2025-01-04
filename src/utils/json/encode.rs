//! Contains the functions used to dencode a [`Map`] into a [`JSONMap`].

use super::json_models::{
    EdgeNode,
    JSONEdge,
    JSONLine,
    JSONMap,
    JSONStation,
};
use crate::{
    components::CanvasState,
    models::{
        Edge,
        Line,
        Map,
        Station,
    },
};

/// Encodes a [`Station`] as a [`JSONStation`].
fn encode_station(station: &Station, state: CanvasState) -> JSONStation {
    let pos = station
        .get_pos()
        .to_canvas_pos(state);

    let name = if station
        .get_name()
        .is_empty()
    {
        None
    } else {
        Some(
            station
                .get_name()
                .to_owned(),
        )
    };

    JSONStation {
        id: "s".to_owned() + &u64::from(station.get_id()).to_string(),
        x: pos.0,
        y: pos.1,
        name,
    }
}

/// Encodes a [`Line`] as a [`JSONLine`].
fn encode_line(line: &Line) -> JSONLine {
    let color = if line.get_color() == (0, 0, 0) {
        None
    } else {
        Some(line.get_color()).map(|(r, g, b)| format!("#{r:02X}{g:02X}{b:02X}"))
    };

    let name = if line
        .get_name()
        .is_empty()
    {
        None
    } else {
        Some(
            line.get_name()
                .to_owned(),
        )
    };

    JSONLine {
        id: "l".to_owned() + &u64::from(line.get_id()).to_string(),
        name,
        color,
    }
}

/// Encodes an [`Edge`] as a [`JSONEdge`].
fn encode_edge(edge: &Edge, state: CanvasState) -> JSONEdge {
    let source = "s".to_owned() + &u64::from(edge.get_from()).to_string();
    let target = "s".to_owned() + &u64::from(edge.get_to()).to_string();

    #[allow(unused_mut)]
    let mut lines = edge
        .get_lines()
        .iter()
        .map(|l| "l".to_owned() + &u64::from(*l).to_string())
        .collect::<Vec<_>>();

    // Sort lines in an edge for deterministic output
    #[cfg(test)]
    {
        lines.sort();
    }

    let mut nodes = Vec::new();
    for node in edge.get_nodes() {
        let pos = node.to_canvas_pos(state);
        nodes.push(EdgeNode {
            x: pos.0,
            y: pos.1,
        });
    }

    JSONEdge {
        source,
        target,
        nodes,
        lines,
    }
}

/// Translates the [`Map`] to a [`JSONMap`]
pub fn map_to_json(graph: &Map, state: CanvasState) -> JSONMap {
    let mut json_map = JSONMap {
        stations: Vec::new(),
        lines: Vec::new(),
        edges: Vec::new(),
    };

    let graph = graph.without_checkpoints();

    // Add stations
    json_map.stations = graph
        .get_stations()
        .into_iter()
        .map(|s| encode_station(s, state))
        .collect();

    // Sort stations for deterministic output
    #[cfg(test)]
    {
        json_map
            .stations
            .sort_by_key(|s| {
                s.id.clone()
            });
    }

    // Add lines
    json_map.lines = graph
        .get_lines()
        .into_iter()
        .map(encode_line)
        .collect();

    // Sort lines for deterministic output
    #[cfg(test)]
    {
        json_map
            .lines
            .sort_by_key(|e| {
                e.id.clone()
            });
    }

    // Add edges
    json_map.edges = graph
        .get_edges()
        .into_iter()
        .map(|e| encode_edge(e, state))
        .collect();

    // Sort edges for deterministic output
    #[cfg(test)]
    {
        json_map
            .edges
            .sort_by_key(|e| {
                e.source
                    .clone()
            });
    }

    json_map
}
