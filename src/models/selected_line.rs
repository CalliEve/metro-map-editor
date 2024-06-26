use wasm_bindgen::JsValue;
use web_sys::js_sys::Uint8Array;

use super::{
    Drawable,
    Line,
    Station,
};
use crate::{
    algorithm::draw_edge,
    utils::calc_canvas_loc,
};

/// Holds information about the currently selected line.
#[derive(Debug, Clone)]
pub struct SelectedLine {
    /// The selected line.
    line: Line,
    /// The coordinate the line was grabbed at.
    grabbed_at: Option<(i32, i32)>,
    /// The stations before and after the point the line was grabbed if
    /// applicable.
    before_after: (Option<Station>, Option<Station>),
    /// The grid coordinate the user is currently hovering over.
    current_hover: (i32, i32),
}

impl SelectedLine {
    /// Select a line.
    pub fn new(line: Line, current_hover: (i32, i32), grabbed_at: Option<(i32, i32)>) -> Self {
        Self {
            line,
            current_hover,
            grabbed_at,
            before_after: (None, None),
        }
    }

    /// Select a newly created line.
    pub fn new_line() -> Self {
        Self::new(
            Line::new(Vec::new(), None),
            (-1, -1),
            None,
        )
    }

    /// Get the current hover coordinate.
    pub fn get_current_hover(&self) -> (i32, i32) {
        self.current_hover
    }

    /// Set the current hover coordinate.
    pub fn set_current_hover(&mut self, at: (i32, i32)) {
        self.current_hover = at;
    }

    /// Get the underlying selected line.
    pub fn get_line(&self) -> &Line {
        &self.line
    }

    /// Get the coordinate the line was grabbet at.
    pub fn get_grabbed_at(&self) -> Option<(i32, i32)> {
        self.grabbed_at
    }

    pub fn get_before_after(&self) -> (Option<&Station>, Option<&Station>) {
        let (before, after) = &self.before_after;
        (before.as_ref(), after.as_ref())
    }
}

impl Drawable for SelectedLine {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, square_size: u32) {
        let fake_station = Station::new(
            self.get_current_hover(),
            Some("temp_line".to_owned()),
        );
        let (hover_x, hover_y) = calc_canvas_loc(self.get_current_hover(), square_size);
        let half_square = f64::from(square_size) / 2.0;

        canvas.set_line_width(2.0);
        canvas.set_stroke_style(&JsValue::from_str(&format!(
            "rgb({} {} {})",
            self.get_line()
                .get_color()
                .0,
            self.get_line()
                .get_color()
                .1,
            self.get_line()
                .get_color()
                .2
        )));
        canvas.set_global_alpha(0.5);
        canvas.begin_path();

        canvas.move_to(hover_x, hover_y + half_square);
        canvas.line_to(hover_x, hover_y - half_square);

        match self.get_before_after() {
            (None, None) => {
                canvas.move_to(hover_x + half_square, hover_y);
                canvas.line_to(hover_x - half_square, hover_y);
            },
            (Some(before), None) => {
                draw_edge(
                    before,
                    &fake_station,
                    canvas,
                    square_size,
                );
            },
            (None, Some(after)) => {
                draw_edge(
                    &fake_station,
                    after,
                    canvas,
                    square_size,
                );
            },
            (Some(before), Some(after)) => {
                draw_edge(
                    before,
                    &fake_station,
                    canvas,
                    square_size,
                );
                draw_edge(
                    &fake_station,
                    after,
                    canvas,
                    square_size,
                );
            },
        }

        canvas.stroke();

        if let Some(origin) = self.get_grabbed_at() {
            canvas
                .set_line_dash(&Uint8Array::from([5u8, 5].as_ref()))
                .unwrap();

            let (origin_x, origin_y) = calc_canvas_loc(origin, square_size);
            canvas.move_to(origin_x, origin_y);
            canvas.line_to(hover_x, hover_y);

            canvas.stroke();
        }
    }
}
