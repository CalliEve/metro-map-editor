use crate::algorithm::{Map, Station};

#[derive(Clone, Debug, Default)]
pub struct MapState {
    map: Option<Map>,
    selected_station: Option<Station>,
}

impl MapState {
    pub fn new(map: Map) -> Self {
        Self {
            map: Some(map),
            selected_station: None,
        }
    }

    pub fn get_map(&self) -> &Option<Map> {
        &self.map
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = Some(map);
    }

    pub fn clear_map(&mut self) {
        self.map = None;
    }

    pub fn get_selected_station(&self) -> &Option<Station> {
        &self.selected_station
    }

    pub fn set_selected_station(&mut self, station: Station) {
        self.selected_station = Some(station);
    }

    pub fn clear_selected_station(&mut self) {
        self.selected_station = None;
    }

    pub fn has_selected_station(&self) -> bool {
        self.selected_station.is_some()
    }
}
