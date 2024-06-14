use std::cmp::Ordering;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{Drawable, Station};

#[derive(Clone, Debug)]
pub struct Line {
    id: String,
    name: String,
    color: (u8, u8, u8),
    stations: Vec<Station>,
}

impl Line {
    pub fn new(stations: Vec<Station>, id: impl ToString) -> Self {
        Self {
            stations,
            id: id.to_string(),
            color: (0, 0, 0),
            name: String::new(),
        }
    }

    pub fn get_stations(&self) -> &[Station] {
        &self.stations
    }

    pub fn get_mut_stations(&mut self) -> &mut [Station] {
        &mut self.stations
    }

    pub fn add_station(&mut self, station: Station) {
        if self.stations.contains(&station) {
            return;
        }

        self.stations.push(station);
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = name.to_string();
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let offset = square_size as f64 / 3.0 - 2.0;
        let stations = self.get_stations();

        canvas.set_line_width(2.0);
        canvas.set_stroke_style(&JsValue::from_str("black"));
        canvas.begin_path();

        match stations.len().cmp(&1) {
            Ordering::Greater => {
                for (start_station, end_station) in stations.iter().zip(stations.iter().skip(1)) {
                    let (from_x, from_y) =
                        station_corner_closest(start_station, end_station, square_size, offset);
                    canvas.move_to(from_x, from_y);

                    let (to_x, to_y) =
                        station_corner_closest(end_station, start_station, square_size, offset);
                    canvas.line_to(to_x, to_y);
                }
            }
            Ordering::Equal => {
                let (station_x, station_y) = stations[0].get_canvas_pos(square_size);

                canvas.move_to(station_x - offset, station_y);
                canvas.line_to(station_x - (square_size as f64 - offset), station_y);

                canvas.move_to(station_x + offset, station_y);
                canvas.line_to(station_x + (square_size as f64 - offset), station_y);
            }
            _ => {}
        }

        canvas.stroke();
    }
}

fn station_corner_closest(
    station: &Station,
    neighbor: &Station,
    square_size: u32,
    offset: f64,
) -> (f64, f64) {
    let (station_x, station_y) = station.get_canvas_pos(square_size);
    let (neighbor_x, neighbor_y) = neighbor.get_canvas_pos(square_size);

    if station_x == neighbor_x {
        if station_y > neighbor_y {
            (station_x, station_y - offset) // below
        } else {
            (station_x, station_y + offset) // above
        }
    } else if station_x > neighbor_x {
        if station_y == neighbor_y {
            (station_x - offset, station_y) // left
        } else if station_y > neighbor_y {
            (station_x - offset, station_y - offset) // below left
        } else {
            (station_x - offset, station_y + offset) // above left
        }
    } else if station_y == neighbor_y {
        (station_x + offset, station_y) // right
    } else if station_y > neighbor_y {
        (station_x + offset, station_y - offset) // below right
    } else {
        (station_x + offset, station_y + offset) // above right
    }
}
