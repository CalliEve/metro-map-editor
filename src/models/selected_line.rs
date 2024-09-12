//! Contains the [`SelectedLine`] struct and all its methods.

use super::{
    GridNode,
    Line,
    LineID,
    Map,
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

/// Holds information about the currently selected [`Line`].
#[derive(Debug, Copy, Clone)]
pub struct SelectedLine {
    /// The selected line.
    line: LineID,
    /// The coordinate the line was grabbed at.
    grabbed_at: Option<GridNode>,
    /// The stations before and after the point the line was grabbed if
    /// applicable.
    before_after: (Option<StationID>, Option<StationID>),
    /// The grid coordinate the user is currently hovering over.
    current_hover: GridNode,
}

impl SelectedLine {
    /// Select a line.
    pub fn new(
        line: &Line,
        map: &Map,
        current_hover: GridNode,
        grabbed_at: Option<GridNode>,
    ) -> Self {
        let mut before_after = (None, None);
        if let Some(grabbed_node) = grabbed_at {
            before_after = line.get_edge_stations(map, grabbed_node);
        }

        Self {
            line: line.get_id(),
            grabbed_at,
            before_after,
            current_hover,
        }
    }

    /// Select a newly created line.
    pub fn new_line(map: &mut Map) -> Self {
        let line = Line::new(None);
        let line_id = line.get_id();
        map.add_line(line);

        Self {
            line: line_id,
            current_hover: GridNode::from((i32::MIN, i32::MIN)),
            before_after: (None, None),
            grabbed_at: None,
        }
    }

    /// Get the current hover coordinate.
    #[inline]
    pub fn get_current_hover(&self) -> GridNode {
        self.current_hover
    }

    /// Set the current hover coordinate.
    pub fn set_current_hover(&mut self, at: GridNode) {
        self.current_hover = at;
    }

    /// Get the underlying selected line.
    #[inline]
    pub fn get_line(&self) -> LineID {
        self.line
    }

    /// Get the coordinate the line was grabbet at.
    #[inline]
    pub fn get_grabbed_at(&self) -> Option<GridNode> {
        self.grabbed_at
    }

    /// Get the stations before and after the point the line was grabbed.
    #[inline]
    pub fn get_before_after(&self) -> (Option<StationID>, Option<StationID>) {
        self.before_after
    }

    /// Set the station that came before the point the line was grabbed.
    pub fn set_before(&mut self, before: StationID) {
        self.before_after
            .0 = Some(before);
    }

    /// Set the station that came after the point the line was grabbed.
    pub fn set_after(&mut self, after: StationID) {
        self.before_after
            .1 = Some(after);
    }

    /// Draw the selected line to the given canvas.
    pub fn draw(&self, map: &Map, canvas: &CanvasContext<'_>, state: CanvasState) {
        let (hover_x, hover_y) = self
            .get_current_hover()
            .to_canvas_pos(state);
        let half_square = state.drawn_square_size() / 2.0;
        let line = map
            .get_line(self.get_line())
            .expect("drawing invalid line id");

        canvas.set_line_width(3.0);
        canvas.set_stroke_style(&format!(
            "rgb({} {} {})",
            line.get_color()
                .0,
            line.get_color()
                .1,
            line.get_color()
                .2
        ));
        canvas.set_global_alpha(0.5);
        canvas.begin_path();

        canvas.move_to(hover_x, hover_y + half_square);
        canvas.line_to(hover_x, hover_y - half_square);
        canvas.move_to(hover_x + half_square, hover_y);
        canvas.line_to(hover_x - half_square, hover_y);

        let draw_before = |before: StationID| {
            let before_station = map
                .get_station(before)
                .expect("invalid station id");
            draw_edge(
                self.get_current_hover(),
                before_station.get_pos(),
                &run_a_star(
                    self.get_current_hover(),
                    before_station.get_pos(),
                ),
                canvas,
                state,
                0.0,
            );
        };
        let draw_after = |after: StationID| {
            let after_station = map
                .get_station(after)
                .expect("invalid station id");
            draw_edge(
                after_station.get_pos(),
                self.get_current_hover(),
                &run_a_star(
                    after_station.get_pos(),
                    self.get_current_hover(),
                ),
                canvas,
                state,
                0.0,
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
                .set_line_dash(&[5u8, 5])
                .unwrap();

            let (origin_x, origin_y) = origin.to_canvas_pos(state);
            canvas.move_to(origin_x, origin_y);
            canvas.line_to(hover_x, hover_y);

            canvas.stroke();
        }
    }
}
