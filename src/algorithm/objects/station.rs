use std::{cell::RefCell, f64};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::algorithm::utils::calc_canvas_loc;

use super::Drawable;

#[derive(Clone, Debug, PartialEq)]
pub struct Station {
    pos: RefCell<(u32, u32)>,
}

impl Station {
    pub fn new(pos: (u32, u32)) -> Self {
        Self {
            pos: RefCell::new(pos),
        }
    }

    pub fn get_pos(&self) -> (u32, u32) {
        *self.pos.borrow()
    }
}

impl Drawable for Station {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let canvas_pos = calc_canvas_loc(self.get_pos(), square_size);

        canvas.set_line_width(4.0);
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
