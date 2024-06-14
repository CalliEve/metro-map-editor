use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Data {
    #[serde(rename = "@key")]
    pub(super) key: String,
    #[serde(rename = "$text")]
    pub(super) value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Node {
    #[serde(rename = "@id")]
    pub(super) id: String,
    #[serde(default)]
    pub(super) data: Vec<Data>,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum GraphItem {
    Node(Node),
    Edge(Edge),
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Graph {
    #[serde(rename = "$value")]
    pub(super) content: Vec<GraphItem>,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphMlMap {
    pub(super) graph: Graph,
    pub(super) key: Vec<Key>,
}
