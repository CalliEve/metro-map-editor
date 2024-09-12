//! Contains the [`MapState`] and all its methods.

use leptos::{
    html::Canvas,
    *,
};
use web_sys::HtmlCanvasElement;

use super::CanvasState;
use crate::{
    algorithm::drawing::redraw_canvas,
    models::{
        Map,
        SelectedLine,
        SelectedStation,
    },
};

/// The type of remove operation that is currently selected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RemoveType {
    Station,
    Line,
}

/// Holds all the state of the current [`Map`], canvas and any potentially
/// selected objects.
#[derive(Clone, Debug)]
pub struct MapState {
    /// The current state of the map.
    map: Map,
    /// The currently selected [`crate::models::Station`].
    selected_station: Option<SelectedStation>,
    /// The currently selected [`crate::models::Line`].
    selected_line: Option<SelectedLine>,
    /// The type of remove operation that is currently selected.
    selected_remove: Option<RemoveType>,
    /// The state of the canvas.
    canvas: CanvasState,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map,
            selected_station: None,
            selected_line: None,
            selected_remove: None,
            canvas: CanvasState::default(),
        }
    }

    /// A getter method for the [`Map`].
    pub fn get_map(&self) -> &Map {
        &self.map
    }

    /// A mutable getter method for the [`Map`].
    pub fn get_mut_map(&mut self) -> &mut Map {
        &mut self.map
    }

    /// A setter method for the [`Map`].
    pub fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    /// A getter method for the selected station.
    pub fn get_selected_station(&self) -> Option<&SelectedStation> {
        self.selected_station
            .as_ref()
    }

    /// A setter method for the selected station.
    pub fn set_selected_station(&mut self, station: SelectedStation) {
        self.selected_station = Some(station);
    }

    /// Set the selected station to None.
    pub fn clear_selected_station(&mut self) {
        self.selected_station = None;
    }

    /// A getter method for the selected remove operation.
    pub fn get_selected_remove(&self) -> Option<RemoveType> {
        self.selected_remove
    }

    /// A setter method for the selected remove operation.
    pub fn set_selected_remove(&mut self, operation: RemoveType) {
        self.selected_remove = Some(operation);
    }

    /// Set the selected remove operation to None.
    pub fn clear_selected_remove(&mut self) {
        self.selected_remove = None;
    }

    /// A mutable getter method for the selected line.
    pub fn get_mut_selected_line(&mut self) -> Option<&mut SelectedLine> {
        self.selected_line
            .as_mut()
    }

    /// A getter method for the selected line.
    pub fn get_selected_line(&self) -> Option<&SelectedLine> {
        self.selected_line
            .as_ref()
    }

    /// A setter method for the selected line.
    pub fn set_selected_line(&mut self, line: SelectedLine) {
        self.selected_line = Some(line);
    }

    /// Set the selected line to None.
    pub fn clear_selected_line(&mut self) {
        self.selected_line = None;
    }

    /// Returns if anything has been selected.
    pub fn has_any_selected(&self) -> bool {
        self.selected_station
            .is_some()
            || self
                .selected_line
                .is_some()
    }

    /// A getter method for the state of the canvas.
    #[inline]
    pub fn get_canvas_state(&self) -> CanvasState {
        self.canvas
    }

    /// Update the state of the canvas.
    pub fn update_canvas_state<F>(&mut self, f: F)
    where
        F: FnOnce(&mut CanvasState),
    {
        f(&mut self.canvas);
    }

    /// Draw the current state to the provided canvas.
    pub fn draw_to_canvas(&self, canvas_ref: &NodeRef<Canvas>) {
        redraw_canvas(
            &canvas_ref
                .get()
                .expect("should be loaded now") as &HtmlCanvasElement,
            self,
        );
    }

    /// Run the local search algorithm on the map.
    pub fn run_local_search(&mut self) {
        let map_clone = self
            .map
            .clone();

        for edge in self
            .map
            .get_mut_edges()
        {
            edge.calculate_nodes(&map_clone);
        }
    }
}
