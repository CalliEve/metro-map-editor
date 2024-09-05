//! Contains the [`Map`] struct and all its methods.

use std::collections::HashMap;

use super::{
    line::LineID,
    station::StationID,
    Edge,
    EdgeID,
    GridNode,
    Line,
    Station,
};
use crate::components::CanvasState;

/// Represents the metro map as a whole with all its lines and stations.
#[derive(Clone, Debug, Default)]
pub struct Map {
    stations: HashMap<StationID, Station>,
    lines: HashMap<LineID, Line>,
    edges: HashMap<EdgeID, Edge>,
}

impl Map {
    /// Creates a new, empty map.
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
            lines: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Get a [`Station`] with the given id.
    pub fn get_station(&self, id: StationID) -> Option<&Station> {
        self.stations
            .get(&id)
    }

    /// Get a mutable [`Station`] with the given id.
    pub fn get_mut_station(&mut self, id: StationID) -> Option<&mut Station> {
        self.stations
            .get_mut(&id)
    }

    /// Get a [`Line`] with the given id.
    pub fn get_line(&self, id: LineID) -> Option<&Line> {
        self.lines
            .get(&id)
    }

    /// Get a mutable [`Line`] with the given id.
    pub fn get_mut_line(&mut self, id: LineID) -> Option<&mut Line> {
        self.lines
            .get_mut(&id)
    }

    /// Get a list of all [`Line`]s on the map.
    pub fn get_lines(&self) -> Vec<&Line> {
        self.lines
            .values()
            .collect()
    }

    /// Get a mutable reference to all [`Line`]s on the map.
    pub fn get_mut_lines(&mut self) -> Vec<&mut Line> {
        self.lines
            .values_mut()
            .collect()
    }

    /// Get a [`Edge`] with the given id.
    pub fn get_edge(&self, id: EdgeID) -> Option<&Edge> {
        self.edges
            .get(&id)
    }

    /// Get a mutable [`Edge`] with the given id.
    pub fn get_mut_edge(&mut self, id: EdgeID) -> Option<&mut Edge> {
        self.edges
            .get_mut(&id)
    }

    /// Get a list of all [`Edge`]s on the map.
    pub fn get_edges(&self) -> Vec<&Edge> {
        self.edges
            .values()
            .collect()
    }

    /// Get a mutable reference to all [`Edge`]s on the map.
    pub fn get_mut_edges(&mut self) -> Vec<&mut Edge> {
        self.edges
            .values_mut()
            .collect()
    }

    /// Get the [`Edge`] between the two given stations.
    pub fn get_edge_id_between_if_exists(&self, from: StationID, to: StationID) -> Option<EdgeID> {
        self.get_edges()
            .into_iter()
            .find(|e| e.is_from(from) && e.is_to(to))
            .map(|e| e.get_id())
    }

    /// Get the id of the [`Edge`] between the two given stations, else create
    /// one.
    pub fn get_edge_id_between(&mut self, from: StationID, to: StationID) -> EdgeID {
        if let Some(e) = self.get_edge_id_between_if_exists(from, to) {
            return e;
        }

        let new = Edge::new(from, to, None);
        self.edges
            .insert(new.get_id(), new);
        self.get_edge_id_between_if_exists(from, to)
            .expect("can't find newly created edge")
    }

    /// A getter for the stations on the map.
    pub fn get_stations(&self) -> Vec<&Station> {
        self.stations
            .values()
            .collect()
    }

    /// A mutable getter for the stations on the map.
    pub fn get_mut_stations(&mut self) -> Vec<&mut Station> {
        self.stations
            .values_mut()
            .collect()
    }

    /// Add a station to the map.
    pub fn add_station(&mut self, station: Station) {
        self.stations
            .insert(station.get_id(), station);
    }

    /// Remove a station from the map.
    pub fn remove_station(&mut self, id: StationID) {
        let lines: Vec<_> = self
            .lines
            .values()
            .cloned()
            .collect();
        for mut line in lines.into_iter() {
            line.remove_station(self, id);
            self.add_line(line);
        }

        self.stations
            .remove(&id);
    }

    /// Add a line to the map.
    pub fn add_line(&mut self, line: Line) {
        for edge_id in line.get_edges() {
            if let Some(edge) = self.get_mut_edge(*edge_id) {
                edge.add_line(line.get_id());
            }
        }

        self.lines
            .insert(line.get_id(), line);
    }

    /// Get mutable [`Line`] if exists, else add new line with that [`LineID`]
    /// and return it.
    pub fn get_or_add_line(&mut self, id: LineID) -> &Line {
        if !self
            .lines
            .contains_key(&id)
        {
            self.lines
                .insert(id, Line::new(Some(id)));
        }

        return self
            .get_line(id)
            .expect("did not find newly inserted line");
    }

    /// Add an edge to map.
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .insert(edge.get_id(), edge);
    }

    /// Get the station located on the given grid node.
    pub fn station_at_node(&self, node: GridNode) -> Option<StationID> {
        self.stations
            .values()
            .find(|s| s.get_pos() == node)
            .map(|s| s.get_id())
    }

    /// Get the line that goes through the given grid node.
    pub fn line_at_node(&self, node: GridNode) -> Option<&Line> {
        self.lines
            .values()
            .find(|l| l.visits_node(self, node))
    }

    /// Draw the map to the given canvas.
    pub fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, state: CanvasState) {
        for edge in self.get_edges() {
            edge.draw(self, canvas, state);
        }

        for line in self.get_lines() {
            line.draw(self, canvas, state);
        }

        for station in self.get_stations() {
            station.draw(canvas, state);
        }
    }

    /// Notify that the given edge was removed from a line and thus all lines
    /// should be check and the edge fully removed if not in use any other than
    /// from.
    pub fn removed_edge(&mut self, id: EdgeID, from: LineID) {
        let mut lines_found = Vec::new();
        for line in self
            .lines
            .values()
        {
            if line.get_id() != from
                && line
                    .get_edges()
                    .contains(&id)
            {
                lines_found.push(line.get_id());
            }
        }

        if lines_found.is_empty() {
            self.edges
                .remove(&id);
        } else if let Some(edge) = self.get_mut_edge(id) {
            edge.set_lines(lines_found);
        }
    }
}
