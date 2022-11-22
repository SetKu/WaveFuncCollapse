extern crate cgmath;
extern crate criterion;
extern crate wfc;

use cgmath::Vector2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wfc::helpers::{overlapping_adjacencies, roll, BorderMode};

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

// fn all_possible_shifts_benchmark(c: &mut Criterion) {
// let data = black_box(vec![
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
// ]);

// c.bench_function("all possible (2d-array) shifts", |b| {
// b.iter(|| all_possible_shifts(data.to_owned()))
// });
// }

criterion_group!(benches, roll_bench, overlap_bench);
criterion_main!(benches);
