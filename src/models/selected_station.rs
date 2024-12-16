//! Contains the [`SelectedStation`] struct and all its methods.

use super::{
    GridNode,
    Map,
    Station,
    StationID,
};
use crate::{
    algorithm::{
        drawing::{
            draw_edge,
            CanvasContext,
        },
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
    /// The position the user is moving the station from.
    moved_from: Option<GridNode>,
}

impl SelectedStation {
    /// Select a station.
    pub fn new(station: Station) -> Self {
        Self {
            moved_from: Some(station.get_pos()),
            station,
            before_after: (Vec::new(), Vec::new()),
        }
    }

    /// Select a newly created station.
    pub fn new_station() -> Self {
        let station = Station::new((i32::MIN, i32::MIN).into(), None);
        Self {
            station,
            before_after: (Vec::new(), Vec::new()),
            moved_from: None,
        }
    }

    /// Select a newly created checkpoint.
    pub fn new_checkpoint() -> Self {
        let station = Station::new_checkpoint((i32::MIN, i32::MIN).into(), None);
        Self {
            station,
            before_after: (Vec::new(), Vec::new()),
            moved_from: None,
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
    pub fn add_before(&mut self, before: StationID) {
        self.before_after
            .0
            .push(before);
    }

    /// Add a station that came after the station that was grabbed.
    pub fn add_after(&mut self, after: StationID) {
        self.before_after
            .1
            .push(after);
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
    pub fn deselect(self) -> Station {
        self.station
    }

    /// If the station has been moved to a new position.
    pub fn has_moved(&self) -> bool {
        self.moved_from
            != Some(
                self.station
                    .get_pos(),
            )
    }

    /// If the station is newly created.
    pub fn is_new(&self) -> bool {
        self.moved_from
            .is_none()
    }

    /// Get the position the station is being dragged from.
    pub fn get_original_position(&self) -> Option<GridNode> {
        self.moved_from
    }

    /// Draw the selected station to the given canvas.
    #[allow(clippy::too_many_lines)] // This function is long but it's mostly drawing code that can't be split up
                                     // easily.
    pub fn draw(
        &self,
        map: &Map,
        canvas: &CanvasContext<'_>,
        state: CanvasState,
        all_selected: &[Self],
    ) {
        let mut station = self
            .station
            .clone();
        station.unlock();
        let canvas_pos = station.get_canvas_pos(state);

        let mut selected_width = state.drawn_square_size() / 3.5;
        if selected_width < 2.5 {
            selected_width = 2.5;
        }

        // draw selected highlight
        canvas.set_line_width(selected_width);
        canvas.set_global_alpha(0.2);
        canvas.set_stroke_style_str("darkblue");
        canvas.begin_path();
        canvas
            .arc(
                canvas_pos.0,
                canvas_pos.1,
                state.drawn_square_size() / 3.0,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
        canvas.stroke();

        if !self.has_moved() {
            return;
        }

        // draw station
        station.draw(canvas, state, 0.5);

        // draw edges to adjacent stations
        let mut edge_width = state.drawn_square_size() / 10.0 + 0.5;
        if edge_width < 1.0 {
            edge_width = 1.0;
        }

        canvas.set_line_width(edge_width);
        canvas.set_stroke_style_str("black");
        canvas.begin_path();

        for before_id in self
            .get_before_after()
            .0
        {
            if all_selected
                .iter()
                .any(|d| {
                    d.get_station()
                        .get_id()
                        == *before_id
                })
            {
                continue;
            }

            let before = map
                .get_station(*before_id)
                .expect("invalid id");
            draw_edge(
                before.get_pos(),
                station.get_pos(),
                &run_a_star(before.get_pos(), station.get_pos()),
                canvas,
                state,
                0.0,
            );
        }

        for after_id in self
            .get_before_after()
            .1
        {
            let after = if let Some(after) = all_selected
                .iter()
                .find(|d| {
                    d.get_station()
                        .get_id()
                        == *after_id
                }) {
                after.get_station()
            } else {
                map.get_station(*after_id)
                    .expect("invalid id")
            };

            draw_edge(
                station.get_pos(),
                after.get_pos(),
                &run_a_star(station.get_pos(), after.get_pos()),
                canvas,
                state,
                0.0,
            );
        }

        canvas.stroke();
    }
}

impl PartialEq for SelectedStation {
    fn eq(&self, other: &SelectedStation) -> bool {
        self.station == other.station
    }
}
