//! Contains the [`Station`] struct and all its methods.

use std::{
    cell::Cell,
    f64,
    rc::Rc,
    sync::atomic::{
        AtomicU32,
        Ordering,
    },
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{
    Drawable,
    GridNode,
};

/// Next generated sequential identifier for a new station.
static STATION_ID: AtomicU32 = AtomicU32::new(1);

/// Represents a metro station, including its grid position on the map, its id,
/// name and if the station should be greyed out when drawn to the canvas.
#[derive(Clone, Debug)]
pub struct Station {
    /// Position of the station
    pos: Rc<Cell<GridNode>>,
    /// ID of the station
    id: String,
    /// If when drawn the station should be greyed out (like when moving)
    is_ghost: bool,
    /// The station name
    name: String,
}

impl Station {
    /// Create a new [`Station`] at the given grid coordinate.
    /// If id is None, the next sequential id from [`STATION_ID`] is used.
    pub fn new(pos: GridNode, id: Option<String>) -> Self {
        Self {
            pos: Rc::new(Cell::new(pos)),
            id: id.unwrap_or_else(|| {
                STATION_ID
                    .fetch_add(1, Ordering::SeqCst)
                    .to_string()
            }),
            is_ghost: false,
            name: String::new(),
        }
    }

    /// A getter for the id.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// A getter for the grid position.
    pub fn get_pos(&self) -> GridNode {
        self.pos
            .get()
    }

    /// A setter for if the stations should be greyed out.
    pub fn set_is_ghost(&mut self, ghost: bool) {
        self.is_ghost = ghost;
    }

    /// A setter for the grid position of the station.
    pub fn set_pos(&mut self, pos: GridNode) {
        self.pos
            .set(pos);
    }

    /// The location of the station on the canvas, given the size of a grid
    /// square.
    pub fn get_canvas_pos(&self, square_size: u32) -> (f64, f64) {
        self.get_pos()
            .to_canvas_pos(square_size)
    }

    /// A setter for the name.
    pub fn set_name(&mut self, name: &impl ToString) {
        self.name = name.to_string();
    }

    /// A getter for the name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Clone the [`Station`] without keeping a reference to the coordinate
    /// position.
    pub fn clone_non_ref(&self) -> Self {
        Self {
            id: self
                .id
                .clone(),
            is_ghost: self.is_ghost,
            pos: Rc::new(Cell::new(self.get_pos())),
            name: self
                .name
                .clone(),
        }
    }
}

impl Drawable for Station {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let canvas_pos = self.get_canvas_pos(square_size);

        canvas.set_line_width(4.0);
        if self.is_ghost {
            canvas.set_global_alpha(0.5);
        } else {
            canvas.set_global_alpha(1.0);
        }
        canvas.set_stroke_style(&JsValue::from_str("black"));
        canvas.begin_path();
        canvas
            .arc(
                canvas_pos.0,
                canvas_pos.1,
                f64::from(square_size) / 3.0,
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
        let second_station = Station::new((2, 3).into(), Some("test".to_owned()));

        let after_id = STATION_ID.load(Ordering::Acquire);

        assert_eq!(before_id + 1, after_id);
        assert_eq!(
            first_station.get_id(),
            before_id.to_string()
        );
        assert_eq!(second_station.get_id(), "test");
    }

    #[test]
    fn test_clone_non_ref() {
        let before_pos = GridNode::from((16, 20));
        let before_station = Station::new(before_pos, None);

        let mut after_station = before_station.clone_non_ref();

        let after_pos = GridNode::from((20, 30));
        after_station.set_pos(after_pos);

        assert_eq!(before_station.get_pos(), before_pos);
        assert_eq!(after_station.get_pos(), after_pos);
        assert_eq!(
            before_station.get_id(),
            after_station.get_id()
        );
    }
}
