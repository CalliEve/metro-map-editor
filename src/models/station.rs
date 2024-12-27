//! Contains the [`Station`] struct and all its methods.

use std::{
    f64,
    fmt::Display,
};

use leptos::logging;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Edge,
    EdgeID,
    GridNode,
    Map,
};
use crate::{
    algorithm::drawing::{
        calc_label_pos,
        CanvasContext,
    },
    components::CanvasState,
    utils::IDManager,
};
/// An identifier for a station.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StationID(u64);

impl From<u64> for StationID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<StationID> for u64 {
    fn from(value: StationID) -> Self {
        value.0
    }
}

impl Display for StationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a metro station, including its grid position on the map, its id,
/// name and if the station should be greyed out when drawn to the canvas.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Station {
    /// Position of the station.
    pos: GridNode,
    /// The position of the station when it first was created.
    original_pos: GridNode,
    /// ID of the station.
    id: StationID,
    /// The station name.
    name: String,
    /// The edges that are connected to this station.
    edges: Vec<EdgeID>,
    /// Marks the location of the station as locked by the user in the
    /// algorithm.
    is_locked: bool,
    /// Marks the location of the station as settled in the algorithm.
    is_settled: bool,
    /// If the station is not actually real, but just a checkpoint.
    is_checkpoint: bool,
    /// The total cost of all the edges attached to the station, used in the
    /// local search algorithm.
    cost: f64,
}

impl Station {
    /// Create a new [`Station`] at the given grid coordinate.
    /// If id is None, the next sequential id is used.
    pub fn new(pos: GridNode, id: Option<StationID>) -> Self {
        if let Some(new_id) = id {
            IDManager::update_station_id(new_id);
        }

        Self {
            pos,
            original_pos: pos,
            id: id.unwrap_or_else(IDManager::next_station_id),
            name: String::new(),
            edges: Vec::new(),
            is_locked: false,
            is_settled: false,
            is_checkpoint: false,
            cost: 0.0,
        }
    }

    /// Create a new checkpoint [`Station`] at the given grid coordinate.
    /// If id is None, the next sequential id is used.
    pub fn new_checkpoint(pos: GridNode, id: Option<StationID>) -> Self {
        if let Some(new_id) = id {
            IDManager::update_station_id(new_id);
        }

        Self {
            pos,
            original_pos: pos,
            id: id.unwrap_or_else(IDManager::next_station_id),
            name: String::new(),
            edges: Vec::new(),
            is_locked: false,
            is_settled: false,
            is_checkpoint: true,
            cost: 0.0,
        }
    }

    /// A getter for the id.
    #[inline]
    pub fn get_id(&self) -> StationID {
        self.id
    }

    /// A getter for the grid position.
    #[inline]
    pub fn get_pos(&self) -> GridNode {
        self.pos
    }

    /// A setter for the grid position of the station.
    pub fn set_pos(&mut self, pos: GridNode) {
        self.pos = pos;
    }

    /// A getter for the original grid position.
    #[inline]
    pub fn get_original_pos(&self) -> GridNode {
        self.original_pos
    }

    /// A setter for the original grid position.
    /// This is used when the station is moved manually.
    pub fn set_original_pos(&mut self, pos: GridNode) {
        self.original_pos = pos;
    }

    /// The location of the station on the canvas, given the size of a grid
    /// square.
    pub fn get_canvas_pos(&self, state: CanvasState) -> (f64, f64) {
        self.get_pos()
            .to_canvas_pos(state)
    }

    /// A setter for the name.
    pub fn set_name(&mut self, name: &impl ToString) {
        self.name = name.to_string();
    }

    /// A getter for the name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Lock the position of the station.
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlock the position of the station.
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Settle the station onto the given grid node.
    pub fn settle(&mut self, pos: GridNode) {
        self.set_pos(pos);
        self.is_settled = true;
    }

    /// Unsettle the station.
    pub fn unsettle(&mut self) {
        self.is_settled = false;
    }

    /// Check if the station is settled.
    #[inline]
    pub fn is_settled(&self) -> bool {
        self.is_settled || self.is_locked()
    }

    /// Check if the station is locked.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Check if the station is a checkpoint.
    #[inline]
    pub fn is_checkpoint(&self) -> bool {
        self.is_checkpoint
    }

    /// Get the cost of the station.
    #[inline]
    pub fn get_cost(&self) -> f64 {
        self.cost
    }

    /// Set the cost of the station.
    pub fn set_cost(&mut self, cost: f64) {
        self.cost = cost;
    }

    /// Add to the cost of the station.
    pub fn add_cost(&mut self, cost: f64) {
        self.cost += cost;
    }

    /// Add an edge to the station.
    pub fn add_edge(&mut self, edge: EdgeID) {
        if self
            .edges
            .contains(&edge)
        {
            return;
        }

        self.edges
            .push(edge);
    }

    /// Remove an edge from the station.
    pub fn remove_edge(&mut self, edge: EdgeID) {
        self.edges
            .retain(|e| *e != edge);
    }

    /// Get the edges connected to the station.
    pub fn get_edges(&self) -> &[EdgeID] {
        &self.edges
    }

    /// Clear all edges from the station.
    pub fn clear_edges(&mut self) {
        self.edges
            .clear();
    }

    /// Check if the station has a locked edge.
    pub fn has_locked_edge(&self, map: &Map) -> bool {
        self.get_edges()
            .iter()
            .filter_map(|edge_id| map.get_edge(*edge_id))
            .any(Edge::is_locked)
    }

    #[allow(dead_code)]
    pub fn print_info(&self) {
        logging::log!(
            "Station: {}({}) at {:?} with edges [{:?}]",
            self.get_name(),
            self.get_id(),
            self.get_pos(),
            self.get_edges()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    /// Draw the station to the given canvas.
    pub fn draw(&self, canvas: &CanvasContext<'_>, state: CanvasState, base_alpha: f64) {
        if !state.is_on_canvas(self.get_pos()) {
            return;
        }

        let canvas_pos = self.get_canvas_pos(state);

        let mut width = state.drawn_square_size() / 10.0 + 1.0;
        if width < 2.0 {
            width = 2.0;
        }
        let radius = state.drawn_square_size() / 3.0;

        canvas.set_line_width(width);
        canvas.set_global_alpha(1.0 * base_alpha);
        canvas.set_stroke_style_str("black");

        if self.is_checkpoint() {
            canvas.begin_path();
            canvas.move_to(canvas_pos.0, canvas_pos.1 - radius);
            canvas.line_to(canvas_pos.0 + radius, canvas_pos.1);
            canvas.line_to(canvas_pos.0, canvas_pos.1 + radius);
            canvas.line_to(canvas_pos.0 - radius, canvas_pos.1);
            canvas.line_to(canvas_pos.0, canvas_pos.1 - radius);
            canvas.line_to(
                canvas_pos.0 + width,
                canvas_pos.1 - radius + width,
            );
            canvas.stroke();
        } else {
            canvas.begin_path();
            canvas
                .arc(
                    canvas_pos.0,
                    canvas_pos.1,
                    radius,
                    0.0,
                    2.0 * f64::consts::PI,
                )
                .unwrap();
            canvas.stroke();
        }

        if self.is_locked() {
            let locked_label_pos = calc_label_pos(state, canvas_pos, None, None)[0]; // FIXME: Check for occupancy

            canvas.set_stroke_style_str("grey");
            canvas.begin_path();
            canvas
                .arc(
                    locked_label_pos.0,
                    locked_label_pos.1,
                    state.drawn_square_size() / 3.0 / 5.0,
                    0.0,
                    2.0 * f64::consts::PI,
                )
                .unwrap();
            canvas.set_fill_style_str("grey");
            canvas.fill();
            canvas.stroke();
        }
    }
}

impl PartialEq for Station {
    fn eq(&self, other: &Station) -> bool {
        other.id == self.id
    }
}
