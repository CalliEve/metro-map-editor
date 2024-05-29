use super::{Drawable, Line, Station};

#[derive(Clone, Debug)]
pub struct Map {
    stations: Vec<Station>,
    lines: Vec<Line>,
    square_size: u32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
            lines: Vec::new(),
            square_size: 30,
        }
    }

    pub fn get_square_size(&self) -> u32 {
        self.square_size
    }

    pub fn set_square_size(&mut self, square_size: u32) {
        self.square_size = square_size;
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

    pub fn station_at_pos(&self, pos: (i32, i32)) -> Option<&Station> {
        self.stations.iter().find(|s| s.get_pos() == pos)
    }

    pub fn calc_nearest_grid_node(&self, pos: (i32, i32)) -> (i32, i32) {
        (
            (pos.0 as f64 / self.square_size as f64).round() as i32,
            (pos.1 as f64 / self.square_size as f64).round() as i32,
        )
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, square_size: u32) {
        for line in self.lines.iter() {
            line.draw(canvas, square_size)
        }
    }
}
