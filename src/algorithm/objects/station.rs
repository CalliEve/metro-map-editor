use std::{
    cell::Cell,
    f64,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::calc_canvas_loc;

use super::Drawable;

static STATION_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Debug)]
pub struct Station {
    /// Position of the station
    pos: Rc<Cell<(i32, i32)>>,
    /// ID of the station
    id: String,
    /// If when drawn the station should be greyed out (like when moving)
    is_ghost: bool,
    /// The station name
    name: String,
}

impl Station {
    pub fn new(pos: (i32, i32), id: Option<String>) -> Self {
        Self {
            pos: Rc::new(Cell::new(pos)),
            id: id.unwrap_or_else(|| STATION_ID.fetch_add(1, Ordering::SeqCst).to_string()),
            is_ghost: false,
            name: String::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos.get()
    }

    pub fn set_is_ghost(&mut self, ghost: bool) {
        self.is_ghost = ghost;
    }

    pub fn set_pos(&self, pos: (i32, i32)) {
        self.pos.set(pos);
    }

    pub fn get_canvas_pos(&self, square_size: u32) -> (f64, f64) {
        calc_canvas_loc(self.get_pos(), square_size)
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = name.to_string();
    }

    pub fn clone_non_ref(&self) -> Self {
        Self {
            id: self.id.clone(),
            is_ghost: self.is_ghost,
            pos: Rc::new(Cell::new(self.get_pos())),
            name: self.name.clone(),
        }
    }
}

impl Drawable for Station {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let canvas_pos = self.get_canvas_pos(square_size);

        canvas.set_line_width(4.0);
        if self.is_ghost {
            canvas.set_global_alpha(0.5);
        }
        canvas.set_stroke_style(&JsValue::from_str("black"));
        canvas.begin_path();
        canvas
            .arc(
                canvas_pos.0,
                canvas_pos.1,
                square_size as f64 / 3.0,
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
