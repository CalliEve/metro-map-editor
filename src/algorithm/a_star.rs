use std::{
    cmp::Ordering,
    collections::BinaryHeap,
};

use crate::models::GridNode;

/// Holds the state for an item in the A* algorithm queue.
#[derive(Clone)]
struct AStarState {
    cost: f64,
    node: GridNode,
    path_length: i32,
    parent: Option<Box<AStarState>>,
}

impl AStarState {
    /// Get the path that led to this state.
    fn to_path(self) -> Vec<GridNode> {
        let mut path = Vec::new();

        let mut current = Box::new(self);
        while let Some(state) = current.parent {
            path.push(state.node);
            current = state;
        }

        path.into_iter()
            .rev()
            .skip(1)
            .collect()
    }
}

// The priority queue depends on [`Ord`].
// This implement the trait so the queue becomes a min-heap
// instead of a max-heap, as a min-heap is needed for A*.
impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| {
                other
                    .path_length
                    .cmp(&self.path_length)
            })
    }
}

// [`PartialOrd`] needs to be implemented as well, as [`Ord`] requires it.
impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// They should be equal if their nodes are equal, as they then represent the
// same node.
impl PartialEq for AStarState {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

// [`Eq`] needs to be implemented as well, as [`Ord`] requires it.
impl Eq for AStarState {}

/// Run the A* algorithm to get the shortest path from the given from node to
/// the given to node.
pub fn run_a_star(from: GridNode, to: GridNode) -> Vec<GridNode> {
    let mut heap = BinaryHeap::with_capacity(from.diagonal_distance_to(to) as usize * 8);

    let init = AStarState {
        cost: 0.0,
        path_length: 0,
        node: from,
        parent: None,
    };
    let mut last = init.clone();
    heap.push(init);

    while let Some(
        current @ AStarState {
            node,
            path_length,
            ..
        },
    ) = heap.pop()
    {
        if node == to {
            last = current;
            break;
        }

        last = current.clone();
        for neighbor in node.get_neighbors() {
            let next = AStarState {
                path_length: path_length + 1,
                cost: path_length as f64 + neighbor.diagonal_distance_to(to),
                node: neighbor,
                parent: Some(Box::new(current.clone())),
            };

            heap.push(next);
        }
    }

    last.to_path()
}
