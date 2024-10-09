//! Contains the custom [`Error`] type and the [`Result`] type alias.

use std::fmt::Display;

use leptos::logging;
use ordered_float::FloatIsNan;
use wasm_bindgen::JsValue;

/// A custom error type for the application.
#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    GraphML(quick_xml::DeError),
    InvalidFloat(FloatIsNan),
    DecodeError(String),
    Other(String),
}

impl Error {
    /// Creates a new [`Error::DecodeError`] with the given message.
    pub fn decode_error<T: Display>(e: T) -> Self {
        Self::DecodeError(e.to_string())
    }

    /// Creates a new [`Error::Other`] error with the given message.
    pub fn other<T: Display>(e: T) -> Self {
        Self::Other(e.to_string())
    }

    /// Prints the error to the console.
    pub fn print_error(self) {
        logging::error!("{}", self);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::GraphML(e) => write!(f, "GraphML error: {e}"),
            Self::InvalidFloat(e) => write!(f, "Invalid float error: {e}"),
            Self::DecodeError(e) => write!(f, "Decode error: {e}"),
            Self::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Self::GraphML(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<FloatIsNan> for Error {
    fn from(e: FloatIsNan) -> Self {
        Self::InvalidFloat(e)
    }
}

impl From<csscolorparser::ParseColorError> for Error {
    fn from(e: csscolorparser::ParseColorError) -> Self {
        Self::DecodeError(format!("Failed to parse color: {e}"))
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Other(
            e.as_string()
                .unwrap_or_else(|| format!("{e:?}")),
        )
    }
}

impl std::error::Error for Error {}

impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (Self::Json(e1), Self::Json(e2)) => e1.to_string() == e2.to_string(),
            (Self::GraphML(e1), Self::GraphML(e2)) => e1.to_string() == e2.to_string(),
            (Self::InvalidFloat(e1), Self::InvalidFloat(e2)) => e1 == e2,
            (Self::DecodeError(e1), Self::DecodeError(e2)) | (Self::Other(e1), Self::Other(e2)) => {
                e1 == e2
            },
            _ => false,
        }
    }
}

/// A custom [`Result`] type for the application using the [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Unwraps the given [`Result`] and returns the value if it is `Ok`, otherwise
/// prints the error and returns without panicing.
#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                $crate::Error::from(e).print_error();
                return;
            },
        }
    };
}
