use std::sync::{
    Arc,
    Mutex,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::models::{
    EdgeID,
    LineID,
    StationID,
};

/// A global instance of the IDManager, only accessible through the static
/// methods of the manager.
static ID_MANAGER: IDManager = IDManager::new();

/// A manager for the ids of stations, lines and edges. This ensures that all
/// IDs are unique.
#[derive(Debug)]
pub struct IDManager {
    station_id: Mutex<u64>,
    line_id: Mutex<u64>,
    edge_id: Mutex<u64>,
}

impl IDManager {
    /// Create a new IDManager with all ids starting at 1.
    const fn new() -> Self {
        Self {
            station_id: Mutex::new(1),
            line_id: Mutex::new(1),
            edge_id: Mutex::new(1),
        }
    }

    /// Get the next station id. This is the next sequential id and should
    /// therefore be unique.
    pub fn next_station_id() -> StationID {
        let mut id_lock = ID_MANAGER
            .station_id
            .lock()
            .unwrap();
        let id = id_lock.clone();
        *id_lock += 1;
        StationID::from(id)
    }

    /// In the case that a station is created with a specific id, this function
    /// can be used to ensure the next sequential id is higher than this given
    /// id.
    pub fn update_station_id(id: StationID) {
        let mut id_lock = ID_MANAGER
            .station_id
            .lock()
            .unwrap();
        if u64::from(id) >= *id_lock {
            *id_lock = u64::from(id) + 1;
        }
    }

    /// Get the next line id. This is the next sequential id and should
    /// therefore be unique.
    pub fn next_line_id() -> LineID {
        let mut id_lock = ID_MANAGER
            .station_id
            .lock()
            .unwrap();
        let id = id_lock.clone();
        *id_lock += 1;
        LineID::from(id)
    }

    /// In the case that a line is created with a specific id, this function can
    /// be used to ensure the next sequential id is higher than this given id.
    pub fn update_line_id(id: LineID) {
        let mut id_lock = ID_MANAGER
            .line_id
            .lock()
            .unwrap();
        if u64::from(id) >= *id_lock {
            *id_lock = u64::from(id) + 1;
        }
    }

    /// Get the next edge id. This is the next sequential id and should
    /// therefore be unique.
    pub fn next_edge_id() -> EdgeID {
        let mut id_lock = ID_MANAGER
            .station_id
            .lock()
            .unwrap();
        let id = id_lock.clone();
        *id_lock += 1;
        EdgeID::from(id)
    }

    /// In the case that an edge is created with a specific id, this function
    /// can be used to ensure the next sequential id is higher than this given
    /// id.
    pub fn update_edge_id(id: EdgeID) {
        let mut id_lock = ID_MANAGER
            .edge_id
            .lock()
            .unwrap();
        if u64::from(id) >= *id_lock {
            *id_lock = u64::from(id) + 1;
        }
    }

    /// Convert the IDManager to a serializable struct.
    pub fn to_data() -> IDData {
        let station_id = *ID_MANAGER
            .station_id
            .lock()
            .unwrap();
        let line_id = *ID_MANAGER
            .line_id
            .lock()
            .unwrap();
        let edge_id = *ID_MANAGER
            .edge_id
            .lock()
            .unwrap();

        IDData {
            station_id,
            line_id,
            edge_id,
        }
    }

    /// Update the IDManager with the data from the serializable struct.
    pub fn from_data(data: IDData) {
        Self::update_station_id(
            data.station_id
                .into(),
        );
        Self::update_line_id(
            data.line_id
                .into(),
        );
        Self::update_edge_id(
            data.edge_id
                .into(),
        );
    }
}

/// A serializable struct that contains the data of the IDManager.
/// This can be used to save the state of the IDManager and transfer it between
/// the main thread and any web-workers.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IDData {
    station_id: u64,
    line_id: u64,
    edge_id: u64,
}
