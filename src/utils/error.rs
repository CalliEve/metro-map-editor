//! Contains the custom [`Error`] type and the [`Result`] type alias.

use std::{
    fmt::Display,
    sync::Arc,
};

use leptos::logging;
use ordered_float::FloatIsNan;
use serde::{
    de::{
        value,
        Error as DeError,
    },
    Deserialize,
    Serialize,
};
use serde_json::Map;
use wasm_bindgen::JsValue;

/// A custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Json(Arc<serde_json::Error>),
    GraphML(quick_xml::DeError),
    InvalidFloat(FloatIsNan),
    EarlyAbort,
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

    /// Returns the type of the error as a string.
    fn get_type(&self) -> &'static str {
        match self {
            Self::Json(_) => "json",
            Self::GraphML(_) => "graphml",
            Self::InvalidFloat(_) => "invalid_float",
            Self::EarlyAbort => "early_abort",
            Self::DecodeError(_) => "decode_error",
            Self::Other(_) => "other",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::GraphML(e) => write!(f, "GraphML error: {e}"),
            Self::InvalidFloat(e) => write!(f, "Invalid float error: {e}"),
            Self::EarlyAbort => {
                write!(
                    f,
                    "Aborting algorithm early as no possible improvement can be reached."
                )
            },
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
        Self::Json(Arc::new(e))
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

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = Map::new();
        map.insert(
            "type".to_owned(),
            self.get_type()
                .into(),
        );

        map.insert(
            "data".to_owned(),
            match self {
                Self::Json(e) => e.to_string(),
                Self::GraphML(e) => e.to_string(),
                Self::InvalidFloat(e) => e.to_string(),
                Self::EarlyAbort => {
                    self.get_type()
                        .to_string()
                },
                Self::DecodeError(e) | Self::Other(e) => e.to_string(),
            }
            .into(),
        );

        map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let map = value
            .as_object()
            .ok_or(D::Error::custom(
                "expected object for error",
            ))?;
        match map
            .get("type")
            .ok_or(D::Error::custom(
                "error object must have type",
            ))?
            .as_str()
            .ok_or(D::Error::custom(
                "error type must be a string",
            ))? {
            "json" => {
                let e = value
                    .get("data")
                    .ok_or(D::Error::custom(
                        "json error must have data",
                    ))?
                    .as_str()
                    .ok_or(D::Error::custom(
                        "json error data must be a string",
                    ))?;
                Ok(Self::Json(Arc::new(
                    serde_json::Error::custom(e),
                )))
            },
            "graphml" => {
                let e = value
                    .get("data")
                    .ok_or(D::Error::custom(
                        "graphml error must have data",
                    ))?
                    .as_str()
                    .ok_or(D::Error::custom(
                        "graphml error data must be a string",
                    ))?;
                Ok(Self::GraphML(
                    quick_xml::DeError::custom(e),
                ))
            },
            "invalid_float" => {
                let e = value
                    .get("data")
                    .ok_or(D::Error::custom(
                        "invalid float error must have data",
                    ))?
                    .as_str()
                    .ok_or(D::Error::custom(
                        "invalid float error data must be a string",
                    ))?;
                Ok(Self::InvalidFloat(FloatIsNan))
            },
            "decode_error" => {
                let e = value
                    .get("data")
                    .ok_or(D::Error::custom(
                        "decode error must have data",
                    ))?
                    .as_str()
                    .ok_or(D::Error::custom(
                        "decode error data must be a string",
                    ))?;
                Ok(Self::DecodeError(e.to_string()))
            },
            "other" => {
                let e = value
                    .get("data")
                    .ok_or(D::Error::custom(
                        "other error must have data",
                    ))?
                    .as_str()
                    .ok_or(D::Error::custom(
                        "other error data must be a string",
                    ))?;
                Ok(Self::Other(e.to_string()))
            },
            "early_abort" => Ok(Self::EarlyAbort),
            _ => Err(D::Error::custom("unknown error type")),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_serde() {
        let error_json: Result<()> = Err(Error::Json(Arc::new(
            serde_json::Error::custom("test"),
        )));
        let serialized_json = serde_json::to_string(&error_json).unwrap();
        let deserialized_json = serde_json::from_str(&serialized_json).unwrap();
        assert_eq!(error_json, deserialized_json);

        let error_graphml = Error::GraphML(quick_xml::DeError::custom("test"));
        let serialized_graphml = serde_json::to_string(&error_graphml).unwrap();
        let deserialized_graphml: crate::Error = serde_json::from_str(&serialized_graphml).unwrap();
        assert_eq!(error_graphml, deserialized_graphml);

        let error_invalid_float = Error::InvalidFloat(FloatIsNan);
        let serialized_invalid_float = serde_json::to_string(&error_invalid_float).unwrap();
        let deserialized_invalid_float: crate::Error =
            serde_json::from_str(&serialized_invalid_float).unwrap();
        assert_eq!(
            error_invalid_float,
            deserialized_invalid_float
        );

        let error_decode = Error::DecodeError("test".to_string());
        let serialized_decode = serde_json::to_string(&error_decode).unwrap();
        let deserialized_decode: crate::Error = serde_json::from_str(&serialized_decode).unwrap();
        assert_eq!(error_decode, deserialized_decode);

        let error_other: Result<crate::models::Station> = Err(Error::Other("test".to_string()));
        let serialized_other = serde_json::to_string(&error_other).unwrap();
        let deserialized_other = serde_json::from_str(&serialized_other).unwrap();
        assert_eq!(error_other, deserialized_other);
    }
}
