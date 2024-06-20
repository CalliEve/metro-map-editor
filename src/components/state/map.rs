//! Contains the [`MapState`] and all its methods.

use leptos::{
    html::Canvas,
    *,
};

use crate::{
    algorithm::redraw_canvas,
    models::{
        Map,
        Station,
    },
};

/// Holds all the state of the current map and canvas.
#[derive(Clone, Debug)]
pub struct MapState {
    map: Option<Map>,
    selected_station: Option<Station>,
    size: (u32, u32),
    square_size: u32,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map: Some(map),
            selected_station: None,
            size: (300, 300),
            square_size: 30,
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
    pub fn get_selected_station(&self) -> Option<&Station> {
        self.selected_station
            .as_ref()
    }

    /// A setter method for the selected station.
    pub fn set_selected_station(&mut self, station: Station) {
        self.selected_station = Some(station);
    }

    /// Set the selected station to None.
    pub fn clear_selected_station(&mut self) {
        self.selected_station = None;
    }

    /// Returns if a station has been selected.
    pub fn has_selected_station(&self) -> bool {
        self.selected_station
            .is_some()
    }

    /// A getter method for the canvas size.
    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// A setter method for the canvas size.
    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    /// A getter method for the grid square size.
    pub fn get_square_size(&self) -> u32 {
        self.square_size
    }

    /// A setter method for the grid square size.
    pub fn set_square_size(&mut self, size: u32) {
        self.square_size = size;
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
}
