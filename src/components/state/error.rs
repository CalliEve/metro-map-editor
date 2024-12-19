//! Contains the [`ErrorState`] and its methods.

use crate::Error;

/// Contains the current error state.
/// This state is then used to display the error message to the user.
#[derive(Clone, Debug)]
pub struct ErrorState {
    /// The error last encountered.
    error: Option<Error>,
}

impl ErrorState {
    /// Creates a new [`ErrorState`] with no error.
    pub fn new() -> Self {
        Self {
            error: None,
        }
    }

    /// Sets the error to be displayed.
    pub fn set_error(&mut self, error: impl Into<Error>) {
        self.error = Some(error.into());
    }

    /// Returns if there is an error to be displayed.
    pub fn has_error(&self) -> bool {
        self.error
            .is_some()
    }

    /// Gets the current error, if any.
    pub fn get_error(&self) -> Option<&Error> {
        self.error
            .as_ref()
    }

    /// Clears the current error.
    pub fn clear_error(&mut self) {
        self.error = None;
    }
}
