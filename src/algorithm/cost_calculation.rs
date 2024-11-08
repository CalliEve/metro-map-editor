//! Contains everything to calculate the cost of a node in the algorithm based
//! on the previous nodes in its path.

use core::f64;

use super::{
    log_print,
    node_outside_grid,
    occupation::{
        OccupiedNode,
        OccupiedNodes,
    },
    overlap_amount,
    AlgorithmSettings,
};
use crate::{
    models::{
        Edge,
        EdgeID,
        GridNode,
        Map,
        Station,
    },
    utils::{
        calculate_angle,
        Result,
    },
    Error,
};

/// A list of edges connected to a station, sorted by the angle with which they
/// are connected to a station
type SortedStationEdgeList = Vec<(Edge, f64)>;

/// Get the edges connected to the given station, sorted by the angle with which
/// they are connected to the station, as seen from the given incoming edge.
fn edges_by_angle(
    map: &Map,
    station: &Station,
    incoming_station_node: GridNode,
    incoming_edge: EdgeID,
) -> Result<(
    SortedStationEdgeList,
    SortedStationEdgeList,
)> {
    let neighbor_nodes = station
        .get_pos()
        .get_neighbors();
    let mut left_wards = Vec::new();

    for edge_id in station.get_edges() {
        if *edge_id == incoming_edge {
            continue;
        }

        let edge = map
            .get_edge(*edge_id)
            .ok_or(Error::other(
                "edge connected to station not found",
            ))?;

        if edge.is_settled() {
            for edge_node in edge.get_edge_ends() {
                if neighbor_nodes.contains(&edge_node) {
                    left_wards.push((
                        edge.clone(),
                        calculate_angle(
                            incoming_station_node,
                            station.get_pos(),
                            edge_node,
                        ),
                    ));
                    break;
                }
            }
        } else {
            let opposite_station = map
                .get_station(
                    edge.opposite(station.get_id())
                        .unwrap(),
                )
                .ok_or(Error::other(
                    "station on connected edge not found",
                ))?;

            left_wards.push((
                edge.clone(),
                calculate_angle(
                    incoming_station_node,
                    station.get_pos(),
                    opposite_station.get_pos(),
                ),
            ));
        }
    }

    // Sort the lists by angle, so we can check the edges in order of angle small to
    // large.
    left_wards.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap()
    });

    // Create a rightwards list by reversing the leftwards list flipping the angles.
    let right_wards = left_wards
        .iter()
        .cloned()
        .map(|(e, a)| (e, (a - 360.0).abs()))
        .rev()
        .collect();

    Ok((left_wards, right_wards))
}

/// Check if the station can be approached from the given node.
/// Considers if the approach leaves enough open nodes on all sides for future
/// connections on those sides.
fn station_approach_available(
    settings: AlgorithmSettings,
    map: &Map,
    station: &Station,
    incoming_station: &Station,
    node: GridNode,
    incoming_edge: EdgeID,
) -> Result<bool> {
    // Get 2 lists of all edges connected to the station together with the angle
    // with which they are connected to it. 1 rightwards and the other
    // leftwards.
    let (left_wards, right_wards) = edges_by_angle(
        map,
        station,
        incoming_station.get_pos(),
        incoming_edge,
    )?;

    let mut cost = 0;

    let possible_angle = move |angle, cost| {
        match angle {
            0.0..=45.0 => cost < 1,
            45.0..=90.0 => cost < 2,
            90.0..=135.0 => cost < 3,
            135.0..=180.0 => cost < 4,
            180.0..=225.0 => cost < 5,
            225.0..=270.0 => cost < 6,
            270.0..=315.0 => cost < 7,
            315.0..=360.0 => cost < 8,
            _ => panic!("found impossible angle of {angle}"),
        }
    };

    // For both the right and leftwards edges, we check if the angle between the
    // incoming edge and the other edges already settled leaves enough room for the
    // edges that still need to be settled.
    for (edge, angle) in left_wards {
        if edge.is_settled() {
            if !possible_angle(angle, cost) {
                log_print(
                    settings,
                    &format!(
                        "station approach to {}{} not available from {node}\nsettled edge {} from {}
                at angle {angle} with {cost} edges between.",
                        station.get_id(),
                        station.get_pos(),
                        edge.get_id(),
                        edge.opposite(station.get_id())
                            .unwrap(),
                    ),
                    super::LogType::Warn,
                );
                return Ok(false);
            }
            break;
        }

        cost += 1;
    }

    cost = 0;
    for (edge, angle) in right_wards {
        if edge.is_settled() {
            if !possible_angle(angle, cost) {
                log_print(
                    settings,
                    &format!(
                        "station approach to {}{} not available from {}\nsettled edge {} from {}
                at angle {} with {} edges between.",
                        station.get_id(),
                        station.get_pos(),
                        node,
                        edge.get_id(),
                        edge.opposite(station.get_id())
                            .unwrap(),
                        angle,
                        cost
                    ),
                    super::LogType::Warn,
                );
                return Ok(false);
            }
            break;
        }

        cost += 1;
    }

    Ok(true)
}

/// Match the given angle to the cost of a bend of that angle
#[inline]
fn match_angle_cost(angle: f64) -> Result<f64> {
    Ok(match angle {
        360.0 => f64::INFINITY,
        315.0 => 5.0,
        270.0 => 2.5,
        225.0 => 0.5,
        180.0 => 0.0,
        135.0 => 0.5,
        90.0 => 2.5,
        45.0 => 5.0,
        0.0 => f64::INFINITY,
        _ => {
            Err(Error::other(format!(
                "found impossible angle of {angle}"
            )))?
        },
    })
}

/// Calculate the cost of the angle between three nodes.
/// The second point is assumed to be the middle node where the angle is
/// located.
fn calc_angle_cost(first: GridNode, second: GridNode, third: GridNode, round: bool) -> Result<f64> {
    let angle = if round {
        (calculate_angle(first, second, third) / 45.0).floor() * 45.0
    } else {
        calculate_angle(first, second, third)
    };

    match_angle_cost(angle).map_err(|_| {
        Error::other(format!(
            "found invalid angle of {angle} between {first}, {second}, {third}",
        ))
    })
}

/// Returns if the diagonal squared described by the given two nodes is already
/// occupied by an edge.
fn diagonal_occupied(
    map: &Map,
    first: GridNode,
    second: GridNode,
    occupied: &OccupiedNodes,
) -> bool {
    if let Some(diag_one) = occupied.get(&GridNode::from((first.0, second.1))) {
        if let Some(diag_two) = occupied.get(&GridNode::from((second.0, first.1))) {
            // if both diagonal nodes are occupied by same edge, the diagonal is occupied.
            if diag_one == diag_two {
                return true;
            }

            // if one of the diagonal nodes is a station, we check if the edge on the other
            // diagonal node is connected to it, if so, the diagonal is occupied.
            if let Some(station_id) = diag_one.get_station_id() {
                return map
                    .get_station(station_id)
                    .zip(diag_two.get_edge_id())
                    .map_or(false, |(s, edge_id)| {
                        s.get_edges()
                            .contains(&edge_id)
                    });
            }

            if let Some(station_id) = diag_two.get_station_id() {
                return map
                    .get_station(station_id)
                    .zip(diag_one.get_edge_id())
                    .map_or(false, |(s, edge_id)| {
                        s.get_edges()
                            .contains(&edge_id)
                    });
            }
        }
    }
    false
}

/// Calculate the cost of the node attached to the given station on the path
/// going away from the station.
///
/// note: the angle cost is halved here to make it have a preference, but not
/// have it force a double bend later on to compensate.
fn calc_station_exit_cost(
    map: &Map,
    current_edge: &Edge,
    station: &Station,
    node: GridNode,
    station_node: GridNode,
    target_node: GridNode,
) -> Result<f64> {
    if !station.is_settled()
        || station
            .get_edges()
            .len()
            <= 1
    {
        return calc_angle_cost(station_node, node, target_node, true);
    }

    let mut biggest_overlap = None;
    let mut current = 0;

    // find the edge with the most overlap in lines with the current edge, this is
    // the opposite edge from our edge that's leaving the station.
    for edge_id in station.get_edges() {
        if *edge_id == current_edge.get_id() {
            continue;
        }

        let edge = map
            .get_edge(*edge_id)
            .ok_or(Error::other(
                "edge connected to station not found",
            ))?;

        let overlap = overlap_amount(
            edge.get_lines(),
            current_edge.get_lines(),
        );
        if overlap > current {
            biggest_overlap = Some(edge);
            current = overlap;
        }
    }

    // if we found an opposite edge, we can calculate the cost of the angle between
    // the station and the node of the station.
    if let Some(mut opposite_edge) = biggest_overlap.cloned() {
        let neighbor_nodes = station
            .get_pos()
            .get_neighbors();

        // If the station has been settled and moved, but the opposite edge might not
        // have been settled, then there is likely a gap in the edge to the station and
        // thus we need to recalculate the nodes in the edge to get a correct bordering
        // edge.
        if !opposite_edge.is_settled() && station.get_pos() != station.get_original_pos() {
            opposite_edge.calculate_nodes(map);
        }

        // If the ends of the opposite edge are in the neighbors of the station, we
        // calculate the angle with that node. We want this to be a preference, but not
        // mandatory, so halve the angle penalty.
        for edge_node in opposite_edge.get_edge_ends() {
            if neighbor_nodes.contains(&edge_node) {
                return calc_angle_cost(
                    edge_node,
                    station.get_pos(),
                    node,
                    false,
                )
                .map(|c| c / 2.0);
            }
        }

        // Else we calculate the angle with the opposite station, this should only occur
        // when the list of nodes in the edge is empty.
        if let Some(opp_station_id) = opposite_edge.opposite(station.get_id()) {
            if let Some(opp_station) = map.get_station(opp_station_id) {
                // If the opposite edge is not
                return calc_angle_cost(
                    opp_station.get_pos(),
                    station.get_pos(),
                    node,
                    true,
                );
            }
        }
    }

    // If we didn't find an opposite edge, we can't calculate the angle and thus
    // every exit angle is equally good and might as well be free.
    Ok(0.0)
}

/// Calculate the cost of the node on the path between two stations.
/// The cost is dependent on the angle between the previous two nodes and if the
/// node is exiting or approaching a station. It also validates if the node can
/// be used for a path, and else giving a cost of infinity.
/// This is the Calculate Node Cost function from the paper.
pub fn calc_node_cost(
    settings: AlgorithmSettings,
    map: &Map,
    edge: &Edge,
    node: GridNode,
    previous: &[GridNode],
    from_station: &Station,
    to_station: &Station,
    occupied: &OccupiedNodes,
) -> Result<f64> {
    if node_outside_grid(settings, node) {
        return Ok(f64::INFINITY);
    }

    // Give the algorithm a preference for nodes that are not adjacent to other
    // stations. We don't get about the from station cause we do not apply an extra
    // penalty there.
    let mut adj_cost = 0.0;
    for neighbor_node in node.get_neighbors() {
        if let Some(&OccupiedNode::Station(neighbor_station)) = occupied.get(&neighbor_node) {
            if neighbor_station != to_station.get_id() {
                adj_cost += 1.0;
            }
        }
    }

    if to_station.is_settled() && node == to_station.get_pos() {
        if !station_approach_available(
            settings,
            map,
            to_station,
            from_station,
            node,
            edge.get_id(),
        )? {
            return Ok(f64::INFINITY);
        }
    } else if occupied.contains_key(&node) {
        return Ok(f64::INFINITY);
    }

    if previous.len() < 2 {
        if !station_approach_available(
            settings,
            map,
            from_station,
            to_station,
            node,
            edge.get_id(),
        )? {
            return Ok(f64::INFINITY);
        }

        if previous[0].0 - node.0 != 0
            && previous[0].1 - node.1 != 0
            && diagonal_occupied(map, previous[0], node, occupied)
        {
            return Ok(f64::INFINITY);
        }

        return calc_station_exit_cost(
            map,
            edge,
            from_station,
            node,
            previous[0],
            to_station.get_pos(),
        ) // cost of exiting station
        .map(|c| c + settings.move_cost); // standard cost of a move
    }

    if previous[1].0 - node.0 != 0
        && previous[1].1 - node.1 != 0
        && diagonal_occupied(map, previous[1], node, occupied)
    {
        return Ok(f64::INFINITY);
    }

    calc_angle_cost(previous[0], previous[1], node, false) // cost of angle between previous nodes
        .map(|c| c + adj_cost) // add the cost of adjacent stations
        .map(|c| c + settings.move_cost) // standard cost of a move
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_approach_available() {
        let mut map = Map::new();

        let mut approach_target = Station::new(GridNode::from((5, 5)), None);
        map.add_station(approach_target.clone());

        let above = Station::new(GridNode::from((5, 0)), None);
        let top_right = Station::new(GridNode::from((10, 0)), None);
        let top_right_2 = Station::new(GridNode::from((16, -4)), None);
        let right = Station::new(GridNode::from((10, 5)), None);
        let bottom = Station::new(GridNode::from((5, 10)), None);
        let bottom_right = Station::new(GridNode::from((10, 10)), None);
        let left = Station::new(GridNode::from((0, 5)), None);
        let bottom_left = Station::new(GridNode::from((0, 10)), None);
        let top_left = Station::new(GridNode::from((0, 0)), None);
        let top_left_2 = Station::new(GridNode::from((-6, -4)), None);

        map.add_station(above.clone());
        println!("above: {:?}", above.get_id());
        map.add_station(top_right.clone());
        println!("top_right: {:?}", top_right.get_id());
        map.add_station(right.clone());
        println!("right: {:?}", right.get_id());
        map.add_station(bottom.clone());
        println!("bottom: {:?}", bottom.get_id());
        map.add_station(left.clone());
        println!("left: {:?}", left.get_id());
        map.add_station(top_left.clone());
        println!("top_left: {:?}", top_left.get_id());

        let mut edge_above = Edge::new(
            above.get_id(),
            approach_target.get_id(),
            None,
        );
        edge_above.settle();

        let mut edge_bottom = Edge::new(
            bottom.get_id(),
            approach_target.get_id(),
            None,
        );
        edge_bottom.settle();

        let edge_top_right = Edge::new(
            top_right.get_id(),
            approach_target.get_id(),
            None,
        );
        let edge_top_right_2 = Edge::new(
            top_right_2.get_id(),
            approach_target.get_id(),
            None,
        );

        let edge_right = Edge::new(
            right.get_id(),
            approach_target.get_id(),
            None,
        );

        let edge_left = Edge::new(
            left.get_id(),
            approach_target.get_id(),
            None,
        );
        let edge_top_left = Edge::new(
            top_left.get_id(),
            approach_target.get_id(),
            None,
        );
        let edge_top_left_2 = Edge::new(
            top_left_2.get_id(),
            approach_target.get_id(),
            None,
        );

        map.add_edge(edge_above.clone());
        println!("edge_above: {:?}", edge_above.get_id());
        map.add_edge(edge_top_right.clone());
        println!(
            "edge_top_right: {:?}",
            edge_top_right.get_id()
        );
        map.add_edge(edge_right.clone());
        println!("edge_right: {:?}", edge_right.get_id());
        map.add_edge(edge_bottom.clone());
        println!(
            "edge_bottom: {:?}",
            edge_bottom.get_id()
        );
        map.add_edge(edge_left.clone());
        println!("edge_left: {:?}", edge_left.get_id());
        map.add_edge(edge_top_left.clone());
        println!(
            "edge_top_left: {:?}",
            edge_top_left.get_id()
        );

        map.quickcalc_edges();

        let mut incoming_node = GridNode::from((6, 5));

        approach_target = map
            .get_station(approach_target.get_id())
            .cloned()
            .unwrap();

        let pos_result_1 = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &right,
            incoming_node,
            edge_right.get_id(),
        )
        .unwrap();
        assert!(pos_result_1);

        map.add_station(top_right_2.clone());
        map.add_edge(edge_top_right_2.clone());
        map.quickcalc_edges();
        approach_target = map
            .get_station(approach_target.get_id())
            .cloned()
            .unwrap();

        let neg_result = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &right,
            incoming_node,
            edge_right.get_id(),
        )
        .unwrap();
        assert!(!neg_result);

        incoming_node = GridNode::from((6, 6));
        let pos_result_2 = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &bottom_right,
            incoming_node,
            edge_right.get_id(),
        )
        .unwrap();
        assert!(pos_result_2);

        // mirrored

        incoming_node = GridNode::from((4, 5));
        let mirrored_pos_result_1 = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &left,
            incoming_node,
            edge_left.get_id(),
        )
        .unwrap();
        assert!(mirrored_pos_result_1);

        map.add_station(top_left_2.clone());
        map.add_edge(edge_top_left_2.clone());
        map.quickcalc_edges();
        approach_target = map
            .get_station(approach_target.get_id())
            .cloned()
            .unwrap();

        let mirrored_neg_result = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &left,
            incoming_node,
            edge_left.get_id(),
        )
        .unwrap();
        assert!(!mirrored_neg_result);

        incoming_node = GridNode::from((4, 6));
        let mirrored_pos_result_2 = station_approach_available(
            AlgorithmSettings::default(),
            &map,
            &approach_target,
            &bottom_left,
            incoming_node,
            edge_left.get_id(),
        )
        .unwrap();
        assert!(mirrored_pos_result_2);
    }

    #[test]
    fn test_calc_angle_cost() {
        let first_45 = GridNode::from((1, 0));
        let second_45 = GridNode::from((1, 1));
        let third_45 = GridNode::from((2, 0));
        let result_45 = calc_angle_cost(first_45, second_45, third_45, false);
        assert_eq!(result_45, Ok(5.0));

        let first_90 = GridNode::from((0, 0));
        let second_90 = GridNode::from((1, 1));
        let third_90 = GridNode::from((2, 0));
        let result_90 = calc_angle_cost(first_90, second_90, third_90, false);
        assert_eq!(result_90, Ok(2.5));

        let first_135 = GridNode::from((0, 1));
        let second_135 = GridNode::from((1, 1));
        let third_135 = GridNode::from((2, 0));
        let result_135 = calc_angle_cost(first_135, second_135, third_135, false);
        assert_eq!(result_135, Ok(0.5));

        let first_180 = GridNode::from((0, 2));
        let second_180 = GridNode::from((1, 1));
        let third_180 = GridNode::from((2, 0));
        let result_180 = calc_angle_cost(first_180, second_180, third_180, false);
        assert_eq!(result_180, Ok(0.0));

        let first_135 = GridNode::from((1, 2));
        let second_135 = GridNode::from((1, 1));
        let third_135 = GridNode::from((2, 0));
        let result_135 = calc_angle_cost(first_135, second_135, third_135, false);
        assert_eq!(result_135, Ok(0.5));

        let first_90 = GridNode::from((2, 2));
        let second_90 = GridNode::from((1, 1));
        let third_90 = GridNode::from((2, 0));
        let result_90 = calc_angle_cost(first_90, second_90, third_90, false);
        assert_eq!(result_90, Ok(2.5));

        let first_45 = GridNode::from((2, 1));
        let second_45 = GridNode::from((1, 1));
        let third_45 = GridNode::from((2, 0));
        let result_45 = calc_angle_cost(first_45, second_45, third_45, false);
        assert_eq!(result_45, Ok(5.0));

        let first_180 = GridNode::from((2, 0));
        let second_180 = GridNode::from((1, 1));
        let third_180 = GridNode::from((0, 2));
        let result_180 = calc_angle_cost(first_180, second_180, third_180, false);
        assert_eq!(result_180, Ok(0.0));

        let first_135 = GridNode::from((2, 0));
        let second_135 = GridNode::from((1, 1));
        let third_135 = GridNode::from((1, 2));
        let result_135 = calc_angle_cost(first_135, second_135, third_135, false);
        assert_eq!(result_135, Ok(0.5));

        let first_90 = GridNode::from((2, 0));
        let second_90 = GridNode::from((1, 1));
        let third_90 = GridNode::from((2, 2));
        let result_90 = calc_angle_cost(first_90, second_90, third_90, false);
        assert_eq!(result_90, Ok(2.5));

        let first_45 = GridNode::from((2, 0));
        let second_45 = GridNode::from((1, 1));
        let third_45 = GridNode::from((2, 1));
        let result_45 = calc_angle_cost(first_45, second_45, third_45, false);
        assert_eq!(result_45, Ok(5.0));
    }

    #[test]
    fn test_diagonal_occupied() {
        let mut map = Map::new();
        let mut occupied = OccupiedNodes::new();

        let top_left = GridNode::from((0, 0));
        let top_right = GridNode::from((1, 0));
        let bottom_left = GridNode::from((0, 1));
        let bottom_right = GridNode::from((1, 1));

        let edge = Edge::new(0.into(), 1.into(), None);
        let edge_id = edge.get_id();
        let mut station = Station::new(GridNode::from((0, 0)), None);
        let station_id = station.get_id();
        station.add_edge(edge_id);
        map.add_station(station);

        assert!(!diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(!diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.insert(top_left, edge_id.into());
        occupied.insert(bottom_right, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, edge_id.into());
        occupied.insert(bottom_left, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));

        occupied.insert(top_left, edge_id.into());
        occupied.insert(bottom_right, station_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, edge_id.into());
        occupied.insert(bottom_left, station_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));

        occupied.insert(top_left, station_id.into());
        occupied.insert(bottom_right, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_left,
            top_right,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_right,
            bottom_left,
            &occupied
        ));

        occupied.clear();
        occupied.insert(top_right, station_id.into());
        occupied.insert(bottom_left, edge_id.into());

        assert!(diagonal_occupied(
            &map,
            bottom_right,
            top_left,
            &occupied
        ));
        assert!(diagonal_occupied(
            &map,
            top_left,
            bottom_right,
            &occupied
        ));
    }

    #[test]
    fn test_calc_station_exit_cost() {
        let mut map = Map::new();

        let unsettled_station = Station::new(GridNode::from((5, 5)), None);

        let mut settled_station = Station::new(GridNode::from((1, 1)), None);
        settled_station.settle((1, 1).into());
        let settled_edge = Edge::new(
            unsettled_station.get_id(),
            settled_station.get_id(),
            None,
        );

        let opposite_station = Station::new(GridNode::from((10, 5)), None);
        let opposite_edge = Edge::new(
            unsettled_station.get_id(),
            opposite_station.get_id(),
            None,
        );

        let opposite_settled_station = Station::new(GridNode::from((-2, -2)), None);
        let opposite_settled_edge = Edge::new(
            unsettled_station.get_id(),
            opposite_station.get_id(),
            None,
        );

        map.add_station(unsettled_station.clone());
        map.add_station(settled_station.clone());
        map.add_station(opposite_station.clone());
        map.add_station(opposite_settled_station.clone());
        map.add_edge(settled_edge.clone());
        map.add_edge(opposite_edge.clone());
        map.add_edge(opposite_settled_edge.clone());

        // unsettled and directly opposite
        assert_eq!(
            0.0,
            calc_station_exit_cost(
                &map,
                &opposite_edge,
                &unsettled_station,
                (6, 5).into(),
                unsettled_station.get_pos(),
                opposite_station.get_pos(),
            )
            .unwrap()
        );
        // unsettled and at 90 degree angle
        assert_eq!(
            2.5,
            calc_station_exit_cost(
                &map,
                &opposite_edge,
                &unsettled_station,
                (6, 6).into(),
                unsettled_station.get_pos(),
                opposite_station.get_pos(),
            )
            .unwrap()
        );

        // settled with other edge at 180 degree angle
        assert_eq!(
            0.0,
            calc_station_exit_cost(
                &map,
                &opposite_settled_edge,
                map.get_station(settled_station.get_id())
                    .unwrap(),
                (-1, -1).into(),
                settled_station.get_pos(),
                opposite_settled_station.get_pos(),
            )
            .unwrap()
        );

        // settled with other edge at 90 degree angle
        assert_eq!(
            2.5,
            calc_station_exit_cost(
                &map,
                &opposite_settled_edge,
                map.get_station(settled_station.get_id())
                    .unwrap(),
                (2, 0).into(),
                settled_station.get_pos(),
                opposite_settled_station.get_pos(),
            )
            .unwrap()
        );
    }
}
