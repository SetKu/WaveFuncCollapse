extern crate cgmath;
extern crate criterion;

use cgmath::Vector2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wavefc::prelude::*;

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

criterion_group!(benches, analysis_bench, collapse_bench);
criterion_main!(benches);
