//! Contains all objects that together represent the [`crate::models::Map`] in
//! the JSON file.

// No need to document all fields on the model structs here.
#![allow(clippy::missing_docs_in_private_items)]

use serde::{
    Deserialize,
    Serialize,
};

/// Represents a connection between two stations for the JSON file.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JSONEdge {
    pub source: String,
    pub target: String,
    pub lines: Vec<String>,
}

/// Represents a line for the JSON file.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JSONLine {
    pub id: String,
    pub name: Option<String>,
    pub color: Option<String>,
}

/// Represents a station for the JSON file.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JSONStation {
    pub id: String,
    pub name: Option<String>,
    pub x: f64,
    pub y: f64,
}

/// Represents the whole map in the JSON file.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JSONMap {
    pub stations: Vec<JSONStation>,
    pub lines: Vec<JSONLine>,
    pub edges: Vec<JSONEdge>,
}
