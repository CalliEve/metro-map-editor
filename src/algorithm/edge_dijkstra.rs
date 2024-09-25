use std::{
    cmp::Reverse,
    hash::{
        Hash,
        Hasher,
    },
};

use priority_queue::PriorityQueue;

use crate::{
    models::{
        GridNode,
        Map,
    },
    utils::Result,
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
            .push(node);
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
    map: &Map,
    from: Vec<GridNode>,
    to: Vec<GridNode>,
) -> Result<(GridNode, Vec<GridNode>, GridNode)> {
    let mut queue = PriorityQueue::new();
    let mut visited = Vec::new();
    let mut to_visited = Vec::new();

    for node in from {
        // FIXME: the cost is dependent upon the distance from the original station
        // location.
        queue.push(QueueItem::new(node), Reverse(0));
    }

    while let Some((current, current_cost)) = queue.pop() {
        visited.push(current.node);

        if to.contains(&current.node) {
            to_visited.push((current.clone(), current_cost));
        }
        if to_visited.len() == to.len() {
            break;
        }

        for neighbor in current
            .node
            .get_neighbors()
        {
            if visited.contains(&neighbor) {
                continue;
            }

            let cost = todo!("calculate the cost");

            let neighbor_item = QueueItem::from_parent(&current, neighbor);
            if let Some((_, old_cost)) = queue.get(&neighbor_item) {
                if old_cost.0 > cost {
                    queue.push_increase(neighbor_item, Reverse(cost));
                }
                todo!("update path if needed");
            } else {
                queue.push(neighbor_item, Reverse(cost));
                todo!("set path if needed");
            }
        }
    }

    // Get the node from the to node set with the cheapest path and return it.
    to_visited.sort_unstable_by_key(|(_, cost)| cost.0);
    let mut best = to_visited[0]
        .0
        .clone();
    best.path
        .truncate(
            best.path
                .len()
                - 1,
        );

    Ok((best.start, best.path, best.node))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_dijkstra() {
        let map = Map::new();
        let from = vec![GridNode::from((0, 0))];
        let to = vec![GridNode::from((3, 3))];

        let result = edge_dijkstra(&map, from, to);

        assert_eq!(
            result,
            Ok((
                GridNode::from((0, 0)),
                vec![
                    GridNode::from((1, 1)),
                    GridNode::from((2, 2))
                ],
                GridNode::from((3, 3))
            ))
        );
    }
}
