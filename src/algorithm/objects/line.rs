use web_sys::CanvasRenderingContext2d;

use super::{Drawable, Station};

#[derive(Clone, Debug)]
pub struct Line {
    stations: Vec<Station>,
}

impl Line {
    pub fn new(stations: Vec<Station>) -> Self {
        Self { stations }
    }

    pub fn get_stations(&self) -> &[Station] {
        &self.stations
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        for station in self.stations.iter() {
            station.draw(canvas, square_size)
        }
    }
}
