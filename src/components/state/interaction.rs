//! Contains the [`InteractionState`] struct and its methods.

/// Stores the current state of the user interaction.
/// This is for keeping track of the cursor state on the canvas and if the
/// application is busy.
#[derive(Debug, Clone)]
pub struct InteractionState {
    /// The cursor, can be "default", "wait" and "grabbing"
    cursor: String,
    /// If the application is busy.
    busy: bool,
}

impl InteractionState {
    /// Create a new interaction state with default values.
    pub fn new() -> Self {
        Self {
            cursor: "default".to_string(),
            busy: false,
        }
    }

    /// Set the cursor to a new value.
    pub fn set_cursor<S: ToString>(&mut self, cursor: S) {
        self.cursor = cursor.to_string();
    }

    /// Change if the application is busy.
    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }

    /// Get the current cursor.
    pub fn get_cursor(&self) -> &str {
        &self.cursor
    }

    /// Check if the application is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.busy
    }
}
