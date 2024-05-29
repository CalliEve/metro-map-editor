use std::{
    cell::Cell,
    f64,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::algorithm::utils::calc_canvas_loc;

use super::Drawable;

static STATION_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Debug)]
pub struct Station {
    /// Position of the station
    pos: Rc<Cell<(i32, i32)>>,
    /// ID of the station
    id: u32,
    /// If when drawn the station should be greyed out (like when moving)
    is_ghost: bool,
}

impl Station {
    pub fn new(pos: (i32, i32)) -> Self {
        Self {
            pos: Rc::new(Cell::new(pos)),
            id: STATION_ID.fetch_add(1, Ordering::SeqCst),
            is_ghost: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
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

    pub fn clone_non_ref(&self) -> Self {
        Self {
            id: self.id,
            is_ghost: self.is_ghost,
            pos: Rc::new(Cell::new(self.get_pos())),
        }
    }
}

impl Drawable for Station {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let canvas_pos = calc_canvas_loc(self.get_pos(), square_size);

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
                (square_size / 3) as f64,
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
