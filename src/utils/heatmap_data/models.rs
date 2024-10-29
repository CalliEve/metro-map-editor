//! Contains the models for containing the heatmap data and serializing it to JSON.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use crate::models::{
    GridNode,
    StationID,
};

/// Contains data on a neighbor of a station.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeighborData {
    /// The ID of the neighbor station.
    pub station_id: StationID,
    /// The distance to the neighbor station.
    pub distance: f64,
    /// The angle to the neighbor station calculated from an imaginary line
    /// straight up from the station.
    pub angle: f64,
}

/// Contains data on the station and its heatmap.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationHeatMap {
    /// The ID of the station.
    pub station_id: StationID,
    /// The current position of the station.
    pub pos: GridNode,
    /// The original position where the station was placed.
    pub original_pos: GridNode,
    /// The heatmap of costs around the station.
    pub heatmap: HashMap<String, f64>,
    /// Data on the neighbors of the station.
    pub neighbors: Vec<NeighborData>,
}

/// Contains the heatmap data for all stations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeatmapData {
    /// The list of heatmap data for all stations.
    pub stations: Vec<StationHeatMap>,
}