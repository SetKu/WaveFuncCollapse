use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cgmath::Vector2;
use wavefc::helpers::overlapping_adjacencies;
use wavefc::helpers::roll;
use wavefc::helpers::BorderMode;

fn black_box_9x9() -> Vec<Vec<i32>> {
    black_box(vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    ])
}

fn overlap_bench(c: &mut Criterion) {
    let data = black_box_9x9();

    c.bench_function("overlapping adjacencies (9x9 grid)", |b| {
        b.iter(|| overlapping_adjacencies(data.to_owned(), Vector2::new(3, 3), BorderMode::Clamp))
    });
}

fn roll_bench(c: &mut Criterion) {
    let mut data = black_box_9x9();

    c.bench_function("roll (9x9 grid)", |b| {
        b.iter(|| roll(&mut data, 1, true, true))
    });
}

criterion_group!(
    benches,
    overlap_bench,
    roll_bench,
);
criterion_main!(benches);
