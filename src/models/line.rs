//! Contains the [`Line`] struct and all its methods.

use std::cmp::Ordering;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{
    Drawable,
    Station,
};
use crate::utils::equal_pixel;

/// Represents a metro line, including its stations, name and color.
#[derive(Clone, Debug)]
pub struct Line {
    /// ID of the line.
    id: String,
    /// Name of the line.
    name: String,
    /// Color of the line.
    color: (u8, u8, u8),
    /// All stations the line visits.
    stations: Vec<Station>,
}

impl Line {
    /// Create a new [`Line`] with the stations it visits and an identifier.
    /// Color and name are set to default values.
    pub fn new(stations: Vec<Station>, id: &impl ToString) -> Self {
        Self {
            stations,
            id: id.to_string(),
            color: (0, 0, 0),
            name: String::new(),
        }
    }

    /// A getter method for the stations the line visits.
    pub fn get_stations(&self) -> &[Station] {
        &self.stations
    }

    /// A mutable getter method for the stations the line visits.
    pub fn get_mut_stations(&mut self) -> &mut [Station] {
        &mut self.stations
    }

    /// Add a station after the after station, or at the end of the line.
    /// If after isn't in the line yet, it will add both.
    pub fn add_station(&mut self, station: Station, after: Option<&Station>) {
        if let Some(index) = after.and_then(|a| {
            self.stations
                .iter()
                .position(|s| s == a)
        }) {
            // found after and will insert station after after
            self.stations
                .insert(index + 1, station);
            return;
        } else if let Some(a) = after.cloned() {
            // after exists but not found, so inserting it at the end
            self.stations
                .push(a);
        }

        self.stations
            .push(station);
    }

    /// A setter for the station's color.
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    /// A getter for the station's color.
    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    /// A setter for the station's name.
    pub fn set_name(&mut self, name: &impl ToString) {
        self.name = name.to_string();
    }

    /// A getter for the station's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// A getter for the station id.
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &CanvasRenderingContext2d, square_size: u32) {
        let offset = f64::from(square_size) / 3.0 - 2.0;
        let stations = self.get_stations();

        canvas.set_line_width(2.0);
        canvas.set_stroke_style(&JsValue::from_str(&format!(
            "rgb({} {} {})",
            self.color
                .0,
            self.color
                .1,
            self.color
                .2
        )));
        canvas.begin_path();

        match stations
            .len()
            .cmp(&1)
        {
            // Draw a line between each two sequential stations on the line.
            Ordering::Greater => {
                for (start_station, end_station) in stations
                    .iter()
                    .zip(
                        stations
                            .iter()
                            .skip(1),
                    )
                {
                    let (from_x, from_y) = station_corner_closest(
                        start_station,
                        end_station,
                        square_size,
                        offset,
                    );
                    canvas.move_to(from_x, from_y);

                    let (to_x, to_y) = station_corner_closest(
                        end_station,
                        start_station,
                        square_size,
                        offset,
                    );
                    canvas.line_to(to_x, to_y);
                }
            },
            // Add two horizontal lines to the single station, showing its a lone station on the
            // line.
            Ordering::Equal => {
                let (station_x, station_y) = stations[0].get_canvas_pos(square_size);

                canvas.move_to(station_x - offset, station_y);
                canvas.line_to(
                    station_x - (f64::from(square_size) - offset),
                    station_y,
                );

                canvas.move_to(station_x + offset, station_y);
                canvas.line_to(
                    station_x + (f64::from(square_size) - offset),
                    station_y,
                );
            },
            // Empty line means nothing to draw
            Ordering::Less => {},
        }

        canvas.stroke();
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        other.id == self.id
    }
}

/// Calculates the coordinate of the corner (on an octilinear grid) of a station
/// closest to the given neighbor. An offset is provided for if the corner is
/// further from the middle of the station coordinate.
fn station_corner_closest(
    station: &Station,
    neighbor: &Station,
    square_size: u32,
    offset: f64,
) -> (f64, f64) {
    let (station_x, station_y) = station.get_canvas_pos(square_size);
    let (neighbor_x, neighbor_y) = neighbor.get_canvas_pos(square_size);

    if equal_pixel(station_x, station_y) {
        if station_y > neighbor_y {
            (station_x, station_y - offset) // below
        } else {
            (station_x, station_y + offset) // above
        }
    } else if station_x > neighbor_x {
        if equal_pixel(station_y, neighbor_y) {
            (station_x - offset, station_y) // left
        } else if station_y > neighbor_y {
            (station_x - offset, station_y - offset) // below left
        } else {
            (station_x - offset, station_y + offset) // above left
        }
    } else if equal_pixel(station_y, neighbor_y) {
        (station_x + offset, station_y) // right
    } else if station_y > neighbor_y {
        (station_x + offset, station_y - offset) // below right
    } else {
        (station_x + offset, station_y + offset) // above right
    }
}
