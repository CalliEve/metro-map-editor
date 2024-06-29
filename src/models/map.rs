//! Contains the [`Map`] struct and all its methods.

use super::{
    Drawable,
    GridNode,
    Line,
    Station,
};

/// Represents the metro map as a whole with all its lines and stations.
#[derive(Clone, Debug, Default)]
pub struct Map {
    stations: Vec<Station>,
    lines: Vec<Line>,
}

impl Map {
    /// Creates a new, empty map.
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Get a [`Station`] with the given id.
    pub fn get_station(&self, id: &str) -> Option<&Station> {
        self.stations
            .iter()
            .find(|s| s.get_id() == id)
    }

    /// Get a [`Line`] with the given id.
    pub fn get_line(&self, id: &str) -> Option<&Line> {
        self.lines
            .iter()
            .find(|l| l.get_id() == id)
    }

    /// Get a mutable [`Line`] with the given id.
    pub fn get_mut_line(&mut self, id: &str) -> Option<&mut Line> {
        self.lines
            .iter_mut()
            .find(|l| l.get_id() == id)
    }

    /// Get a list of all [`Line`]s on the map.
    pub fn get_lines(&self) -> &[Line] {
        self.lines
            .as_ref()
    }

    /// Get a mutable reference to all [`Line`]s on the map.
    pub fn get_mut_lines(&mut self) -> &mut [Line] {
        self.lines
            .as_mut_slice()
    }

    /// Add a [`Line`] if it doesn't already exist, else replaces it. Also adds
    /// any stations not yet added before.
    pub fn add_line(&mut self, mut line: Line) {
        for station in line.get_mut_stations() {
            if let Some(existing) = self
                .stations
                .iter()
                .find(|s| s == &station)
            {
                *station = existing.clone();
            } else {
                self.stations
                    .push(station.clone());
            }
        }

        if let Some(existing) = self
            .lines
            .iter_mut()
            .find(|l| **l == line)
        {
            *existing = line;
            return;
        }

        self.lines
            .push(line);
    }

    /// A getter for the stations on the map.
    pub fn get_stations(&self) -> &[Station] {
        &self.stations
    }

    /// A mutable getter for the stations on the map.
    pub fn get_mut_stations(&mut self) -> &mut [Station] {
        &mut self.stations
    }

    /// Add a station to the map.
    pub fn add_station(&mut self, station: Station) {
        self.stations
            .push(station);
    }

    /// Get the station located on the given grid node.
    pub fn station_at_node(&self, node: GridNode) -> Option<&Station> {
        self.stations
            .iter()
            .find(|s| s.get_pos() == node)
    }

    /// Get the line that goes through the given grid node.
    pub fn line_at_node(&self, node: GridNode) -> Option<&Line> {
        self.lines
            .iter()
            .find(|l| l.visits_node(node))
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, square_size: u32) {
        leptos::logging::log!("redrawing");
        for line in &self.lines {
            line.draw(canvas, square_size);
        }
        for station in self.get_stations() {
            station.draw(canvas, square_size);
        }
    }
}
