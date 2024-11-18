use std::path::Path;

use super::{
    graphml,
    json,
};
use crate::{
    algorithm::recalculate_map,
    CanvasState,
    MapState,
};

mod create_heatmap;
mod models;

/// Run the heatmap generation for the map file given as the last command line
/// argument.
pub fn run_heatmap() {
    let args: Vec<String> = std::env::args().collect();
    let map_file = Path::new(
        args.last()
            .unwrap(),
    );

    let mut canvas = CanvasState::new();
    canvas.set_square_size(7);
    canvas.set_size((800, 1648));

    let test_file_content = std::fs::read_to_string(
        "existing_maps/".to_string()
            + map_file
                .to_str()
                .unwrap(),
    )
    .unwrap_or_else(|_| {
        panic!(
            "test data file {} does not exist",
            map_file
                .to_str()
                .unwrap()
        )
    });

    let mut map = if map_file
        .extension()
        .map_or(false, |ext| {
            ext.eq_ignore_ascii_case("json")
        }) {
        json::decode_map(&test_file_content, canvas).unwrap_or_else(|_| {
            panic!(
                "failed to decode json of {}",
                map_file
                    .to_str()
                    .unwrap()
            )
        })
    } else {
        graphml::decode_map(&test_file_content, canvas).unwrap_or_else(|_| {
            panic!(
                "failed to decode graphml of {}",
                map_file
                    .to_str()
                    .unwrap()
            )
        })
    };

    let mut state = MapState::new(map.clone());
    state.calculate_algorithm_settings();
    let mut settings = state.get_algorithm_settings();
    settings.edge_routing_attempts = 1;
    settings.local_search = false;
    settings.early_local_search_abort = false;

    let occupied = recalculate_map(settings, &mut map).unwrap();

    let heatmap = create_heatmap::create_heatmap(settings, &map, &occupied);

    let heatmap_file = format!(
        "research_notebooks/{}_heatmap.json",
        map_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let heatmap_json = serde_json::to_string_pretty(&heatmap).unwrap();
    std::fs::write(heatmap_file, heatmap_json).expect("failed to write heatmap file");
}
