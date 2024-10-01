use std::time::Duration;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use metro_map_editor::{
    algorithm::run_a_star,
    models::GridNode,
    utils::json,
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

pub fn full_recalculation_benchmark(c: &mut Criterion) {
    let mut canvas = CanvasState::new();
    canvas.set_square_size(5);

    let test_file_content = std::fs::read_to_string("exisiting_maps/routing_test.json")
        .expect("test data file does not exist");
    let map = json::decode_map(&test_file_content, canvas).expect("failed to decode graphml");

    let state = MapState::new(map);

    c.bench_function("full_recalculation", |b| {
        b.iter(|| {
            let mut alg_state = state.clone();
            MapState::run_algorithm(black_box(&mut alg_state))
        })
    });
}

criterion_group!(
    name = full_recalculation_benches;
    config = Criterion::default().measurement_time(Duration::from_secs(20)).sample_size(20);
    targets = full_recalculation_benchmark
);
criterion_group!(a_star_benches, a_star_benchmark);
criterion_main!(
    a_star_benches,
    full_recalculation_benches
);
