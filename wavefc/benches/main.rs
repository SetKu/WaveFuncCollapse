extern crate cgmath;
extern crate criterion;
extern crate wfc;

use cgmath::Vector2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wfc::helpers::overlapping_adjacencies;
use wfc::helpers::roll;
use wfc::prelude::*;

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

fn analysis_bench(c: &mut Criterion) {
    let string = include_str!("sample.txt")
        .replace(", ", "")
        .replace(",", "");
    let parsed = deconstruct_string(&string, false);
    let data = black_box(parsed.0);
    let mut wave = Wave::new();

    c.bench_function("analysis", |b| {
        b.iter(|| wave.analyze(data.to_owned(), Vector2::new(2, 2), BorderMode::Clamp))
    });
}

fn collapse_bench(c: &mut Criterion) {
    let string = include_str!("sample.txt")
        .replace(", ", "")
        .replace(",", "");
    let parsed = deconstruct_string(&string, false);
    let data = black_box(parsed.0);
    let mut wave = Wave::new();
    wave.analyze(data.to_owned(), Vector2::new(2, 2), BorderMode::Clamp);

    c.bench_function("collapse (the actual wave function)", |b| {
        b.iter(|| {
            wave.fill(Vector2::new(10, 10)).unwrap();
            let _ = wave.collapse_all(5000, Some(|_, _, _| {}));
        })
    });
}

criterion_group!(
    benches,
    roll_bench,
    overlap_bench,
    analysis_bench,
    collapse_bench
);
criterion_main!(benches);
