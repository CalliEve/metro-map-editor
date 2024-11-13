//! Contains the [`Edge`] struct and all its methods.
use std::fmt::{
    self,
    Display,
    Formatter,
};

use leptos::logging;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    GridNode,
    Line,
    LineID,
    Map,
    StationID,
};
use crate::{
    algorithm::{
        drawing::{
            calc_label_pos,
            draw_edge,
            CanvasContext,
        },
        run_a_star,
    },
    components::CanvasState,
    utils::IDManager,
};

/// An identifier for an edge.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EdgeID(u64);

impl From<u64> for EdgeID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<EdgeID> for u64 {
    fn from(value: EdgeID) -> Self {
        value.0
    }
}

impl Display for EdgeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents an edge, which is the connection between two stations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    /// ID of the edge.
    id: EdgeID,
    /// Start of the edge.
    from: StationID,
    /// End of the edge.
    to: StationID,
    /// Nodes visited between the stations.
    nodes: Vec<GridNode>,
    /// Lines that use this edge.
    lines: Vec<LineID>,
    /// If the edge is settled in the Dijkstra algorithm.
    is_settled: bool,
    /// If the edge is locked into its current shape by the user.
    is_locked: bool,
    /// If the edge is selected by the user.
    is_selected: bool,
    /// The stations contracted into this line in the algorithm.
    contracted_stations: Vec<StationID>,
}

impl Edge {
    /// Creates a new edge with start and goal.
    pub fn new(from: StationID, to: StationID, id: Option<EdgeID>) -> Self {
        Self {
            from,
            to,
            id: id.unwrap_or_else(IDManager::next_edge_id),
            nodes: Vec::new(),
            lines: Vec::new(),
            is_settled: false,
            is_locked: false,
            is_selected: false,
            contracted_stations: Vec::new(),
        }
    }

    /// Returns the id of the edge.
    #[inline]
    pub fn get_id(&self) -> EdgeID {
        self.id
    }

    /// Returns true if the given station id is the edge start.
    pub fn is_from(&self, id: StationID) -> bool {
        self.from == id
    }

    /// Returns true if the given station id is the edge goal.
    pub fn is_to(&self, id: StationID) -> bool {
        self.to == id
    }

    /// Get the id of the edge start.
    #[inline]
    pub fn get_from(&self) -> StationID {
        self.from
    }

    /// Get the id of the edge goal.
    #[inline]
    pub fn get_to(&self) -> StationID {
        self.to
    }

    /// Get the other end of the edge from the station given, returns None if
    /// the station is not an end of the edge.
    pub fn opposite(&self, station: StationID) -> Option<StationID> {
        if self.from == station {
            Some(self.to)
        } else if self.to == station {
            Some(self.from)
        } else {
            None
        }
    }

    /// A setter for the lines that use this edge.
    pub fn set_lines(&mut self, mut lines: Vec<LineID>) {
        lines.sort_unstable();
        self.lines = lines;
    }

    /// A getter for the lines that use this edge.
    pub fn get_lines(&self) -> &[LineID] {
        &self.lines
    }

    /// Add a line to the lines using the edge if it didn't already exist
    pub fn add_line(&mut self, line: LineID) {
        match self
            .lines
            .binary_search(&line)
        {
            Ok(_) => {},
            Err(pos) => {
                self.lines
                    .insert(pos, line);
            },
        }
    }

    /// Remove a line from the lines using the edge if it exists
    pub fn remove_line(&mut self, line: LineID) {
        if let Ok(pos) = self
            .lines
            .binary_search(&line)
        {
            self.lines
                .remove(pos);
        }
    }

    /// A getter for the nodes visited between the stations.
    pub fn get_nodes(&self) -> &[GridNode] {
        &self.nodes
    }

    /// A setter for the nodes visited between the stations.
    pub fn set_nodes(&mut self, nodes: Vec<GridNode>) {
        self.nodes = nodes;
    }

    /// Get the start and end nodes of the edge.
    pub fn get_edge_ends(&self) -> Vec<GridNode> {
        if self
            .get_nodes()
            .len()
            < 3
        {
            self.get_nodes()
                .to_vec()
        } else {
            vec![
                self.get_nodes()[0],
                self.get_nodes()[self
                    .get_nodes()
                    .len()
                    - 1],
            ]
        }
    }

    /// A getter for if the edge is settled.
    #[inline]
    pub fn is_settled(&self) -> bool {
        self.is_settled || self.is_locked()
    }

    /// Settle the edge.
    pub fn settle(&mut self) {
        self.is_settled = true;
    }

    /// Unsettle the edge.
    pub fn unsettle(&mut self) {
        self.is_settled = false;
    }

    /// A getter for if the edge is locked.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Lock the edge.
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlock the edge.
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// A getter for if the edge is selected.
    #[inline]
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    /// Select the edge.
    pub fn select(&mut self) {
        self.is_selected = true;
    }

    /// Unselect the edge.
    pub fn deselect(&mut self) {
        self.is_selected = false;
    }

    /// Add a station to the contracted stations.
    pub fn add_contracted_station(&mut self, station: StationID) {
        self.contracted_stations
            .push(station);
    }

    /// Extend the contracted stations with the given stations.
    pub fn extend_contracted_stations(&mut self, stations: &[StationID]) {
        self.contracted_stations
            .extend(stations);
    }

    /// Get the contracted stations.
    pub fn get_contracted_stations(&self) -> &[StationID] {
        &self.contracted_stations
    }

    /// Clear the contracted stations.
    pub fn clear_contracted_stations(&mut self) {
        self.contracted_stations
            .clear();
    }

    /// Returns if the edge visits the node.
    pub fn visits_node(&self, map: &Map, node: GridNode) -> bool {
        if self
            .nodes
            .contains(&node)
        {
            return true;
        }

        if let Some(from) = map.get_station(self.get_from()) {
            if from.get_pos() == node {
                return true;
            }
        }

        if let Some(to) = map.get_station(self.get_to()) {
            if to.get_pos() == node {
                return true;
            }
        }

        false
    }

    #[allow(dead_code)]
    pub fn print_info(&self) {
        logging::log!(
            "Edge: {} from {} to {} with lines [{:?}]",
            self.id,
            self.get_from(),
            self.get_to(),
            self.get_lines()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    /// Get the stations bordering the node on this edge if exists
    #[allow(clippy::unnecessary_wraps)]
    pub fn get_neigboring_stations(
        &self,
        map: &Map,
        node: GridNode,
    ) -> Option<(Option<StationID>, Option<StationID>)> {
        if self
            .nodes
            .contains(&node)
        {
            return Some((
                Some(self.get_from()),
                Some(self.get_to()),
            ));
        }

        if let Some(from) = map.get_station(self.get_from()) {
            if from
                .get_pos()
                .get_neighbors()
                .contains(&node)
            {
                return Some((None, Some(self.get_from())));
            }
        }

        if let Some(to) = map.get_station(self.get_to()) {
            if to
                .get_pos()
                .get_neighbors()
                .contains(&node)
            {
                return Some((Some(self.get_to()), None));
            }
        }

        Some((None, None))
    }

    /// Recalculates the nodes between the stations using the A* algorithm.
    pub fn calculate_nodes(&mut self, map: &Map) {
        let from = map
            .get_station(self.get_from())
            .expect("invalid station id");
        let to = map
            .get_station(self.get_to())
            .expect("invalid station id");

        self.set_nodes(run_a_star(from.get_pos(), to.get_pos()));
    }

    /// Draw the edge to the given canvas.
    #[allow(clippy::too_many_lines)]
    pub fn draw(&self, map: &Map, canvas: &CanvasContext<'_>, state: CanvasState, base_alpha: f64) {
        let from = map
            .get_station(self.get_from())
            .expect("invalid from station id when drawing");
        let to = map
            .get_station(self.get_to())
            .expect("invalid to station id when drawing");

        // Highlight if selected
        if self.is_selected() {
            let mut selected_width = state.drawn_square_size() / 3.0;
            if selected_width < 2.0 {
                selected_width = 2.0;
            }

            canvas.set_line_width(selected_width);
            canvas.set_global_alpha(0.2);

            canvas.set_stroke_style_str("darkblue");
            canvas.begin_path();

            draw_edge(
                from.get_pos(),
                to.get_pos(),
                &self.nodes,
                canvas,
                state,
                0.0,
            );

            canvas.stroke();
        }

        let colors = self
            .lines
            .iter()
            .filter_map(|l| map.get_line(*l))
            .map(Line::get_color)
            .collect::<Vec<_>>();

        let mut width = state.drawn_square_size() / 10.0 + 0.5;
        if width < 1.0 {
            width = 1.0;
        }

        let color_count = colors.len();
        for (i, color) in colors
            .into_iter()
            .enumerate()
        {
            canvas.set_line_width(width);
            canvas.set_global_alpha(1.0 * base_alpha);

            canvas.set_stroke_style_str(&format!(
                "rgb({} {} {})",
                color.0, color.1, color.2,
            ));
            canvas.begin_path();

            let color_offset = if color_count == 1 {
                0.0
            } else {
                ((i as f64) * width) - ((color_count as f64 * width) / 2.0) + (width / 2.0)
            };

            draw_edge(
                from.get_pos(),
                to.get_pos(),
                &self.nodes,
                canvas,
                state,
                color_offset,
            );

            canvas.stroke();
        }

        // Add lock icon if locked
        if self.is_locked() {
            let first_pos = if let Some(first_node) = self
                .get_nodes()
                .first()
            {
                first_node.to_canvas_pos(state)
            } else {
                return;
            };

            let second_pos = if let Some(second_node) = self
                .get_nodes()
                .get(1)
            {
                second_node.to_canvas_pos(state)
            } else {
                to.get_canvas_pos(state)
            };

            let offset = ((self
                .lines
                .len() as f64)
                * width)
                - ((color_count as f64 * width) / 2.0)
                + (width / 2.0);
            let locked_label_pos = calc_label_pos(
                state,
                first_pos,
                Some(second_pos),
                Some(offset),
            )[0]; // FIXME: Check for occupancy

            canvas.set_stroke_style_str("grey");
            canvas.begin_path();
            canvas
                .arc(
                    locked_label_pos.0,
                    locked_label_pos.1,
                    state.drawn_square_size() / 3.0 / 5.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                )
                .unwrap();
            canvas.set_fill_style_str("grey");
            canvas.fill();
            canvas.stroke();
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Station;

    #[test]
    fn test_draw() {
        let mut map = Map::new();
        let canvas = CanvasContext::new();
        let mut state = CanvasState::new();
        state.set_square_size(5);
        state.set_size((100, 100));
        assert_eq!(state.drawn_square_size(), 5.0);

        let mut line1 = Line::new(None);
        line1.set_color((255, 1, 1));
        let mut line2 = Line::new(None);
        line2.set_color((1, 255, 1));

        let from = Station::new((0, 0).into(), None);
        let to = Station::new((3, 3).into(), None);
        let mut edge = Edge::new(from.get_id(), to.get_id(), None);

        map.add_station(from);
        map.add_station(to);

        edge.set_lines(vec![line1.get_id(), line2.get_id()]);
        map.add_line(line1);
        map.add_line(line2);

        edge.calculate_nodes(&map);
        edge.draw(&map, &canvas, state, 1.0);

        assert_eq!(
            canvas.get_record("move_to"),
            Some(vec![
                "1.8,0.8".to_owned(),
                "0.8,1.8".to_owned(),
            ])
        );

        assert_eq!(
            canvas.get_record("line_to"),
            Some(vec![
                "5.5,4.5".to_owned(),
                "10.5,9.5".to_owned(),
                "14.2,13.2".to_owned(),
                "4.5,5.5".to_owned(),
                "9.5,10.5".to_owned(),
                "13.2,14.2".to_owned()
            ])
        );
    }
}
