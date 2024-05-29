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

    pub fn add_line(&mut self, line: Line) {
        for station in line.get_stations().iter() {
            if !self.stations.contains(&station) {
                self.stations.push(station.clone())
            }
        }

        self.lines.push(line);
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &web_sys::CanvasRenderingContext2d, square_size: u32) {
        for line in self.lines.iter() {
            line.draw(canvas, square_size)
        }
    }
}
