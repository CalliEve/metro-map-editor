//! Contains the [`CanvasState`] struct and its methods.

use crate::models::GridNode;

/// Contains the current state of the canvas.
#[derive(Clone, Copy, Debug)]
pub struct CanvasState {
    /// The height and width of the current canvas.
    size: (u32, u32),
    /// The size of the map grid squares.
    square_size: u32,
    /// How much the canvas is zoomed in.
    zoom_factor: f64,
    /// The height and width offset from panning the canvas.
    offset: (i32, i32),
    /// The maximum and minimum values for the x-axis.
    x_limit: (i32, i32),
    /// The maximum and minimum values for the x-axis.
    y_limit: (i32, i32),
}

impl CanvasState {
    /// Create a new canvas state with default values.
    pub fn new() -> Self {
        let mut s = Self {
            size: (300, 300),
            square_size: 7,
            zoom_factor: 1.0,
            offset: (0, 0),
            x_limit: (0, 0),
            y_limit: (0, 0),
        };
        s.recalculate_limits();
        s
    }

    /// A getter method for the canvas size.
    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Get the size of the canvas that is currently visible.
    pub fn get_visible_size(&self) -> (u32, u32) {
        let width = (self
            .x_limit
            .1
            - self
                .x_limit
                .0) as u32;
        let height = (self
            .y_limit
            .1
            - self
                .y_limit
                .0) as u32;

        (width, height)
    }

    /// A setter method for the canvas size.
    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
        self.recalculate_limits();
    }

    /// A getter method for the grid square size.
    pub fn get_square_size(&self) -> u32 {
        self.square_size
    }

    /// A setter method for the grid square size.
    pub fn set_square_size(&mut self, size: u32) {
        self.square_size = size;
        self.recalculate_limits();
    }

    /// Zooms in on the canvas.
    pub fn zoom_in(&mut self) {
        let old_x = self.x_limit;
        let old_y = self.y_limit;

        self.zoom_factor += 0.1;
        self.recalculate_limits();

        let x_change = (self
            .x_limit
            .1
            - old_x.1)
            / 2;
        let y_change = (self
            .y_limit
            .1
            - old_y.1)
            / 2;

        self.offset
            .0 -= x_change;
        self.offset
            .1 -= y_change;

        self.recalculate_limits();
    }

    /// Zooms out on the canvas.
    pub fn zoom_out(&mut self) {
        let old_x = self.x_limit;
        let old_y = self.y_limit;

        self.zoom_factor -= 0.1;
        if self.zoom_factor <= 0.21 {
            self.zoom_factor = 0.2;
        }

        self.recalculate_limits();

        let x_change = (self
            .x_limit
            .1
            - old_x.1)
            / 2;
        let y_change = (self
            .y_limit
            .1
            - old_y.1)
            / 2;

        self.offset
            .0 -= x_change;
        self.offset
            .1 -= y_change;

        self.recalculate_limits();
    }

    /// Move upwards on the canvas.
    pub fn move_up(&mut self) {
        let mut amount = (self
            .y_limit
            .1
            - self
                .y_limit
                .0)
            / 30;

        if amount < 1 {
            amount = 1;
        }

        self.offset
            .1 -= amount;
        self.recalculate_limits();
    }

    /// Move downwards on the canvas.
    pub fn move_down(&mut self) {
        let mut amount = (self
            .y_limit
            .1
            - self
                .y_limit
                .0)
            / 30;

        if amount < 1 {
            amount = 1;
        }

        self.offset
            .1 += amount;
        self.recalculate_limits();
    }

    /// Move leftwards on the canvas.
    pub fn move_left(&mut self) {
        let mut amount = (self
            .x_limit
            .1
            - self
                .x_limit
                .0)
            / 30;

        if amount < 1 {
            amount = 1;
        }

        self.offset
            .0 -= amount;
        self.recalculate_limits();
    }

    /// Move rightwards on the canvas.
    pub fn move_right(&mut self) {
        let mut amount = (self
            .x_limit
            .1
            - self
                .x_limit
                .0)
            / 30;

        if amount < 1 {
            amount = 1;
        }

        self.offset
            .0 += amount;
        self.recalculate_limits();
    }

    /// A getter method for the zoom factor of the canvas.
    pub fn get_zoom_factor(&self) -> f64 {
        self.zoom_factor
    }

    /// A setter method for the zoom factor of the canvas.
    pub fn set_zoom_factor(&mut self, factor: f64) {
        self.zoom_factor = factor;
        self.recalculate_limits();
    }

    /// Get the square size with zoom factored in.
    pub fn drawn_square_size(&self) -> f64 {
        f64::from(self.get_square_size()) * self.get_zoom_factor()
    }

    /// A getter method for the offset
    pub fn get_offset(&self) -> (i32, i32) {
        self.offset
    }

    /// A setter method for the offset
    pub fn set_offset(&mut self, offset: (i32, i32)) {
        self.offset = offset;
        self.recalculate_limits();
    }

    /// Recalculates the maximum and minimum values for the x and y coordinates
    /// to fit on the canvas.
    fn recalculate_limits(&mut self) {
        let width_node_count = (f64::from(
            self.size
                .1,
        ) / self.drawn_square_size())
        .round() as i32;
        let height_node_count = (f64::from(
            self.size
                .0,
        ) / self.drawn_square_size())
        .round() as i32;

        self.x_limit = (
            self.offset
                .0,
            width_node_count
                + self
                    .offset
                    .0,
        );
        self.y_limit = (
            self.offset
                .1,
            height_node_count
                + self
                    .offset
                    .1,
        );
    }

    /// Returns true if the given grid node is on the canvas.
    #[inline]
    pub fn is_on_canvas(&self, node: GridNode) -> bool {
        self.x_limit
            .0
            - 1
            < node.0
            && node.0
                < self
                    .x_limit
                    .1
                    + 1
            && self
                .y_limit
                .0
                - 1
                < node.1
            && node.1
                < self
                    .y_limit
                    .1
                    + 1
    }
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::new()
    }
}
