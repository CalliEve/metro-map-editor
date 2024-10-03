use std::{
    cmp::Reverse,
    collections::HashSet,
    hash::{
        Hash,
        Hasher,
    },
};

use ordered_float::NotNan;
use priority_queue::PriorityQueue;

use super::{
    cost_calculation::calc_node_cost,
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
    node: GridNode,
    path: Vec<GridNode>,
    start: GridNode,
}

impl QueueItem {
    /// Create a new [`QueueItem`] with the given node as the start of the path.
    fn new(node: GridNode) -> Self {
        Self {
            node,
            path: Vec::new(),
            start: node,
        }
    }

    /// Create a new [`QueueItem`] that grows from the given previous item.
    fn from_parent(parent: &QueueItem, node: GridNode) -> Self {
        let mut new = Self {
            node,
            path: parent
                .path
                .clone(),
            start: parent.start,
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
    occupied: &HashSet<GridNode>,
) -> Result<(GridNode, Vec<GridNode>, GridNode)> {
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut to_visited = None;
    let to_nodes = to
        .iter()
        .map(|(node, _)| *node)
        .collect::<HashSet<_>>();

    for (node, cost) in from {
        // FIXME: the cost is dependent upon the distance from the original station
        // location.
        queue.push(
            QueueItem::new(*node),
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

        if to_nodes.contains(&current.node) {
            // FIXME: this returns on first found from to set, not sure if we shouldn't
            // search for more or when it is enough then
            to_visited = Some((current.clone(), current_cost));
            break;
        }

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

            let cost = NotNan::new(calc_node_cost(
                settings,
                map,
                edge,
                neighbor,
                previous,
                from_station,
                to_station,
                occupied,
            )?)? + current_cost.0;

            let neighbor_item = QueueItem::from_parent(&current, neighbor);
            if let Some((_, old_cost)) = queue.get(&neighbor_item) {
                if old_cost.0 > cost {
                    queue.push_increase(neighbor_item, Reverse(cost));
                }
            } else {
                queue.push(neighbor_item, Reverse(cost));
            }
        }
    }

    if to_visited.is_none() {
        return Err(Error::other(format!(
            "No path found between {} and {}.",
            from_station.get_id(),
            to_station.get_id()
        )));
    }

    let mut best = to_visited
        .unwrap()
        .0;

    if best
        .path
        .len()
        > 1
    {
        best.path
            .drain(..1);
    } else {
        best.path
            .clear();
    }

    Ok((best.start, best.path, best.node))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_dijkstra() {
        let mut map = Map::new();
        let occupied = HashSet::new();
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
        );

        assert_eq!(
            result,
            Ok((
                GridNode::from((0, 0)),
                vec![
                    GridNode::from((1, 1)),
                    GridNode::from((2, 2)),
                    GridNode::from((3, 3)),
                    GridNode::from((4, 4)),
                    GridNode::from((5, 4)),
                    GridNode::from((6, 4))
                ],
                GridNode::from((7, 4))
            ))
        );
    }
}
