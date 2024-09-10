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
    path_length: f64,
    parent: Option<Box<AStarState>>,
}

impl AStarState {
    /// Get the path that led to this state.
    ///
    /// Note: This method excludes the starting node of the path and the node of
    /// the current state.
    fn to_path(&self) -> Vec<GridNode> {
        let mut path = Vec::new();

        let mut current = Box::new(self.clone());
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
                    .total_cmp(&self.path_length)
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
        path_length: 0.0,
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
                path_length: path_length + 1.0,
                cost: path_length + neighbor.diagonal_distance_to(to),
                node: neighbor,
                parent: Some(Box::new(current.clone())),
            };

            heap.push(next);
        }
    }

    last.to_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_path() {
        let states = AStarState {
            path_length: 3.0,
            cost: 0.0,
            node: (3, 3).into(),
            parent: Some(Box::new(AStarState {
                cost: 0.0,
                node: (2, 2).into(),
                path_length: 2.0,
                parent: Some(Box::new(AStarState {
                    cost: 0.0,
                    node: (1, 1).into(),
                    path_length: 1.0,
                    parent: Some(Box::new(AStarState {
                        cost: 0.0,
                        node: (0, 0).into(),
                        path_length: 0.0,
                        parent: None,
                    })),
                })),
            })),
        };

        let path = states.to_path();

        let expected = vec![(1, 1), (2, 2)];

        assert_eq!(path, expected);
    }

    #[test]
    fn test_a_star() {
        // down
        assert_eq!(
            run_a_star((1, 1).into(), (1, 5).into()),
            vec![(1, 2), (1, 3), (1, 4)]
        );

        // down diag left
        assert_eq!(
            run_a_star((5, 1).into(), (1, 5).into()),
            vec![(4, 2), (3, 3), (2, 4)]
        );

        // left
        assert_eq!(
            run_a_star((5, 1).into(), (1, 1).into()),
            vec![(4, 1), (3, 1), (2, 1)]
        );

        // up diag left
        assert_eq!(
            run_a_star((5, 5).into(), (1, 1).into()),
            vec![(4, 4), (3, 3), (2, 2)]
        );

        // up
        assert_eq!(
            run_a_star((1, 5).into(), (1, 1).into()),
            vec![(1, 4), (1, 3), (1, 2)]
        );

        // up diag right
        assert_eq!(
            run_a_star((1, 5).into(), (5, 1).into()),
            vec![(2, 4), (3, 3), (4, 2)]
        );

        // right
        assert_eq!(
            run_a_star((1, 1).into(), (5, 1).into()),
            vec![(2, 1), (3, 1), (4, 1)]
        );

        // down diag right
        assert_eq!(
            run_a_star((1, 1).into(), (5, 5).into()),
            vec![(2, 2), (3, 3), (4, 4)]
        );

        // long with corner
        assert_eq!(
            run_a_star((1, 1).into(), (10, 5).into()),
            vec![
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 5),
                (7, 5),
                (8, 5),
                (9, 5)
            ]
        );
    }
}
