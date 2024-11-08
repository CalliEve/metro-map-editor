//! Contains the [`MapState`] and all its methods.

use leptos::{
    html::Canvas,
    *,
};
use web_sys::HtmlCanvasElement;

use super::CanvasState;
use crate::{
    algorithm::{
        drawing::redraw_canvas,
        recalculate_map,
        AlgorithmSettings,
    },
    models::{
        EdgeID,
        Map,
        SelectedLine,
        SelectedStation,
    },
};

/// The type of operation that is currently selected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionType {
    /// User wants to remove a [`Station`].
    ///
    /// [`Station`]: crate::models::Station
    RemoveStation,
    /// User wants to remove a [`Line`].
    ///
    /// [`Line`]: crate::models::Line
    RemoveLine,
    /// User wants to lock the location of a [`Station`].
    ///
    /// [`Station`]: crate::models::Station
    LockStation,
    /// User wants to unlock the location of a [`Station`].
    ///
    /// [`Station`]: crate::models::Station
    UnlockStation,
    /// User wants to lock the [`Edge`] between two [`Station`]s.
    ///
    /// [`Edge`]: crate::models::Edge
    /// [`Station`]: crate::models::Station
    LockEdge,
    /// User wants to unlock the [`Edge`] between two [`Station`]s.
    ///
    /// [`Edge`]: crate::models::Edge
    /// [`Station`]: crate::models::Station
    UnlockEdge,
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
    /// The type of action that is currently selected.
    selected_action: Option<ActionType>,
    /// The currently selected edges.
    selected_edges: Vec<EdgeID>,
    /// The state of the canvas.
    canvas: CanvasState,
    /// The settings for the algorithm.
    algorithm_settings: AlgorithmSettings,
    /// The last loaded map.
    last_loaded: Option<Map>,
    /// If the `last_loaded` map should be overlayed on the current map.
    original_overlay_enabled: bool,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map,
            selected_station: None,
            selected_line: None,
            selected_action: None,
            selected_edges: Vec::new(),
            canvas: CanvasState::default(),
            algorithm_settings: AlgorithmSettings::default(),
            last_loaded: None,
            original_overlay_enabled: false,
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

    /// A getter method for the selected action.
    pub fn get_selected_action(&self) -> Option<ActionType> {
        self.selected_action
    }

    /// A setter method for the selected action.
    pub fn set_selected_action(&mut self, operation: ActionType) {
        self.selected_action = Some(operation);
    }

    /// Set the selected action to None.
    pub fn clear_selected_action(&mut self) {
        self.selected_action = None;
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

    /// A getter method for the selected edges.
    pub fn get_selected_edges(&self) -> &[EdgeID] {
        &self.selected_edges
    }

    /// A setter method for the selected edges.
    pub fn set_selected_edges(&mut self, edges: Vec<EdgeID>) {
        self.clear_selected_edges();

        for id in &edges {
            self.map
                .get_mut_edge(*id)
                .expect("Edge to select does not exist.")
                .select();
        }
        self.selected_edges = edges;
    }

    /// Deselect all selected edges.
    pub fn clear_selected_edges(&mut self) {
        for id in &self.selected_edges {
            self.map
                .get_mut_edge(*id)
                .expect("Edge to deselect does not exist.")
                .deselect();
        }

        self.selected_edges = Vec::new();
    }

    /// A getter method for the last loaded map.
    pub fn get_last_loaded(&self) -> Option<&Map> {
        self.last_loaded
            .as_ref()
    }

    /// A setter method for the last loaded map.
    pub fn set_last_loaded(&mut self, map: Map) {
        self.last_loaded = Some(map);
    }

    /// A getter method for the original overlay enabled state.
    #[inline]
    pub fn is_original_overlay_enabled(&self) -> bool {
        self.original_overlay_enabled
    }

    /// A setter method for the original overlay enabled state.
    pub fn set_original_overlay_enabled(&mut self, enabled: bool) {
        self.original_overlay_enabled = enabled;
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

    /// Getter for the algorithm settings.
    #[inline]
    pub fn get_algorithm_settings(&self) -> AlgorithmSettings {
        self.algorithm_settings
    }

    /// Update the algorithm settings.
    pub fn update_algorithm_settings<F>(&mut self, f: F)
    where
        F: FnOnce(&mut AlgorithmSettings),
    {
        f(&mut self.algorithm_settings);
    }

    /// Setter for the algorithm settings.
    pub fn set_algorithm_settings(&mut self, settings: AlgorithmSettings) {
        self.algorithm_settings = settings;
    }

    /// Recalculate the x and y limits for the algorithm settings based on the
    /// current map.
    pub fn calculate_algorithm_settings(&mut self) {
        let mut x_limits = (i32::MAX, i32::MIN);
        let mut y_limits = (i32::MAX, i32::MIN);

        for station in self
            .map
            .get_stations()
        {
            let pos = station.get_pos();

            x_limits.0 = x_limits
                .0
                .min(pos.0);
            x_limits.1 = x_limits
                .1
                .max(pos.0);
            y_limits.0 = y_limits
                .0
                .min(pos.1);
            y_limits.1 = y_limits
                .1
                .max(pos.1);
        }

        self.algorithm_settings
            .grid_x_limits = (x_limits.0 - 2, x_limits.1 + 2);
        self.algorithm_settings
            .grid_y_limits = (y_limits.0 - 2, y_limits.1 + 2);
    }

    /// Run the full algorithm on the map. Returns true if successful.
    pub fn run_algorithm(&mut self) -> bool {
        self.calculate_algorithm_settings();
        let res = recalculate_map(self.algorithm_settings, &mut self.map);
        if let Err(e) = res {
            e.print_error();
            false
        } else {
            true
        }
    }
}
