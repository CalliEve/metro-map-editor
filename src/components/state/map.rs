//! Contains the [`MapState`] and all its methods.

use leptos::{
    html::Canvas,
    *,
};

use super::CanvasState;
use crate::{
    algorithm::redraw_canvas,
    models::{
        Map,
        SelectedLine,
        SelectedStation,
    },
};

/// Holds all the state of the current map and canvas.
#[derive(Clone, Debug)]
pub struct MapState {
    /// The current state of the map.
    map: Option<Map>,
    /// The currently selected [`Station`].
    selected_station: Option<SelectedStation>,
    /// The currently selected [`Line`].
    selected_line: Option<SelectedLine>,
    /// The state of the canvas.
    canvas: CanvasState,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map: Some(map),
            selected_station: None,
            selected_line: None,
            canvas: CanvasState::default(),
        }
    }

    /// A getter method for the [`Map`].
    pub fn get_map(&self) -> Option<&Map> {
        self.map
            .as_ref()
    }

    /// A setter method for the [`Map`].
    pub fn set_map(&mut self, map: Map) {
        self.map = Some(map);
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
                .expect("should be loaded now"),
            self,
        );
    }

    pub fn run_local_search(&mut self) {
        if let Some(map) = &mut self.map {
            for line in map.get_mut_lines() {
                line.calculate_line_edges();
            }
        }
    }
}
