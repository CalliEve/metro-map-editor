//! Contains the models for containing the heatmap data and serializing it to
//! JSON.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    algorithm::OccupiedNode,
    models::{
        GridNode,
        StationID,
    },
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
    /// The location of the neighbor station.
    pub position: GridNode,
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
    pub heatmap: HashMap<GridNode, f64>,
    /// Data on the neighbors of the station.
    pub neighbors: Vec<NeighborData>,
    /// What is currently occupying the original position of the station.
    pub current_og_pos: Option<OccupiedNode>,
    /// The 4 neighbor nodes of the station that are on average closest to the
    /// neighboring stations, sorted closest to furthest.
    pub average_closest_coords: Vec<GridNode>,
}

/// Contains the heatmap data for all stations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeatmapData {
    /// The list of heatmap data for all stations.
    pub stations: Vec<StationHeatMap>,
}
