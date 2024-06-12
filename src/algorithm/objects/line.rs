use wasm_bindgen::JsValue;
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

    pub fn get_mut_stations(&mut self) -> &mut [Station] {
        &mut self.stations
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let offset = square_size as f64 / 3.0 - 2.0;
        let stations = self.get_stations();

        canvas.set_line_width(2.0);
        canvas.set_stroke_style(&JsValue::from_str("black"));
        canvas.begin_path();

        if stations.len() > 1 {
            for (start_station, end_station) in stations.iter().zip(stations.iter().skip(1)) {
                let (from_x, from_y) =
                    station_corner_closest(start_station, end_station, square_size, offset);
                canvas.move_to(from_x, from_y);

                let (to_x, to_y) =
                    station_corner_closest(end_station, start_station, square_size, offset);
                canvas.line_to(to_x, to_y);
            }
        } else if stations.len() == 1 {
            let (station_x, station_y) = stations[0].get_canvas_pos(square_size);

            canvas.move_to(station_x - offset, station_y);
            canvas.line_to(station_x - (square_size as f64 - offset), station_y);

            canvas.move_to(station_x + offset, station_y);
            canvas.line_to(station_x + (square_size as f64 - offset), station_y);
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
    } else {
        if station_y == neighbor_y {
            (station_x + offset, station_y) // right
        } else if station_y > neighbor_y {
            (station_x + offset, station_y - offset) // below right
        } else {
            (station_x + offset, station_y + offset) // above right
        }
    }
}
