//! Contains all objects that together represent the data in a GraphML file.

use serde::{
    Deserialize,
    Serialize,
};

/// A key-value pair that contains information about its parent object.
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Data {
    #[serde(rename = "@key")]
    pub(super) key: String,
    #[serde(rename = "$text")]
    pub(super) value: String,
}

/// Represents a station in the GraphML.
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Node {
    #[serde(rename = "@id")]
    pub(super) id: String,
    #[serde(default)]
    pub(super) data: Vec<Data>,
}

/// Represents an edge connecting two stations as part of a line.
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Edge {
    #[serde(rename = "@id")]
    pub(super) id: String,
    #[serde(rename = "@source")]
    pub(super) source: String,
    #[serde(rename = "@target")]
    pub(super) target: String,
    pub(super) data: Vec<Data>,
}

/// This GraphML object can either be a [`Node`] (station) or [`Edge`] (edge
/// connecting two stations).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum GraphItem {
    Node(Node),
    Edge(Edge),
}

/// The struct containing a list of either [`Edge`] or [`Node`]
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Graph {
    #[serde(rename = "$value")]
    pub(super) content: Vec<GraphItem>,
}

/// A [`Key`] represents base information about either a line or what
/// information is available about a station.
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Key {
    #[serde(rename = "@id")]
    pub(super) id: String,
    #[serde(rename = "@for")]
    pub(super) for_item: String,
    #[serde(rename = "@color.r")]
    pub(super) r: Option<String>,
    #[serde(rename = "@color.g")]
    pub(super) g: Option<String>,
    #[serde(rename = "@color.g")]
    pub(super) b: Option<String>,
    #[serde(rename = "@attr.name")]
    pub(super) name: String,
}

/// The root struct that contains the map in a GraphML data file
#[derive(Debug, Deserialize, Serialize)]
pub struct GraphMlMap {
    pub(super) graph: Graph,
    pub(super) key: Vec<Key>,
}
