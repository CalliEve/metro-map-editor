//! Contains the [`MapState`] and all its methods.

use leptos::{
    html::Canvas,
    *,
};

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
    /// The height and width of the current canvas.
    size: (u32, u32),
    /// The size of the map grid squares.
    square_size: u32,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map: Some(map),
            selected_station: None,
            selected_line: None,
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

    pub fn run_local_search(&mut self) {
        if let Some(map) = &mut self.map {
            for line in map.get_mut_lines() {
                line.calculate_line_edges();
            }
        }
    }
}
