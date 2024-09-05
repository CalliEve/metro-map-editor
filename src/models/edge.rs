use std::sync::atomic::{
    AtomicU64,
    Ordering as AtomicOrdering,
};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{
    GridNode,
    LineID,
    Map,
    StationID,
};
use crate::{
    algorithm::{
        draw_edge,
        run_a_star,
    },
    components::CanvasState,
};

/// Next generated sequential identifier for a new edge.
static EDGE_ID: AtomicU64 = AtomicU64::new(1);

/// An identifier for an edge.
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeID(u64);

impl From<u64> for EdgeID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// Represents an edge, which is the connection between two stations.
#[derive(Clone, Debug)]
pub struct Edge {
    /// ID of the edge
    id: EdgeID,
    /// Start of the edge
    from: StationID,
    /// End of the edge
    to: StationID,
    /// Nodes visited between the stations
    nodes: Vec<GridNode>,
    /// Lines that use this edge
    lines: Vec<LineID>,
}

impl Edge {
    /// Creates a new edge with start and goal.
    pub fn new(from: StationID, to: StationID, id: Option<EdgeID>) -> Self {
        Self {
            from,
            to,
            id: id.unwrap_or_else(|| {
                EDGE_ID
                    .fetch_add(1, AtomicOrdering::SeqCst)
                    .into()
            }),
            nodes: Vec::new(),
            lines: Vec::new(),
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

    /// A setter for the lines that use this edge.
    pub fn set_lines(&mut self, mut lines: Vec<LineID>) {
        lines.sort_unstable();
        self.lines = lines;
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
                    .insert(pos, line)
            },
        }
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

    /// Get the stations bordering the node on this edge if exists
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

    /// Recalculates the nodes between the stations.
    pub fn calculate_nodes(&mut self, map: &Map) {
        let from = map
            .get_station(self.get_from())
            .expect("invalid station id");
        let to = map
            .get_station(self.get_to())
            .expect("invalid station id");

        self.nodes = run_a_star(from.get_pos(), to.get_pos());
    }

    pub fn draw(&self, map: &Map, canvas: &CanvasRenderingContext2d, state: CanvasState) {
        let from = map
            .get_station(self.get_from())
            .expect("invalid station id when drawing");
        let to = map
            .get_station(self.get_to())
            .expect("invalid station id when drawing");

        let colors = self
            .lines
            .iter()
            .filter_map(|l| map.get_line(*l))
            .map(|l| l.get_color())
            .collect::<Vec<_>>();

        let color_count = colors.len();
        for (i, color) in colors
            .into_iter()
            .enumerate()
        {
            let mut width = state.drawn_square_size() / 10.0 + 0.5;
            if width < 1.0 {
                width = 1.0
            }

            canvas.set_line_width(width);
            canvas.set_global_alpha(1.0);
            canvas.set_stroke_style(&JsValue::from_str(&format!(
                "rgb({} {} {})",
                color.0, color.1, color.2,
            )));
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
    }
}
