//! Contains the [`Map`] struct and all its methods.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use super::{
    line::LineID,
    station::StationID,
    Edge,
    EdgeID,
    GridNode,
    Line,
    Station,
};
use crate::{
    algorithm::{
        drawing::CanvasContext,
        OccupiedNodes,
    },
    components::CanvasState,
    unwrap_or_return,
    utils::Result,
    Error,
};

/// Represents the metro map as a whole with all its lines and stations.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Map {
    /// A [`HashMap`] of all stations on the map.
    stations: HashMap<StationID, Station>,
    /// A [`HashMap`] of all lines on the map.
    lines: HashMap<LineID, Line>,
    /// A [`HashMap`] of all edges on the map.
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
            .find(|e| (e.is_from(from) && e.is_to(to)) || (e.is_from(to) && e.is_to(from)))
            .map(Edge::get_id)
    }

    /// Get the id of the [`Edge`] between the two given stations, else create
    /// one.
    pub fn get_edge_id_between(&mut self, from: StationID, to: StationID) -> EdgeID {
        if let Some(e) = self.get_edge_id_between_if_exists(from, to) {
            return e;
        }

        let new = Edge::new(from, to, None);
        let new_id = new.get_id();
        self.add_edge(new);

        new_id
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

    /// Add a station to the map, if a station already exists with that ID, it
    /// will be replaces.
    pub fn add_station(&mut self, station: Station) {
        self.stations
            .insert(station.get_id(), station);
    }

    /// Remove a station from the map.
    pub fn remove_station(&mut self, id: StationID) {
        let Some(station) = self
            .get_station(id)
            .cloned()
        else {
            return;
        };

        let lines: Vec<_> = self
            .lines
            .values()
            .cloned()
            .collect();
        for mut line in lines {
            line.remove_station(self, &station);
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
        self.lines
            .entry(id)
            .or_insert_with(|| Line::new(Some(id)));

        self.get_line(id)
            .expect("did not find newly inserted line")
    }

    /// Remove a line from the map.
    pub fn remove_line(&mut self, id: LineID) {
        let line = unwrap_or_return!(self
            .lines
            .remove(&id)
            .ok_or(Error::other("line to remove not found")));

        for edge_id in line.get_edges() {
            if let Some(edge) = self.get_mut_edge(*edge_id) {
                edge.remove_line(id);

                if edge
                    .get_lines()
                    .is_empty()
                {
                    self.remove_edge(*edge_id);
                }
            }
        }
    }

    /// Add an edge to map, if an edge with that ID already exists, it will get
    /// replaces.
    pub fn add_edge(&mut self, edge: Edge) {
        self.get_mut_station(edge.get_from())
            .expect("from station not found")
            .add_edge(edge.get_id());
        self.get_mut_station(edge.get_to())
            .expect("to station not found")
            .add_edge(edge.get_id());

        self.edges
            .insert(edge.get_id(), edge);
    }

    /// Remove an edge from the map.
    pub fn remove_edge(&mut self, id: EdgeID) {
        if let Some(edge) = self
            .edges
            .remove(&id)
        {
            if let Some(from_station) = self.get_mut_station(edge.get_from()) {
                from_station.remove_edge(id);
            }
            if let Some(to_station) = self.get_mut_station(edge.get_to()) {
                to_station.remove_edge(id);
            }
            for line in self.get_mut_lines() {
                line.remove_edge_raw(id);
            }
        }
    }

    /// Get the station located on the given grid node.
    pub fn station_at_node(&self, node: GridNode) -> Option<StationID> {
        self.stations
            .values()
            .find(|s| s.get_pos() == node)
            .map(Station::get_id)
    }

    /// Get the line that goes through the given grid node.
    pub fn line_at_node(&self, node: GridNode) -> Option<&Line> {
        self.lines
            .values()
            .find(|l| l.visits_node(self, node))
    }

    /// Get the edge located on the given grid node.
    pub fn edge_at_node(&self, node: GridNode) -> Option<EdgeID> {
        self.edges
            .values()
            .find(|s| s.visits_node(self, node))
            .map(Edge::get_id)
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
            self.remove_edge(id);
        } else if let Some(edge) = self.get_mut_edge(id) {
            edge.set_lines(lines_found);
        }
    }

    /// Update the locations of stations and the edges from the ones in the
    /// partial map.
    pub fn update_from_partial(&mut self, other: &Map) -> Result<()> {
        for station in other.get_stations() {
            self.get_mut_station(station.get_id())
                .ok_or(Error::other(format!(
                    "Station {} from partial not found.",
                    station.get_id()
                )))?
                .set_pos(station.get_pos());
        }

        for edge in other.get_edges() {
            let own_id = self
                .get_edge_id_between_if_exists(edge.get_from(), edge.get_to())
                .ok_or(Error::other(format!(
                    "Edge going from {} to {} in partial not found in full map.\nEdges of {}: {:?}\nEdges of {}: {:?}", // FIXME: this gets triggered? Do some stations switch around in expand??
                    edge.get_from(),
                    edge.get_to(),
                    edge.get_from(),
                    self.get_station(edge.get_from())
                        .unwrap()
                        .get_edges(),
                    edge.get_to(),
                    self.get_station(edge.get_to())
                        .unwrap()
                        .get_edges(),
                )))?;
            self.get_mut_edge(own_id)
                .unwrap()
                .set_nodes(
                    edge.get_nodes()
                        .to_owned(),
                );
        }

        Ok(())
    }

    /// Get the nodes that are occupied by locked stations and edges.
    pub fn get_occupied_by_locks(&self) -> OccupiedNodes {
        let mut occupied = OccupiedNodes::new();
        for station in self.get_stations() {
            if station.is_locked() {
                occupied.insert(
                    station.get_pos(),
                    station
                        .get_id()
                        .into(),
                );
            }
        }
        for edge in self.get_edges() {
            if edge.is_locked() {
                for node in edge.get_nodes() {
                    occupied.insert(
                        *node,
                        edge.get_id()
                            .into(),
                    );
                }
            }
        }

        occupied
    }

    /// Draw the map to the given canvas.
    pub fn draw(&self, canvas: &CanvasContext<'_>, state: CanvasState, base_alpha: f64) {
        for edge in self.get_edges() {
            edge.draw(self, canvas, state, base_alpha);
        }

        for line in self.get_lines() {
            line.draw(self, canvas, state, base_alpha);
        }

        for station in self.get_stations() {
            station.draw(canvas, state, base_alpha);
        }
    }

    /// Use the A* algorithm to calculate the edges between all stations
    /// quickly.
    pub fn quickcalc_edges(&mut self) {
        let temp_map = self.clone();
        for edge in self.get_mut_edges() {
            if !edge.is_locked() {
                edge.calculate_nodes(&temp_map);
            }
        }
    }
}
