//! Contains the [`Line`] struct and all its methods.

use std::{
    f64::consts::PI,
    sync::atomic::{
        AtomicU64,
        Ordering as AtomicOrdering,
    },
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{
    station::StationID,
    EdgeID,
    GridNode,
    Map,
};
use crate::components::CanvasState;

/// Next generated sequential identifier for a new line.
static LINE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineID(u64);

impl From<u64> for LineID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// Represents a metro line, including its stations, name and color.
#[derive(Clone, Debug)]
pub struct Line {
    /// ID of the line.
    id: LineID,
    /// Name of the line.
    name: String,
    /// Color of the line.
    color: (u8, u8, u8),
    /// All stations the line visits.
    stations: Vec<StationID>,
    /// All edges between the stations.
    edges: Vec<EdgeID>,
}

impl Line {
    /// Create a new [`Line`] with the stations it visits and an identifier.
    /// Color and name are set to default values.
    pub fn new(id: Option<LineID>) -> Self {
        Self {
            edges: Vec::new(),
            stations: Vec::new(),
            id: id.unwrap_or_else(|| {
                LINE_ID
                    .fetch_add(1, AtomicOrdering::SeqCst)
                    .into()
            }),
            color: (0, 0, 0),
            name: String::new(),
        }
    }

    /// A getter method for the stations the line visits.
    pub fn get_stations(&self) -> &[StationID] {
        &self.stations
    }

    /// A mutable getter method for the stations the line visits.
    pub fn get_mut_stations(&mut self) -> &mut [StationID] {
        &mut self.stations
    }

    /// Add a station. It will be inserted before the before station and after
    /// the after station, Or at the end of the line. If before isn't in the
    /// line yet, it will add both.
    pub fn add_station(
        &mut self,
        map: &mut Map,
        station: StationID,
        before: Option<StationID>,
        after: Option<StationID>,
    ) {
        if !self
            .stations
            .contains(&station)
        {
            self.stations
                .push(station.clone());
        }

        if let (Some(before_station), Some(after_station)) = (before, after) {
            if let Some(index) = self
                .edges
                .iter()
                .map(|id| {
                    map.get_edge(*id)
                        .expect("line edge list contains invalid id")
                })
                .position(|e| e.is_from(before_station) && e.is_to(after_station))
            {
                // replace edge with the station and the two edges connecting it
                let edge_id = self.edges[index];
                self.edges
                    .remove(index);
                map.removed_edge(edge_id, self.get_id());

                self.add_edge(
                    map.get_edge_id_between(station, before_station),
                    map,
                );
                self.add_edge(
                    map.get_edge_id_between(after_station, station),
                    map,
                );
                return;
            }
            unreachable!("Station inserted on an edge, but can't find the edge.");
        }

        if let Some(after_station) = after {
            // Insert edge between station and the station it comes before
            self.add_edge(
                map.get_edge_id_between(station, after_station),
                map,
            );
            return;
        }

        if let Some(before_station) = before {
            // Insert edge between station and the station it comes after
            self.add_edge(
                map.get_edge_id_between(before_station, station),
                map,
            );
        }
    }

    /// Add an edge that is being used by this line.
    pub fn add_edge(&mut self, edge_id: EdgeID, map: &mut Map) {
        let edge = map
            .get_mut_edge(edge_id)
            .expect("adding invalid edge id to line");
        edge.add_line(self.get_id());
        self.edges
            .push(edge_id);
    }

    /// A setter for the station's color.
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    /// A getter for the station's color.
    #[inline]
    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    /// A setter for the station's name.
    #[inline]
    pub fn set_name(&mut self, name: &impl ToString) {
        self.name = name.to_string();
    }

    /// A getter for the station's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// A getter for the station id.
    #[inline]
    pub fn get_id(&self) -> LineID {
        self.id
    }

    /// A getter for the edges the line uses.
    pub fn get_edges(&self) -> &[EdgeID] {
        &self.edges
    }

    /// Get a list of neighbors of the given station.
    pub fn get_station_neighbors(
        &self,
        map: &Map,
        station: StationID,
    ) -> (Vec<StationID>, Vec<StationID>) {
        let (mut before, mut after) = (Vec::new(), Vec::new());

        for id in &self.edges {
            let edge = map
                .get_edge(*id)
                .expect("invalid edge id");

            if edge.get_from() == station {
                after.push(edge.get_to());
            } else if edge.get_to() == station {
                before.push(edge.get_from());
            }
        }

        (before, after)
    }

    /// Gets the stations on either side of the position on this line.
    pub fn get_edge_stations(
        &self,
        map: &Map,
        node: GridNode,
    ) -> (Option<StationID>, Option<StationID>) {
        let mut from = None;
        let mut to = None;

        if self
            .stations
            .len()
            == 1
        {
            if let Some(station) = map.get_station(self.stations[0]) {
                if station
                    .get_pos()
                    .get_neighbors()
                    .contains(&node)
                {
                    return (Some(station.get_id()), None);
                }
            }
            return (None, None);
        }
        if self
            .stations
            .is_empty()
            || self
                .edges
                .is_empty()
        {
            return (None, None);
        }

        for id in &self.edges {
            if let Some(edge) = map.get_edge(*id) {
                if let Some(res) = edge.get_neigboring_stations(map, node) {
                    if res
                        .0
                        .is_some()
                        && res
                            .1
                            .is_some()
                    {
                        return res;
                    }

                    if res
                        .0
                        .is_some()
                        || res
                            .1
                            .is_some()
                    {
                        from = res.0;
                        to = res.1;
                    }
                }
            }
        }

        return (from, to);
    }

    /// Returns true if the line goes through the given grid node.
    pub fn visits_node(&self, map: &Map, node: GridNode) -> bool {
        if self
            .edges
            .iter()
            .any(|e| {
                map.get_edge(*e)
                    .expect("invalid edge id")
                    .visits_node(map, node)
            })
        {
            return true;
        }

        return self
            .get_line_ends(map)
            .into_iter()
            .any(|e| {
                map.get_station(e)
                    .expect("edge contains invalid station id")
                    .is_neighbor(node)
            });
    }

    /// Gets the start and end stations of the line.
    fn get_line_ends(&self, map: &Map) -> Vec<StationID> {
        let mut ends = Vec::new();
        let mut middles = Vec::new();

        if self
            .stations
            .is_empty()
            || self
                .edges
                .is_empty()
        {
            return self
                .stations
                .clone();
        }

        for id in &self.edges {
            let edge = map
                .get_edge(*id)
                .expect("invalid edge id");

            if !middles.contains(&edge.get_from()) {
                if let Some(i) = ends
                    .iter()
                    .position(|e| e == &edge.get_from())
                {
                    ends.remove(i);
                    middles.push(edge.get_from());
                } else {
                    ends.push(edge.get_from());
                }
            }

            if !middles.contains(&edge.get_to()) {
                if let Some(i) = ends
                    .iter()
                    .position(|e| e == &edge.get_to())
                {
                    ends.remove(i);
                    middles.push(edge.get_to());
                } else {
                    ends.push(edge.get_to());
                }
            }
        }

        ends
    }

    /// Draws the line around a station if this line has only a single station.
    pub fn draw(&self, map: &Map, canvas: &CanvasRenderingContext2d, state: CanvasState) {
        let stations = self.get_stations();

        if stations.len() == 1 {
            let station = map
                .get_station(stations[0])
                .expect("invalid station id on line");

            let mut width = state.drawn_square_size() / 10.0;
            if width < 1.0 {
                width = 1.0
            }

            canvas.set_line_width(width);
            canvas.set_global_alpha(1.0);
            canvas.set_stroke_style(&JsValue::from_str(&format!(
                "rgb({} {} {})",
                self.color
                    .0,
                self.color
                    .1,
                self.color
                    .2
            )));
            canvas.begin_path();

            let square_size = state.drawn_square_size();
            let (station_x, station_y) = station.get_canvas_pos(state);
            let offset = square_size / PI;

            canvas.move_to(station_x - offset, station_y);
            canvas.line_to(
                station_x - (square_size - offset),
                station_y,
            );

            canvas.move_to(station_x + offset, station_y);
            canvas.line_to(
                station_x + (square_size - offset),
                station_y,
            );

            canvas.stroke();
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        other.id == self.id
    }
}
