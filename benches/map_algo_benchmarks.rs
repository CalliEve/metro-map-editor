use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use metro_map_editor::{
    algorithm::run_a_star,
    models::GridNode,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let from = GridNode::from((0, 0));
    let to = GridNode::from((10, 15));

    c.bench_function("a_star", |b| {
        b.iter(|| run_a_star(black_box(from), black_box(to)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
