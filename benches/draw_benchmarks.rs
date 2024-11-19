use std::time::Duration;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use metro_map_editor::{
    algorithm::drawing::{
        draw_edge,
        CanvasContext,
    },
    models::{
        GridNode,
        Map,
    },
    utils::graphml,
    CanvasState,
};

fn draw_edge_benchmark(c: &mut Criterion) {
    let context = CanvasContext::new();
    let mut state = CanvasState::default();
    state.set_square_size(5);
    state.set_size((50.0, 50.0));
    let from = GridNode::from((0, 0));
    let to = GridNode::from((8, 8));
    let steps = vec![
        GridNode::from((1, 1)),
        GridNode::from((2, 2)),
        GridNode::from((3, 3)),
        GridNode::from((4, 4)),
        GridNode::from((5, 5)),
        GridNode::from((6, 6)),
        GridNode::from((7, 7)),
        GridNode::from((8, 8)),
        GridNode::from((9, 9)),
        GridNode::from((10, 10)),
        GridNode::from((11, 11)),
        GridNode::from((11, 12)),
        GridNode::from((11, 13)),
        GridNode::from((11, 12)),
        GridNode::from((11, 11)),
        GridNode::from((10, 10)),
        GridNode::from((9, 9)),
    ];

    c.bench_function("draw_edge", |b| {
        b.iter(|| {
            draw_edge(
                black_box(from),
                black_box(to),
                black_box(&steps),
                black_box(&context),
                black_box(state),
                black_box(1.0),
            )
        })
    });
}

fn draw_map_benchmark(c: &mut Criterion) {
    let context = CanvasContext::new();
    let mut canvas = CanvasState::new();
    canvas.set_square_size(7);
    canvas.set_size((700.0, 1500.0)); // Without enlarging the canvas, some stations will overlap due to map size

    let test_file_content = std::fs::read_to_string("existing_maps/berlin.graphml")
        .expect("test data file does not exist");
    let map = graphml::decode_map(&test_file_content, canvas).expect("failed to decode graphml");

    c.bench_function("draw_map", |b| {
        b.iter(|| {
            Map::draw(
                black_box(&map),
                black_box(&context),
                black_box(canvas),
                1.0,
            )
        })
    });
}

criterion_group!(
    name = drawing_benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = draw_edge_benchmark,draw_map_benchmark
);
criterion_main!(drawing_benches);
