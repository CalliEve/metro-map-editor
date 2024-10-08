use std::collections::HashMap;

use crate::models::{
    EdgeID,
    GridNode,
    StationID,
};

pub type OccupiedNodes = HashMap<GridNode, OccupiedNode>;

/// Describes the contents of an occupied node of the map grid.
/// Either shows the node occupied by a station or by an edge.
#[derive(Debug, Clone, Copy)]
pub enum OccupiedNode {
    Station(StationID),
    Edge(EdgeID),
}

impl OccupiedNode {
    pub fn get_edge_id(&self) -> Option<EdgeID> {
        match self {
            Self::Edge(e) => Some(*e),
            Self::Station(_) => None,
        }
    }

    pub fn get_station_id(&self) -> Option<StationID> {
        match self {
            Self::Station(s) => Some(*s),
            Self::Edge(_) => None,
        }
    }
}

impl PartialEq for OccupiedNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Station(a), Self::Station(b)) => a == b,
            (Self::Edge(a), Self::Edge(b)) => a == b,
            _ => false,
        }
    }
}

impl From<StationID> for OccupiedNode {
    fn from(id: StationID) -> Self {
        Self::Station(id)
    }
}

impl From<EdgeID> for OccupiedNode {
    fn from(id: EdgeID) -> Self {
        Self::Edge(id)
    }
}
