//! Contains the implementation of the Dijkstra algorithm for tracing an edge
//! between two sets of possible station locations.

use std::{
    cmp::Reverse,
    collections::{
        HashMap,
        HashSet,
    },
    hash::{
        Hash,
        Hasher,
    },
};

use ordered_float::NotNan;
use priority_queue::PriorityQueue;

use super::{
    cost_calculation::calc_node_cost,
    occupation::OccupiedNodes,
    AlgorithmSettings,
};
use crate::{
    models::{
        Edge,
        GridNode,
        Map,
        Station,
    },
    utils::Result,
    Error,
};

/// Holds the state for an item in the Dijkstra algorithm queue.
#[derive(Clone, Debug)]
struct QueueItem {
    /// The current node.
    node: GridNode,
    /// The path to the current node.
    path: Vec<GridNode>,
    /// The cost of the path so far.
    cost: NotNan<f64>,
}

impl QueueItem {
    /// Create a new [`QueueItem`] with the given node as the start of the path.
    fn new(node: GridNode, cost: NotNan<f64>) -> Self {
        Self {
            node,
            path: Vec::new(),
            cost,
        }
    }

    /// Create a new [`QueueItem`] that grows from the given previous item.
    fn from_parent(parent: &QueueItem, node: GridNode, cost: NotNan<f64>) -> Self {
        let mut new = Self {
            node,
            path: parent
                .path
                .clone(),
            cost: parent.cost + cost,
        };
        new.path
            .push(parent.node);
        new
    }
}

/// The equality only depends on the current node.
impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for QueueItem {}

/// The hash only depends on the current node.
impl Hash for QueueItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node
            .hash(state);
    }
}

/// A Dijkstra implementation that finds the shortest path between two start and
/// end node sets. This is the Edge Dijkstra algorithm in the paper.
pub fn edge_dijkstra(
    settings: AlgorithmSettings,
    map: &Map,
    edge: &Edge,
    from: &[(GridNode, f64)],
    from_station: &Station,
    to: &[(GridNode, f64)],
    to_station: &Station,
    occupied: &OccupiedNodes,
) -> Result<(
    GridNode,
    Vec<GridNode>,
    GridNode,
    NotNan<f64>,
)> {
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut to_visited = Vec::new();
    let to_nodes = to
        .iter()
        .copied()
        .collect::<HashMap<GridNode, f64>>();

    for (node, cost) in from {
        // FIXME: the cost is dependent upon the distance from the original station
        // location.
        queue.push(
            QueueItem::new(*node, NotNan::new(*cost)?),
            Reverse(NotNan::new(*cost)?),
        );
    }

    while let Some((current, current_cost)) = queue.pop() {
        visited.insert(current.node);

        if current_cost
            .0
            .is_infinite()
        {
            break;
        }

        // Check if the current node is in the to_nodes set and if so, add it to the
        // list of to_nodes we have visited so far. We are done once we have visited all
        // to_nodes.
        if let Some(to_cost) = to_nodes.get(&current.node) {
            to_visited.push((current.clone(), current.cost + to_cost));
            if to_visited.len() == to_nodes.len() {
                break;
            }
        }

        // The up-to two last nodes in the path so far.
        let previous = &current
            .path
            .last()
            .map_or(vec![current.node], |p| {
                vec![*p, current.node]
            });

        for neighbor in current
            .node
            .get_neighbors()
        {
            if visited.contains(&neighbor) {
                continue;
            }

            // Calculate the cost of the node and enforce it's not NaN.
            let cost = NotNan::new(calc_node_cost(
                settings,
                map,
                edge,
                neighbor,
                previous,
                from_station,
                to_station,
                occupied,
            )?)?;

            // Don't even look at nodes that have infinite cost.
            // CHECKME: this is a bit of a hack, we should probably add them to the queue
            // and handle the break when we encounter the first, but this causes
            // a bug?
            if cost.is_infinite() {
                continue;
            }

            // Add the heuristic cost to the cost for the queue.
            let cost_with_heuristic = cost + neighbor.diagonal_distance_to(from_station.get_pos());

            let neighbor_item = QueueItem::from_parent(&current, neighbor, cost);
            if let Some((_, old_cost)) = queue.get(&neighbor_item) {
                if old_cost.0 > cost_with_heuristic {
                    queue.push(
                        neighbor_item,
                        Reverse(cost_with_heuristic),
                    );
                }
            } else {
                queue.push(
                    neighbor_item,
                    Reverse(cost_with_heuristic),
                );
            }
        }
    }

    if to_visited.is_empty() {
        return Err(Error::other(format!(
            "No path found between {} and {}.",
            from_station.get_id(),
            to_station.get_id()
        )));
    }

    // Get the cheapest path found.
    let mut best = to_visited
        .into_iter()
        .min_by_key(|(_, c)| *c)
        .unwrap()
        .0;

    // Removes the first node from the path as this is the starting station location
    let start = if best
        .path
        .is_empty()
    {
        return Err(Error::other(format!(
            "Path is empty, start and end are equal on edge {} from {} to {}",
            edge.get_id(),
            from_station.get_id(),
            to_station.get_id()
        )));
    } else {
        best.path
            .drain(..1)
            .next()
            .expect("path is empty")
    };

    Ok((start, best.path, best.node, best.cost))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_dijkstra() {
        let mut map = Map::new();
        let occupied = HashMap::new();
        let from_station = Station::new(GridNode::from((0, 0)), None);
        let from_nodes = vec![(from_station.get_pos(), 0.0)];
        let to_station = Station::new(GridNode::from((8, 4)), None);
        let to_nodes = vec![
            (GridNode::from((7, 4)), 1.0),
            (to_station.get_pos(), 0.0),
            (GridNode::from((9, 4)), 1.0),
        ];
        let edge = Edge::new(
            from_station.get_id(),
            to_station.get_id(),
            None,
        );

        map.add_station(from_station.clone());
        map.add_station(to_station.clone());
        map.add_edge(edge.clone());
        map.quickcalc_edges();

        let result = edge_dijkstra(
            AlgorithmSettings::default(),
            &map,
            &edge,
            &from_nodes,
            &from_station,
            &to_nodes,
            &to_station,
            &occupied,
        )
        .unwrap();

        assert_eq!(
            (result.0, result.1, result.2),
            (
                GridNode::from((0, 0)),
                vec![
                    GridNode::from((1, 0)),
                    GridNode::from((2, 0)),
                    GridNode::from((3, 0)),
                    GridNode::from((4, 1)),
                    GridNode::from((5, 2)),
                    GridNode::from((6, 3))
                ],
                GridNode::from((7, 4))
            )
        );
    }
}
