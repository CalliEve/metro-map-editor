//! Contains the [`Line`] struct and all its methods.

use std::{
    cmp::Ordering,
    f64::consts::PI,
    sync::atomic::{
        AtomicU32,
        Ordering as AtomicOrdering,
    },
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{
    Drawable,
    GridNode,
    Station,
};
use crate::{
    algorithm::{
        draw_edge,
        run_a_star,
    },
    components::CanvasState,
};

/// Next generated sequential identifier for a new line.
static LINE_ID: AtomicU32 = AtomicU32::new(1);

/// Represents a metro line, including its stations, name and color.
#[derive(Clone, Debug)]
pub struct Line {
    /// ID of the line.
    id: String,
    /// Name of the line.
    name: String,
    /// Color of the line.
    color: (u8, u8, u8),
    /// All stations the line visits.
    stations: Vec<(Station, Vec<GridNode>, Option<Station>)>,
}

impl Line {
    /// Create a new [`Line`] with the stations it visits and an identifier.
    /// Color and name are set to default values.
    pub fn new(stations: Vec<Station>, id: Option<String>) -> Self {
        let mut l = Self {
            stations: stations
                .clone()
                .into_iter()
                .zip(
                    stations
                        .clone()
                        .into_iter()
                        .skip(1),
                )
                .map(|(s, a)| (s, Vec::new(), Some(a)))
                .collect(),
            id: id.unwrap_or_else(|| {
                LINE_ID
                    .fetch_add(1, AtomicOrdering::SeqCst)
                    .to_string()
            }),
            color: (0, 0, 0),
            name: String::new(),
        };

        if let Some(s) = stations
            .last()
            .cloned()
        {
            l.stations
                .push((s, Vec::new(), None));
        }

        l
    }

    /// A getter method for the stations the line visits.
    pub fn get_stations(&self) -> Vec<&Station> {
        self.stations
            .iter()
            .map(|(s, ..)| s)
            .collect()
    }

    /// A mutable getter method for the stations the line visits.
    pub fn get_mut_stations(&mut self) -> Vec<&mut Station> {
        self.stations
            .iter_mut()
            .map(|(s, ..)| s)
            .collect()
    }

    /// Add a station. It will be inserted before the before station and after
    /// the after station, Or at the end of the line. If before isn't in the
    /// line yet, it will add both.
    pub fn add_station(
        &mut self,
        station: Station,
        before: Option<&Station>,
        after: Option<&Station>,
    ) {
        if let (Some(before_station), Some(after_station)) = (before, after) {
            if let Some(index) = self
                .stations
                .iter()
                .position(|s| {
                    &s.0 == before_station
                        && s.2
                            .as_ref()
                            == after
                })
            {
                self.stations
                    .remove(index);
                self.stations
                    .insert(
                        index,
                        (
                            station.clone(),
                            Vec::new(),
                            Some(before_station.clone()),
                        ),
                    );
                self.stations
                    .insert(
                        index,
                        (
                            after_station.clone(),
                            Vec::new(),
                            Some(station),
                        ),
                    );
                return;
            }
        }

        if let Some(index) = before.and_then(|a| {
            self.stations
                .iter()
                .position(|s| &s.0 == a)
        }) {
            // found after and will insert station after after
            self.stations
                .insert(
                    index,
                    (station, Vec::new(), before.cloned()),
                );
            return;
        } else if let Some(b) = before.cloned() {
            // before exists but not found, so inserting it at the end
            self.stations
                .push((station, Vec::new(), Some(b.clone())));
            self.stations
                .push((b, Vec::new(), None));
        } else {
            self.stations
                .push((station, Vec::new(), None));
        }
    }

    /// A setter for the station's color.
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    /// A getter for the station's color.
    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    /// A setter for the station's name.
    pub fn set_name(&mut self, name: &impl ToString) {
        self.name = name.to_string();
    }

    /// A getter for the station's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// A getter for the station id.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Get a list of neighbors of the given station.
    pub fn get_station_neighbors(&self, station: &Station) -> (Vec<Station>, Vec<Station>) {
        let (mut before, mut after) = (Vec::new(), Vec::new());

        for edge in &self.stations {
            if &edge.0 == station {
                if let Some(after_station) = edge
                    .2
                    .clone()
                {
                    after.push(after_station);
                }
            } else if edge
                .2
                .as_ref()
                .is_some_and(|a| a == station)
            {
                before.push(
                    edge.0
                        .clone(),
                );
            }
        }

        (before, after)
    }

    /// Gets the stations on either side of the position on this line.
    pub fn get_edge_stations(&self, node: GridNode) -> (Option<Station>, Option<Station>) {
        for edge in &self.stations {
            if edge
                .1
                .contains(&node)
            {
                let before = Some(
                    edge.0
                        .clone(),
                );
                let after = edge
                    .2
                    .clone();
                return (before, after);
            }
        }

        (None, None)
    }

    /// Returns true if the line goes through the given grid node.
    pub fn visits_node(&self, node: GridNode) -> bool {
        if self
            .stations
            .len()
            > 1
        {
            self.stations
                .iter()
                .any(|(s, steps, _)| s.get_pos() == node || steps.contains(&node))
        } else if let Some(s) = self
            .stations
            .first()
        {
            s.0.get_pos() == node
                || s.0
                    .get_pos()
                    .get_neighbors()
                    .contains(&node)
        } else {
            unreachable!("line can't have 0 stations")
        }
    }

    /// Recalculates the edges between the stations.
    pub fn calculate_line_edges(&mut self) {
        for (from, edges, to) in self
            .stations
            .iter_mut()
            .filter(|(_, _, to)| to.is_some())
        {
            *edges = run_a_star(
                from.get_pos(),
                to.as_ref()
                    .unwrap()
                    .get_pos(),
            );
        }
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &CanvasRenderingContext2d, state: CanvasState) {
        let stations = self.get_stations();

        canvas.set_line_width(3.0);
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

        match stations
            .len()
            .cmp(&1)
        {
            // Draw a line between each two sequential stations on the line.
            Ordering::Greater => {
                for (start_station, steps, end_station) in self
                    .stations
                    .iter()
                    .filter(|(_, _, to)| to.is_some())
                {
                    draw_edge(
                        start_station.get_pos(),
                        end_station
                            .as_ref()
                            .unwrap()
                            .get_pos(),
                        &steps,
                        canvas,
                        state,
                    );
                }
            },
            // Add two horizontal lines to the single station, showing its a lone station on the
            // line.
            Ordering::Equal => {
                let square_size = state.drawn_square_size();
                let (station_x, station_y) = stations[0].get_canvas_pos(state);
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
            },
            // Empty line means nothing to draw
            Ordering::Less => {},
        }

        canvas.stroke();
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        other.id == self.id
    }
}
