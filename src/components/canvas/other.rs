//! Contains non-event functions used by the [`Canvas`] component.

use ev::UiEvent;
use leptos::*;

use crate::{
    models::{
        EdgeID,
        Map,
    },
    MapState,
};

/// Calculates and updates the size of the canvas.
///
/// To have a canvas resize dynamically, we need to manually adjust its size
/// CSS will NOT work, as it will just make everything blurry.
/// This means we have to manually calculate the desired size of the canvas.
pub fn update_canvas_size(map_state: &RwSignal<MapState>) {
    let doc = document();

    // the navbar borders the top, so the height is `window - navbar`.
    let win_height = window()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();

    let navbar = doc
        .get_element_by_id("navbar")
        .expect("navbar should exist");
    let nav_height_px = window()
        .get_computed_style(&navbar)
        .unwrap()
        .expect("should have style")
        .get_property_value("height")
        .expect("should have height property");
    let nav_height = nav_height_px
        .trim_end_matches("px")
        .parse::<f64>()
        .expect("height should be a number")
        .round();

    let height = win_height - nav_height;

    // the sidebar borders its side, so width is `window - sidebar`.
    let win_width = window()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();

    let sidebar = doc
        .get_element_by_id("sidebar")
        .expect("sidebar should exist");
    let side_width_px = window()
        .get_computed_style(&sidebar)
        .unwrap()
        .expect("should have style")
        .get_property_value("width")
        .expect("should have width property");
    let side_width = side_width_px
        .trim_end_matches("px")
        .parse::<f64>()
        .expect("width should be a number")
        .round();

    let width = win_width - side_width;

    // update the state with the new size.
    logging::log!(
        "new canvas size: ({}, {})",
        height,
        width
    );
    map_state.update(|state| {
        state.update_canvas_state(|canvas| {
            canvas.set_size((height, width));
            canvas.set_neighbor_sizes((nav_height, side_width));
        })
    });
}

/// Gets the position on the canvas that was clicked.
pub fn canvas_click_pos(map_size: (f64, f64), ev: &UiEvent) -> (f64, f64) {
    let win_height = window()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();
    let win_width = window()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap()
        .round();

    (
        (f64::from(ev.page_x()) - (win_width - map_size.1)),
        (f64::from(ev.page_y()) - (win_height - map_size.0)),
    )
}

/// Helper function for recalculating an edge nodes.
pub fn recalculate_edge_nodes(map: &mut Map, edge_id: EdgeID) {
    let edge = map
        .get_edge(edge_id)
        .cloned()
        .expect("edge should exist");
    let mut edge = edge.clone();
    edge.calculate_nodes(map);
    map.add_edge(edge);
}
