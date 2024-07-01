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
    stations: Vec<(Station, Vec<GridNode>)>,
}

impl Line {
    /// Create a new [`Line`] with the stations it visits and an identifier.
    /// Color and name are set to default values.
    pub fn new(stations: Vec<Station>, id: Option<String>) -> Self {
        Self {
            stations: stations
                .into_iter()
                .map(|s| (s, Vec::new()))
                .collect(),
            id: id.unwrap_or_else(|| {
                LINE_ID
                    .fetch_add(1, AtomicOrdering::SeqCst)
                    .to_string()
            }),
            color: (0, 0, 0),
            name: String::new(),
        }
    }

    /// A getter method for the stations the line visits.
    pub fn get_stations(&self) -> Vec<&Station> {
        self.stations
            .iter()
            .map(|(s, _)| s)
            .collect()
    }

    /// A mutable getter method for the stations the line visits.
    pub fn get_mut_stations(&mut self) -> Vec<&mut Station> {
        self.stations
            .iter_mut()
            .map(|(s, _)| s)
            .collect()
    }

    /// Add a station after the after station, or at the end of the line.
    /// If after isn't in the line yet, it will add both.
    pub fn add_station(&mut self, station: Station, after: Option<&Station>) {
        if let Some(index) = after.and_then(|a| {
            self.stations
                .iter()
                .position(|s| &s.0 == a)
        }) {
            // found after and will insert station after after
            self.stations
                .insert(index + 1, (station, Vec::new()));
            return;
        } else if let Some(a) = after.cloned() {
            // after exists but not found, so inserting it at the end
            self.stations
                .push((a, Vec::new()));
        }

        self.stations
            .push((station, Vec::new()));
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

    /// Gets the stations on either side of the position on this line.
    ///
    /// Note: In case there is a station on this node, the stations before and
    /// after that station are returned.
    pub fn get_neighbors(&self, node: GridNode) -> (Option<Station>, Option<Station>) {
        let mut found = false;
        let mut before = None;
        let mut after = None;

        let mut iterator = self
            .stations
            .iter();
        while let Some(line_station) = iterator.next() {
            // station is located at the node; grab the station before it as the before.
            if line_station
                .0
                .get_pos()
                == node
            {
                found = true;
                after = iterator
                    .next()
                    .cloned()
                    .map(|s| s.0);
                break;
            }
            before = Some(&line_station.0);

            // node is in the steps; grab the current station and the one after it.
            if line_station
                .1
                .contains(&node)
            {
                found = true;
                after = iterator
                    .next()
                    .cloned()
                    .map(|s| s.0);
                break;
            }
        }

        if found {
            (before.cloned(), after)
        } else {
            (None, None)
        }
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
                .any(|(s, steps)| s.get_pos() == node || steps.contains(&node))
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
        let to_stations = self
            .stations
            .iter()
            .map(|(s, _)| s.get_pos())
            .skip(1)
            .collect::<Vec<GridNode>>();

        for ((from, edges), to) in self
            .stations
            .iter_mut()
            .zip(to_stations)
        {
            *edges = run_a_star(from.get_pos(), to);
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
                for (start_station, end_station) in self
                    .stations
                    .iter()
                    .zip(
                        stations
                            .iter()
                            .skip(1),
                    )
                {
                    draw_edge(
                        start_station
                            .0
                            .get_pos(),
                        end_station.get_pos(),
                        &start_station.1,
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
