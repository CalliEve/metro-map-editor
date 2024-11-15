//! Contains the [`MapState`] and all its methods.

use std::collections::HashMap;

use leptos::{
    html::Canvas,
    *,
};
use web_sys::HtmlCanvasElement;

use super::CanvasState;
use crate::{
    algorithm::{
        drawing::redraw_canvas,
        log_print,
        recalculate_map,
        AlgorithmSettings,
        LogType,
        OccupiedNodes,
    },
    models::{
        EdgeID,
        GridNode,
        Line,
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
    /// User wants to lock a [`Station`] or [`Edge`].
    ///
    /// [`Station`]: crate::models::Station
    Lock,
    /// User wants to unlock a [`Station`] or [`Edge`].
    ///
    /// [`Station`]: crate::models::Station
    Unlock,
}

/// Holds all the state of the current [`Map`], canvas and any potentially
/// selected objects.
#[derive(Clone, Debug)]
pub struct MapState {
    /// The current state of the map.
    map: Map,
    /// The currently selected [`crate::models::Station`]s.
    selected_stations: Vec<SelectedStation>,
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
    /// The point the user is dragging the map from and if they're dragging the
    /// map as a whole, or a station and/or edge.
    drag_offset: Option<((f64, f64), bool)>,
    /// The point the user is selecting a part of the map with a box-select
    /// from.
    box_select: Option<((f64, f64), (f64, f64))>,
}

impl MapState {
    /// Create a new [`MapState`] using the given [`Map`]. Sets all other state
    /// properties to default values.
    pub fn new(map: Map) -> Self {
        Self {
            map,
            selected_stations: Vec::new(),
            selected_line: None,
            selected_action: None,
            selected_edges: Vec::new(),
            canvas: CanvasState::default(),
            algorithm_settings: AlgorithmSettings::default(),
            last_loaded: None,
            original_overlay_enabled: false,
            drag_offset: None,
            box_select: None,
        }
    }

    /// Clear all selections.
    pub fn clear_all_selections(&mut self) {
        self.clear_selected_stations();
        self.clear_selected_line();
        self.clear_selected_action();
        self.clear_selected_edges();
        self.clear_box_select();
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
    pub fn get_selected_stations(&self) -> &[SelectedStation] {
        &self.selected_stations
    }

    /// A mutable getter method for the selected station.
    pub fn get_mut_selected_stations(&mut self) -> &mut [SelectedStation] {
        &mut self.selected_stations
    }

    /// A setter method for the selected station.
    pub fn select_station(&mut self, station: SelectedStation) {
        self.selected_stations
            .push(station);
    }

    /// A setter method for the selected stations.
    pub fn set_selected_stations(&mut self, stations: Vec<SelectedStation>) {
        self.selected_stations = stations;
    }

    /// Deselect all stations.
    pub fn clear_selected_stations(&mut self) {
        self.selected_stations = Vec::new();
    }

    /// If the given node is on a selected object.
    pub fn is_on_selected_object(&self, node: GridNode) -> bool {
        self.get_selected_stations()
            .iter()
            .any(|station| {
                station
                    .get_station()
                    .get_pos()
                    == node
            })
            || self
                .get_selected_edges()
                .iter()
                .any(|id| {
                    self.map
                        .get_edge(*id)
                        .expect("Selected edge does not exist.")
                        .visits_node(&self.map, node)
                })
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

    /// Add a new edge to the selected edges.
    pub fn select_edge(&mut self, edge: EdgeID) {
        self.map
            .get_mut_edge(edge)
            .expect("Edge to select does not exist.")
            .select();

        self.selected_edges
            .push(edge);
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

    /// Lock all selected edges and stations.
    pub fn lock_selected(&mut self) {
        for id in &self.selected_edges {
            self.map
                .get_mut_edge(*id)
                .expect("Edge to lock does not exist.")
                .lock();
        }
        for station in &self.selected_stations {
            if let Some(map_station) = self
                .map
                .get_mut_station(
                    station
                        .get_station()
                        .get_id(),
                )
            {
                map_station.lock();
            }
        }
    }

    /// Unlock all selected edges and stations.
    pub fn unlock_selected(&mut self) {
        for id in &self.selected_edges {
            self.map
                .get_mut_edge(*id)
                .expect("Edge to unlock does not exist.")
                .unlock();
        }
        for station in &self.selected_stations {
            if let Some(map_station) = self
                .map
                .get_mut_station(
                    station
                        .get_station()
                        .get_id(),
                )
            {
                map_station.unlock();
            }
        }
    }

    /// A getter method for the drag offset.
    pub fn get_drag_offset(&self) -> Option<((f64, f64), bool)> {
        self.drag_offset
    }

    /// A setter method for the drag offset.
    pub fn set_drag_offset(&mut self, offset: Option<((f64, f64), bool)>) {
        self.drag_offset = offset;
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

    /// Getter for the box select.
    #[inline]
    pub fn get_box_select(&self) -> Option<((f64, f64), (f64, f64))> {
        self.box_select
    }

    /// Set the box selection start.
    pub fn set_box_select_start(&mut self, start: (f64, f64)) {
        self.box_select = Some((start, start));
    }

    /// Update the box selection end.
    pub fn update_box_select_end(&mut self, end: (f64, f64)) {
        if let Some((start, _)) = self.box_select {
            self.box_select = Some((start, end));
        }
    }

    /// Clear the box selection.
    pub fn clear_box_select(&mut self) {
        self.box_select = None;
    }

    /// Make a map from the selected edges and stations.
    pub fn get_selected_partial_map(&self) -> (Map, OccupiedNodes) {
        let mut map = Map::new();
        let mut stations = Vec::new();
        let mut lines = Vec::new();
        let mut occupied = HashMap::new();

        for edge_id in self.get_selected_edges() {
            let edge = self
                .map
                .get_edge(*edge_id)
                .expect("Selected edge does not exist.");
            stations.push(edge.get_from());
            stations.push(edge.get_to());
            lines.append(
                &mut edge
                    .get_lines()
                    .to_vec(),
            );
        }

        stations.sort();
        stations.dedup();
        lines.sort();
        lines.dedup();

        for station_id in &stations {
            let mut station = self
                .map
                .get_station(*station_id)
                .expect("Station of selected edge does not exist.")
                .clone();

            let selected = self
                .get_selected_stations()
                .iter()
                .find(|s| {
                    s.get_station()
                        .get_id()
                        == *station_id
                });
            if selected.is_none()
                || station
                    .get_edges()
                    .iter()
                    .any(|e| {
                        !self
                            .selected_edges
                            .contains(e)
                    })
            {
                station.lock();
            }

            station.clear_edges();
            map.add_station(station.clone());
        }

        for station in self
            .map
            .get_stations()
        {
            if stations.contains(&station.get_id()) {
                continue;
            }

            occupied.insert(
                station.get_pos(),
                station
                    .get_id()
                    .into(),
            );
        }

        for line_id in &lines {
            map.add_line(Line::new(Some(*line_id)));
        }

        for edge_id in &self.selected_edges {
            let edge = self
                .get_map()
                .get_edge(*edge_id)
                .expect("Selected edge does not exist.");

            let new_edge_id = map.get_edge_id_between(edge.get_from(), edge.get_to());

            for line_id in edge.get_lines() {
                let mut line = map
                    .get_line(*line_id)
                    .expect("Line of selected edge does not exist.")
                    .clone();

                line.add_edge(new_edge_id, &mut map);
                map.add_line(line);
            }
        }

        for edge in self
            .map
            .get_edges()
        {
            if self
                .selected_edges
                .contains(&edge.get_id())
            {
                continue;
            }

            for node in edge.get_nodes() {
                occupied.insert(
                    *node,
                    edge.get_id()
                        .into(),
                );
            }
        }

        log_print(
            self.get_algorithm_settings(),
            &format!(
                "Created partial map with {} stations, {} edges and {} lines.",
                map.get_stations()
                    .len(),
                map.get_edges()
                    .len(),
                map.get_lines()
                    .len(),
            ),
            LogType::Debug,
        );

        (map, occupied)
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
    pub fn run_algorithm(&mut self, already_occupied: Option<OccupiedNodes>) -> bool {
        self.calculate_algorithm_settings();
        let res = recalculate_map(
            self.algorithm_settings,
            &mut self.map,
            already_occupied.unwrap_or_default(),
        );
        if let Err(e) = res {
            e.print_error();
            false
        } else {
            true
        }
    }
}
