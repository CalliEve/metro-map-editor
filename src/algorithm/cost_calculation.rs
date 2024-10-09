//! Contains everything to calculate the cost of a node in the algorithm based
//! on the previous nodes in its path.

use core::f64;

use super::{
    calculate_angle,
    debug_print,
    node_outside_grid,
    occupation::OccupiedNodes,
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
    utils::Result,
    Error,
};

/// Check if the station can be approached from the given node.
/// Considers if the approach leaves enough open nodes on all sides for future
/// connections on those sides.
fn station_approach_available(
    settings: AlgorithmSettings,
    map: &Map,
    station: &Station,
    node: GridNode,
    incoming_edge: EdgeID,
) -> Result<bool> {
    let neighbor_nodes = station
        .get_pos()
        .get_neighbors();
    let mut left_wards = Vec::new();
    let mut right_wards = Vec::new();

    // Get 2 lists of all edges connected to the station together with the angle
    // with which they are connected to it. 1 rightwards and the other
    // leftwards.
    for edge_id in station.get_edges() {
        if *edge_id == incoming_edge {
            continue;
        }

        let edge = map
            .get_edge(*edge_id)
            .ok_or(Error::other(
                "edge connected to station not found",
            ))?;

        for edge_node in edge.get_nodes() {
            if neighbor_nodes.contains(edge_node) {
                left_wards.push((
                    edge.clone(),
                    calculate_angle(node, station.get_pos(), *edge_node),
                ));

                right_wards.push((
                    edge.clone(),
                    (calculate_angle(node, station.get_pos(), *edge_node) - 360.0).abs(),
                ));
            }
        }
    }

    // Sort the lists by angle, so we can check the edges in order of angle small to
    // large.
    left_wards.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap()
    });
    right_wards.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap()
    });

    let mut cost = 0;

    let possible_angle = move |angle, cost| {
        match angle {
            315.0 => cost < 7,
            270.0 => cost < 6,
            225.0 => cost < 5,
            180.0 => cost < 4,
            135.0 => cost < 3,
            90.0 => cost < 2,
            45.0 => cost < 1,
            0.0 => false,
            _ => panic!("found impossible angle of {angle}"),
        }
    };

    // For both the right and leftwards edges, we check if the angle between the
    // incoming edge and the other edges already settled leaves enough room for the
    // edges that still need to be settled.
    for (edge, angle) in left_wards {
        if edge.is_settled() {
            if !possible_angle(angle, cost) {
                debug_print(
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
                    true,
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
                debug_print(
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
                    true,
                );
                return Ok(false);
            }
            break;
        }

        cost += 1;
    }

    Ok(true)
}

/// Calculate the cost of the angle between three nodes.
/// The second point is assumed to be the middle node where the angle is
/// located.
fn calc_angle_cost(first: GridNode, second: GridNode, third: GridNode) -> Result<f64> {
    let angle = calculate_angle(first, second, third);

    Ok(match angle {
        315.0 => 2.0,
        270.0 => 1.5,
        225.0 => 1.0,
        180.0 => 0.0,
        135.0 => 1.0,
        90.0 => 1.5,
        45.0 => 2.0,
        0.0 => f64::INFINITY,
        _ => {
            Err(Error::other(format!(
                "found invalid angle of {angle} between {first}, {second}, {third}",
            )))?
        },
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
fn calc_station_exit_cost(
    map: &Map,
    current_edge: &Edge,
    station: &Station,
    node: GridNode,
) -> Result<f64> {
    if !station.is_settled() {
        return Ok(0.0);
    }

    let mut biggest_overlap = None;
    let mut current = 0;

    // find the edge with the most overlap in lines with the current edge, this is
    // the opposite edge from our edge that's leaving the station.
    for edge_id in station.get_edges() {
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
    if let Some(opposite_edge) = biggest_overlap {
        let neighbor_nodes = station
            .get_pos()
            .get_neighbors();

        // If the ends of the opposite edge are in the neighbors of the station, we
        // calculate the angle with that node.
        for edge_node in opposite_edge.get_edge_ends() {
            if neighbor_nodes.contains(&edge_node) {
                return calc_angle_cost(edge_node, station.get_pos(), node);
            }
        }

        // Else we calculate the angle with the opposite station, this should only occur
        // when the list of nodes in the edge is empty.
        if let Some(opp_station_id) = opposite_edge.opposite(station.get_id()) {
            if let Some(opp_station) = map.get_station(opp_station_id) {
                // CHECKME: we should instead round angles for such cases. Also: investigate why
                // an edge nodes might be empty / not be neighbors of the station.
                return Ok(calc_angle_cost(
                    opp_station.get_pos(),
                    station.get_pos(),
                    node,
                )
                .unwrap_or(0.0));
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

    if to_station.is_settled() && node == to_station.get_pos() {
        if !station_approach_available(
            settings,
            map,
            to_station,
            *previous
                .last()
                .unwrap(),
            edge.get_id(),
        )? {
            return Ok(f64::INFINITY);
        }
    } else if occupied.contains_key(&node) {
        return Ok(f64::INFINITY);
    }

    if previous.len() < 2 {
        if previous[0].0 - node.0 != 0
            && previous[0].1 - node.1 != 0
            && diagonal_occupied(map, previous[0], node, occupied)
        {
            return Ok(f64::INFINITY);
        }

        return calc_station_exit_cost(map, edge, from_station, node) // cost of exiting station
            .map(|c| c + settings.move_cost); // standard cost of a move
    }

    if previous[1].0 - node.0 != 0
        && previous[1].1 - node.1 != 0
        && diagonal_occupied(map, previous[1], node, occupied)
    {
        return Ok(f64::INFINITY);
    }

    calc_angle_cost(previous[0], previous[1], node) // cost of angle between previous nodes
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
        let top_right_2 = Station::new(GridNode::from((15, -5)), None);
        let right = Station::new(GridNode::from((10, 5)), None);
        let bottom = Station::new(GridNode::from((5, 10)), None);
        let left = Station::new(GridNode::from((0, 5)), None);
        let top_left = Station::new(GridNode::from((0, 0)), None);
        let top_left_2 = Station::new(GridNode::from((-5, -5)), None);

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
            incoming_node,
            edge_left.get_id(),
        )
        .unwrap();
        assert!(mirrored_pos_result_2);
    }

    #[test]
    fn test_calc_angle_cost() {
        let first_180 = GridNode::from((0, 2));
        let second_180 = GridNode::from((1, 1));
        let third_180 = GridNode::from((2, 0));
        let result_180 = calc_angle_cost(first_180, second_180, third_180);
        assert_eq!(result_180, Ok(0.0));

        let first_135 = GridNode::from((2, 2));
        let second_135 = GridNode::from((1, 1));
        let third_135 = GridNode::from((1, 0));
        let result_135 = calc_angle_cost(first_135, second_135, third_135);
        assert_eq!(result_135, Ok(1.0));

        let first_90 = GridNode::from((0, 0));
        let second_90 = GridNode::from((1, 1));
        let third_90 = GridNode::from((2, 0));
        let result_90 = calc_angle_cost(first_90, second_90, third_90);
        assert_eq!(result_90, Ok(1.5));

        let first_45 = GridNode::from((1, 0));
        let second_45 = GridNode::from((1, 1));
        let third_45 = GridNode::from((2, 0));
        let result_45 = calc_angle_cost(first_45, second_45, third_45);
        assert_eq!(result_45, Ok(2.0));
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
        // TODO: implement test
    }

    #[test]
    fn test_calc_node_cost() {
        // TODO: implement test
    }
}
