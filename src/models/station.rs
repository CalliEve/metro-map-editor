//! Contains the [`Station`] struct and all its methods.

use std::{
    f64,
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
};

use super::GridNode;
use crate::{
    algorithm::drawing::CanvasContext,
    components::CanvasState,
};

/// Next generated sequential identifier for a new station.
static STATION_ID: AtomicU64 = AtomicU64::new(1);

/// An identifier for a station.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

/// Represents a metro station, including its grid position on the map, its id,
/// name and if the station should be greyed out when drawn to the canvas.
#[derive(Clone, Debug)]
pub struct Station {
    /// Position of the station
    pos: GridNode,
    /// ID of the station
    id: StationID,
    /// If when drawn the station should be greyed out (like when moving)
    is_ghost: bool,
    /// The station name
    name: String,
}

impl Station {
    /// Create a new [`Station`] at the given grid coordinate.
    /// If id is None, the next sequential id is used.
    pub fn new(pos: GridNode, id: Option<StationID>) -> Self {
        if let Some(new_id) = id {
            if STATION_ID.load(Ordering::SeqCst) <= new_id.into() {
                STATION_ID.store(u64::from(new_id) + 1, Ordering::SeqCst);
            }
        }

        Self {
            pos,
            id: id.unwrap_or_else(|| {
                STATION_ID
                    .fetch_add(1, Ordering::SeqCst)
                    .into()
            }),
            is_ghost: false,
            name: String::new(),
        }
    }

    /// A getter for the id.
    #[inline]
    pub fn get_id(&self) -> StationID {
        self.id
    }

    /// A getter for the grid position.
    pub fn get_pos(&self) -> GridNode {
        self.pos
    }

    /// A setter for if the stations should be greyed out.
    pub fn set_is_ghost(&mut self, ghost: bool) {
        self.is_ghost = ghost;
    }

    /// A setter for the grid position of the station.
    pub fn set_pos(&mut self, pos: GridNode) {
        self.pos = pos;
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

    /// If the given node is a neighboring grid node to the station.
    pub fn is_neighbor(&self, node: GridNode) -> bool {
        self.get_pos()
            .get_neighbors()
            .contains(&node)
    }

    /// Draw the station to the given canvas.
    pub fn draw(&self, canvas: &CanvasContext<'_>, state: CanvasState) {
        if !state.is_on_canvas(self.get_pos()) {
            return;
        }

        let canvas_pos = self.get_canvas_pos(state);

        let mut width = state.drawn_square_size() / 10.0 + 1.0;
        if width < 2.0 {
            width = 2.0;
        }

        canvas.set_line_width(width);
        if self.is_ghost {
            canvas.set_global_alpha(0.5);
        } else {
            canvas.set_global_alpha(1.0);
        }
        canvas.set_stroke_style("black");
        canvas.begin_path();
        canvas
            .arc(
                canvas_pos.0,
                canvas_pos.1,
                state.drawn_square_size() / 3.0,
                0.0,
                2.0 * f64::consts::PI,
            )
            .unwrap();
        canvas.stroke();
    }
}

impl PartialEq for Station {
    fn eq(&self, other: &Station) -> bool {
        other.id == self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_station() {
        let before_id = STATION_ID.load(Ordering::Relaxed);

        let first_station = Station::new((2, 3).into(), None);
        let second_station = Station::new(
            (2, 3).into(),
            Some(StationID(before_id + 5)),
        );

        let after_id = STATION_ID.load(Ordering::Acquire);

        assert_eq!(before_id + 6, after_id);
        assert_eq!(
            first_station.get_id(),
            StationID(before_id)
        );
        assert_eq!(
            second_station.get_id(),
            StationID(before_id + 5)
        );
    }
}
