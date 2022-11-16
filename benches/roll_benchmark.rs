use wfc::helpers::roll;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn roll_benchmark(c: &mut Criterion) {
    let mut data = black_box(
        vec![
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
            vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
        ]
    );

    c.bench_function(
        "roll (2D-Array shifting) function",
        |b| { b.iter(|| roll(&mut data, 1, true, true)) }
    );
}

criterion_group!(benches, roll_benchmark);
criterion_main!(benches);
