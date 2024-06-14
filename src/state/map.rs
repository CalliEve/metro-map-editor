use leptos::html::Canvas;
use leptos::*;

use crate::algorithm::{redraw_canvas, Map, Station};

#[derive(Clone, Debug)]
pub struct MapState {
    map: Option<Map>,
    selected_station: Option<Station>,
    size: (u32, u32),
    square_size: u32,
}

impl MapState {
    pub fn new(map: Map) -> Self {
        Self {
            map: Some(map),
            selected_station: None,
            size: (300, 300),
            square_size: 30,
        }
    }

    pub fn get_map(&self) -> Option<&Map> {
        self.map.as_ref()
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = Some(map);
    }

    pub fn clear_map(&mut self) {
        self.map = None;
    }

    pub fn get_selected_station(&self) -> Option<&Station> {
        self.selected_station.as_ref()
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

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub fn get_square_size(&self) -> u32 {
        self.square_size
    }

    pub fn set_square_size(&mut self, size: u32) {
        self.square_size = size;
    }

    pub fn draw_to_canvas(&self, canvas_ref: &NodeRef<Canvas>) {
        redraw_canvas(&canvas_ref.get().expect("should be loaded now"), self);
    }
}
