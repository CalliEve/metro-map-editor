//! Contains the [`SelectedStation`] struct and all its methods.

use wasm_bindgen::JsValue;

use super::{
    GridNode,
    Map,
    Station,
    StationID,
};
use crate::{
    algorithm::{
        draw_edge,
        run_a_star,
    },
    components::CanvasState,
};

/// Holds information about the currently selected [`Station`].
#[derive(Debug, Clone)]
pub struct SelectedStation {
    /// The selected station.
    station: Station,
    /// The stations before and after the station that was grabbed if
    /// applicable.
    before_after: (Vec<StationID>, Vec<StationID>),
}

impl SelectedStation {
    /// Select a station.
    pub fn new(mut station: Station) -> Self {
        station.set_is_ghost(true);
        Self {
            station,
            before_after: (Vec::new(), Vec::new()),
        }
    }

    /// Select a newly created station.
    pub fn new_station() -> Self {
        let mut station = Station::new((-1, -1).into(), None);
        station.set_is_ghost(true);
        Self {
            station,
            before_after: (Vec::new(), Vec::new()),
        }
    }

    /// Get the station that is currently selected.
    pub fn get_station(&self) -> &Station {
        &self.station
    }

    /// Get the stations before and after the station that was grabbed.
    pub fn get_before_after(&self) -> (&[StationID], &[StationID]) {
        let (before, after) = &self.before_after;
        (before.as_ref(), after.as_ref())
    }

    /// Add a station that came before the station that was grabbed.
    pub fn add_before(&mut self, mut before: Vec<StationID>) {
        self.before_after
            .0
            .append(&mut before);
    }

    /// Add a station that came after the station that was grabbed.
    pub fn add_after(&mut self, mut after: Vec<StationID>) {
        self.before_after
            .1
            .append(&mut after);
    }

    /// Update the current grid position of the station.
    pub fn update_pos(&mut self, new_pos: GridNode) {
        self.station
            .set_pos(new_pos);
    }

    /// A getter for the current grid position of the station.
    pub fn get_pos(&self) -> GridNode {
        self.station
            .get_pos()
    }

    /// Deselects the station and returns it.
    pub fn deselect(mut self) -> Station {
        self.station
            .set_is_ghost(false);
        self.station
    }

    /// Draw the selected station to the given canvas.
    pub fn draw(&self, map: &Map, canvas: &web_sys::CanvasRenderingContext2d, state: CanvasState) {
        self.station
            .draw(canvas, state);

        canvas.set_line_width(3.0);
        canvas.set_stroke_style(&JsValue::from_str("black"));
        canvas.set_global_alpha(0.5);
        canvas.begin_path();

        for before_id in self
            .get_before_after()
            .0
        {
            let before = map
                .get_station(*before_id)
                .expect("invalid id");
            draw_edge(
                before.get_pos(),
                self.station
                    .get_pos(),
                &run_a_star(
                    before.get_pos(),
                    self.station
                        .get_pos(),
                ),
                canvas,
                state,
                0.0,
            );
        }

        for after_id in self
            .get_before_after()
            .1
        {
            let after = map
                .get_station(*after_id)
                .expect("invalid id");
            draw_edge(
                self.station
                    .get_pos(),
                after.get_pos(),
                &run_a_star(
                    self.station
                        .get_pos(),
                    after.get_pos(),
                ),
                canvas,
                state,
                0.0,
            );
        }

        canvas.stroke();
    }
}
