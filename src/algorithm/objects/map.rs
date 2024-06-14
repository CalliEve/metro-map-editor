use super::{Drawable, Line, Station};

#[derive(Clone, Debug)]
pub struct Map {
    stations: Vec<Station>,
    lines: Vec<Line>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn get_station(&self, id: &str) -> Option<&Station> {
        self.stations.iter().find(|s| s.get_id() == id)
    }

    pub fn get_mut_line(&mut self, id: &str) -> Option<&mut Line> {
        self.lines.iter_mut().find(|l| l.get_id() == id)
    }

    pub fn add_line(&mut self, mut line: Line) {
        for station in line.get_mut_stations() {
            if let Some(existing) = self.stations.iter().find(|s| s == &station) {
                *station = existing.clone()
            } else {
                self.stations.push(station.clone())
            }
        }

        self.lines.push(line);
    }

    pub fn get_stations(&self) -> &[Station] {
        &self.stations
    }

    pub fn get_mut_stations(&mut self) -> &mut [Station] {
        &mut self.stations
    }

    pub fn add_station(&mut self, station: Station) {
        self.stations.push(station);
    }

    pub fn station_at_pos(&self, pos: (i32, i32)) -> Option<&Station> {
        self.stations.iter().find(|s| s.get_pos() == pos)
    }

    pub fn calc_nearest_grid_node(&self, square_size: u32, pos: (i32, i32)) -> (i32, i32) {
        (
            (pos.0 as f64 / square_size as f64).round() as i32,
            (pos.1 as f64 / square_size as f64).round() as i32,
        )
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, square_size: u32) {
        for station in self.get_stations() {
            station.draw(canvas, square_size)
        }
        for line in self.lines.iter() {
            line.draw(canvas, square_size)
        }
    }
}
