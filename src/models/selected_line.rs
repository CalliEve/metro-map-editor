//! Contains the [`SelectedLine`] struct and all its methods.

use wasm_bindgen::JsValue;
use web_sys::js_sys::Uint8Array;

use super::{
    Drawable,
    GridNode,
    Line,
    Station,
};
use crate::{
    algorithm::{
        draw_edge,
        run_a_star,
    },
    components::CanvasState,
};

/// Holds information about the currently selected [`Line`].
#[derive(Debug, Clone)]
pub struct SelectedLine {
    /// The selected line.
    line: Line,
    /// The coordinate the line was grabbed at.
    grabbed_at: Option<GridNode>,
    /// The stations before and after the point the line was grabbed if
    /// applicable.
    before_after: (Option<Station>, Option<Station>),
    /// The grid coordinate the user is currently hovering over.
    current_hover: GridNode,
}

impl SelectedLine {
    /// Select a line.
    pub fn new(line: Line, current_hover: GridNode, grabbed_at: Option<GridNode>) -> Self {
        let mut before_after = (None, None);
        if let Some(grabbed_node) = grabbed_at {
            before_after = line.get_edge_stations(grabbed_node);
        }

        Self {
            line,
            grabbed_at,
            before_after,
            current_hover,
        }
    }

    /// Select a newly created line.
    pub fn new_line() -> Self {
        Self::new(
            Line::new(Vec::new(), None),
            GridNode::from((-1, -1)),
            None,
        )
    }

    /// Get the current hover coordinate.
    pub fn get_current_hover(&self) -> GridNode {
        self.current_hover
    }

    /// Set the current hover coordinate.
    pub fn set_current_hover(&mut self, at: GridNode) {
        self.current_hover = at;
    }

    /// Get the underlying selected line.
    pub fn get_line(&self) -> &Line {
        &self.line
    }

    /// Get the coordinate the line was grabbet at.
    pub fn get_grabbed_at(&self) -> Option<GridNode> {
        self.grabbed_at
    }

    /// Get the stations before and after the point the line was grabbed.
    pub fn get_before_after(&self) -> (Option<&Station>, Option<&Station>) {
        let (before, after) = &self.before_after;
        (before.as_ref(), after.as_ref())
    }

    /// Set the station that came before the point the line was grabbed.
    pub fn set_before(&mut self, before: Station) {
        self.before_after
            .0 = Some(before);
    }

    /// Set the station that came after the point the line was grabbed.
    pub fn set_after(&mut self, after: Station) {
        self.before_after
            .1 = Some(after);
    }
}

impl Drawable for SelectedLine {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, state: CanvasState) {
        let (hover_x, hover_y) = self
            .get_current_hover()
            .to_canvas_pos(state);
        let half_square = state.drawn_square_size() / 2.0;

        canvas.set_line_width(3.0);
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
        canvas.move_to(hover_x + half_square, hover_y);
        canvas.line_to(hover_x - half_square, hover_y);

        let draw_before = |before: &Station| {
            draw_edge(
                before.get_pos(),
                self.get_current_hover(),
                &run_a_star(
                    before.get_pos(),
                    self.get_current_hover(),
                ),
                canvas,
                state,
            );
        };
        let draw_after = |after: &Station| {
            draw_edge(
                self.get_current_hover(),
                after.get_pos(),
                &run_a_star(
                    self.get_current_hover(),
                    after.get_pos(),
                ),
                canvas,
                state,
            );
        };

        match self.get_before_after() {
            (None, None) => {},
            (Some(before), None) => {
                draw_before(before);
            },
            (None, Some(after)) => {
                draw_after(after);
            },
            (Some(before), Some(after)) => {
                draw_before(before);
                draw_after(after);
            },
        }

        canvas.stroke();
        canvas.begin_path();

        if let Some(origin) = self.get_grabbed_at() {
            canvas
                .set_line_dash(&Uint8Array::from([5u8, 5].as_ref()))
                .unwrap();

            let (origin_x, origin_y) = origin.to_canvas_pos(state);
            canvas.move_to(origin_x, origin_y);
            canvas.line_to(hover_x, hover_y);

            canvas.stroke();
        }
    }
}
