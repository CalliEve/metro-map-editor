use std::time::Duration;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use metro_map_editor::{
    algorithm::{
        recalculate_map,
        run_a_star,
    },
    models::GridNode,
    utils::{
        graphml,
        json,
    },
    CanvasState,
    MapState,
};

pub fn a_star_benchmark(c: &mut Criterion) {
    let from = GridNode::from((0, 0));
    let to = GridNode::from((10, 15));

    c.bench_function("a_star", |b| {
        b.iter(|| run_a_star(black_box(from), black_box(to)))
    });
}

pub fn full_recalculation_simple_benchmark(c: &mut Criterion) {
    let mut canvas = CanvasState::new();
    canvas.set_square_size(5);

    let test_file_content = std::fs::read_to_string("existing_maps/routing_test.json")
        .expect("test data file does not exist");
    let map = json::decode_map(&test_file_content, canvas).expect("failed to decode json");

    let mut state = MapState::new(map.clone());
    state.calculate_algorithm_settings();
    let settings = state.get_algorithm_settings();

    c.bench_function("full_recalculation_simple", |b| {
        b.iter(|| {
            let mut map = map.clone();
            recalculate_map(black_box(settings), black_box(&mut map))
        })
    });
}

pub fn full_recalculation_sydney_benchmark(c: &mut Criterion) {
    let mut canvas = CanvasState::new();
    canvas.set_square_size(7);
    canvas.set_size((700, 1600)); // Without enlarging the canvas, some stations will overlap due to map size

    let test_file_content = std::fs::read_to_string("existing_maps/sydney.graphml") // TODO: change back to berlin
        .expect("test data file does not exist");
    let map = graphml::decode_map(&test_file_content, canvas).expect("failed to decode graphml");

    let mut state = MapState::new(map.clone());
    state.calculate_algorithm_settings();
    let settings = state.get_algorithm_settings();

    c.bench_function("full_recalculation_berlin", |b| {
        b.iter(|| {
            let mut map = map.clone();
            recalculate_map(black_box(settings), black_box(&mut map))
                .expect("failed to recalculate map")
        })
    });
}

criterion_group!(
    name = full_recalculation_benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30)).sample_size(20);
    targets = full_recalculation_simple_benchmark, full_recalculation_berlin_benchmark
);
criterion_group!(a_star_benches, a_star_benchmark);
criterion_main!(
    a_star_benches,
    full_recalculation_benches
);
