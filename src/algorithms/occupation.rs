//! Contains the data structure for specifying what is currently occupying a
//! node on the map.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

use crate::models::{
    EdgeID,
    GridNode,
    Map,
    StationID,
};

/// A map of grid nodes to the contents of the node, listing all nodes currently
/// occupied on the map.
pub type OccupiedNodes = HashMap<GridNode, OccupiedNode>;

/// Describes the contents of an occupied node of the map grid.
/// Either shows the node occupied by a station or by an edge.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OccupiedNode {
    /// The node is occupied by a station.
    Station(StationID),
    /// The node is occupied by an edge.
    Edge(EdgeID),
}

impl OccupiedNode {
    /// Get the ID of the edge if the node is occupied by an edge, else returns
    /// None.
    pub fn get_edge_id(&self) -> Option<EdgeID> {
        match self {
            Self::Edge(e) => Some(*e),
            Self::Station(_) => None,
        }
    }

    /// Get the ID of the station if the node is occupied by a station, else
    /// returns None.
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

/// Returns if the diagonal squared described by the given two nodes is already
/// occupied by an edge.
pub fn diagonal_occupied(
    map: &Map,
    first: GridNode,
    second: GridNode,
    occupied: &OccupiedNodes,
) -> bool {
    // Not diagonal
    if first.0 - second.0 == 0 || first.1 - second.1 == 0 {
        return false;
    }

    if let Some(diag_one) = occupied.get(&GridNode::from((first.0, second.1))) {
        if let Some(diag_two) = occupied.get(&GridNode::from((second.0, first.1))) {
            // if both diagonal nodes are occupied by same edge, the diagonal is occupied.
            if diag_one == diag_two {
                return true;
            }

            // if one of the diagonal nodes is a station, we check if the edge on the other
            // diagonal node is connected to it, if so, the diagonal is occupied.
            if let Some(station_id) = diag_one.get_station_id() {
                return map
                    .get_station(station_id)
                    .zip(diag_two.get_edge_id())
                    .is_some_and(|(s, edge_id)| {
                        s.get_edges()
                            .contains(&edge_id)
                    });
            }

            if let Some(station_id) = diag_two.get_station_id() {
                return map
                    .get_station(station_id)
                    .zip(diag_one.get_edge_id())
                    .is_some_and(|(s, edge_id)| {
                        s.get_edges()
                            .contains(&edge_id)
                    });
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Edge,
        Station,
    };

    #[test]
    fn test_diagonal_occupied() {
        let mut map = Map::new();
        let mut occupied = OccupiedNodes::new();

        let top_left = GridNode::from((0, 0));
        let top_right = GridNode::from((1, 0));
        let bottom_left = GridNode::from((0, 1));
        let bottom_right = GridNode::from((1, 1));

        let edge = Edge::new(0.into(), 1.into(), None);
        let edge_id = edge.get_id();
        let mut station = Station::new(GridNode::from((0, 0)), None);
        let station_id = station.get_id();
        station.add_edge(edge_id);
        map.add_station(station);

        assert!(!diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.insert(top_left, edge_id.into());
        occupied.insert(bottom_right, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, edge_id.into());
        occupied.insert(bottom_left, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));

        occupied.insert(top_left, edge_id.into());
        occupied.insert(bottom_right, station_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, edge_id.into());
        occupied.insert(bottom_left, station_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));

        occupied.insert(top_left, station_id.into());
        occupied.insert(bottom_right, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, station_id.into());
        occupied.insert(bottom_left, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));
    }
}
