use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wfc::helpers::{all_possible_shifts, roll};

fn roll_benchmark(c: &mut Criterion) {
    let mut data = black_box(vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ]);

    c.bench_function("roll (2d-array shifting)", |b| {
        b.iter(|| roll(&mut data, 1, true, true))
    });
}

fn all_possible_shifts_benchmark(c: &mut Criterion) {
    let mut data = black_box(vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ]);

    c.bench_function("all possible (2d-array) shifts", |b| {
        b.iter(|| all_possible_shifts(data.to_owned()))
    });
}

criterion_group!(benches, roll_benchmark, all_possible_shifts_benchmark);
criterion_main!(benches);
