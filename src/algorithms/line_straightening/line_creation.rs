//! Contains functions to create straight edges between two nodes based upon the
//! existing connections between them.

use itertools::Itertools;

use crate::{
    algorithms::{
        calc_direction::node_direction,
        occupation::diagonal_occupied,
        EdgeDirection,
        OccupiedNodes,
    },
    models::{
        Edge,
        GridNode,
        Map,
    },
    utils::Result,
    Error,
};

/// A short-hand for a tuple of a start node, a straight edge and an end node.
/// Together these describe a possible straight edge between two nodes.
type EdgeCandidate = (GridNode, Vec<GridNode>, GridNode);

/// Create valid edge candidates for the given start and end nodes if they
/// exist.
pub fn create_edge_candidates(
    map: &Map,
    occupied: &OccupiedNodes,
    start: GridNode,
    end: GridNode,
    edges: &[Edge],
) -> Result<Vec<EdgeCandidate>> {
    let overal_direction = edges
        .iter()
        .map(|edge| get_edge_direction(map, edge))
        .counts()
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map_or(
            EdgeDirection::Equal,
            |(direction, _)| direction,
        );

    if overal_direction == EdgeDirection::Equal {
        return Err(Error::other(
            "No overall direction found",
        ));
    }

    let possible_start_ends = get_start_ends(start, end, overal_direction);

    Ok(possible_start_ends
        .into_iter()
        .map(|(start, end)| {
            Ok((
                start,
                create_straight_edge(
                    map,
                    occupied,
                    start,
                    end,
                    overal_direction,
                )?,
                end,
            ))
        })
        .filter_map(|res: Result<_>| res.ok())
        .collect::<Vec<_>>())
}

/// Returns the most prevalent direction of the given edge.
fn get_edge_direction(map: &Map, edge: &Edge) -> EdgeDirection {
    let from_station = map
        .get_station(edge.get_from())
        .expect("from-station of edge not found");
    let to_station = map
        .get_station(edge.get_to())
        .expect("to-station of edge not found");

    let mut all_nodes = vec![from_station.get_pos()];
    all_nodes.extend(edge.get_nodes());
    all_nodes.push(to_station.get_pos());

    all_nodes
        .iter()
        .zip(
            all_nodes
                .iter()
                .skip(1),
        )
        .map(|(start, end)| node_direction(*start, *end))
        .counts()
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map_or(
            EdgeDirection::Equal,
            |(direction, _)| direction,
        )
}

/// Get all possible start and end nodes for the straight edge.
fn get_start_ends(
    start: GridNode,
    end: GridNode,
    direction: EdgeDirection,
) -> Vec<(GridNode, GridNode)> {
    match direction {
        EdgeDirection::Equal => {
            panic!("start {start} and end {end} are equal",)
        },
        EdgeDirection::Right | EdgeDirection::Left => {
            match start
                .1
                .cmp(&end.1)
            {
                std::cmp::Ordering::Equal => vec![(start, end)],
                std::cmp::Ordering::Less => {
                    (start.1..=end.1)
                        .map(|y| {
                            (
                                GridNode::new(start.0, y),
                                GridNode::new(end.0, y),
                            )
                        })
                        .collect()
                },
                std::cmp::Ordering::Greater => {
                    (end.1..=start.1)
                        .map(|y| {
                            (
                                GridNode::new(start.0, y),
                                GridNode::new(end.0, y),
                            )
                        })
                        .collect()
                },
            }
        },
        EdgeDirection::Up | EdgeDirection::Down => {
            match start
                .0
                .cmp(&end.0)
            {
                std::cmp::Ordering::Equal => vec![(start, end)],
                std::cmp::Ordering::Less => {
                    (start.0..=end.0)
                        .map(|x| {
                            (
                                GridNode::new(x, start.1),
                                GridNode::new(x, end.1),
                            )
                        })
                        .collect()
                },
                std::cmp::Ordering::Greater => {
                    (end.0..=start.0)
                        .map(|x| {
                            (
                                GridNode::new(x, start.1),
                                GridNode::new(x, end.1),
                            )
                        })
                        .collect()
                },
            }
        },
        EdgeDirection::DiagUpRight
        | EdgeDirection::DiagUpLeft
        | EdgeDirection::DiagDownRight
        | EdgeDirection::DiagDownLeft => diag_start_end(start, end),
    }
}

/// Get all possible start and end nodes for the straight edge in diagonal
/// cases.
fn diag_start_end(start: GridNode, end: GridNode) -> Vec<(GridNode, GridNode)> {
    let hor_diff = (start.0 - end.0).abs();
    let vert_diff = (start.1 - end.1).abs();
    let mut needed_adjustment = (hor_diff - vert_diff).abs();

    if needed_adjustment == 0 {
        return vec![(start, end)];
    }

    if hor_diff != (start.1 - (end.1 + needed_adjustment)).abs() {
        needed_adjustment *= -1;
    }

    let start_adjust = needed_adjustment / 2;
    let end_adjust = needed_adjustment - start_adjust;

    vec![
        (
            GridNode::new(start.0, start.1),
            GridNode::new(end.0, end.1 + needed_adjustment),
        ),
        (
            GridNode::new(start.0, start.1 - start_adjust),
            GridNode::new(end.0, end.1 + end_adjust),
        ),
        (
            GridNode::new(start.0, start.1 - needed_adjustment),
            GridNode::new(end.0, end.1),
        ),
    ]
}

/// Create a straight edge between the two given nodes.
pub fn create_straight_edge(
    map: &Map,
    occupied: &OccupiedNodes,
    start: GridNode,
    end: GridNode,
    mut direction: EdgeDirection,
) -> Result<Vec<GridNode>> {
    let mut line = Vec::new();
    let mut current = start;

    let max_iter = (start.0 - end.0).abs() + (start.1 - end.1).abs();
    let mut iter = 0;

    if node_direction(start, end) == direction.flip() {
        direction = direction.flip();
    }

    while current != end {
        let next = match direction {
            EdgeDirection::Right => GridNode::new(current.0 + 1, current.1),
            EdgeDirection::Left => GridNode::new(current.0 - 1, current.1),
            EdgeDirection::Up => GridNode::new(current.0, current.1 - 1),
            EdgeDirection::Down => GridNode::new(current.0, current.1 + 1),
            EdgeDirection::DiagUpRight => GridNode::new(current.0 + 1, current.1 - 1),
            EdgeDirection::DiagUpLeft => GridNode::new(current.0 - 1, current.1 - 1),
            EdgeDirection::DiagDownRight => GridNode::new(current.0 + 1, current.1 + 1),
            EdgeDirection::DiagDownLeft => GridNode::new(current.0 - 1, current.1 + 1),
            EdgeDirection::Equal => {
                return Err(Error::other(
                    "Cannot create line with equal direction",
                ));
            },
        };

        if next == end {
            break;
        }

        if next.manhattan_distance_to(end) > current.manhattan_distance_to(end) {
            leptos::logging::warn!(
                "Line does not reach end; start {} end {} direction {:?}",
                start,
                end,
                direction
            );
            return Err(Error::other("Line does not reach end"));
        }

        if occupied.contains_key(&next) || diagonal_occupied(map, current, next, occupied) {
            return Err(Error::EarlyAbort);
        }

        line.push(next);
        current = next;

        iter += 1;
        if iter > max_iter {
            leptos::logging::warn!(
                "Max iter. Line does not reach end; start {} end {} direction {:?}",
                start,
                end,
                direction
            );
            return Err(Error::other("Max iterations reached"));
        }
    }

    Ok(line)
}
