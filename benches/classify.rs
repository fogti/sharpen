#[macro_use]
extern crate criterion;

use criterion::{Criterion};
use sharpen::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input: Vec<Option<Vec<u8>>> = vec![
        Some(vec![0, 0, 1]),
        Some(vec![0, 1]),
        None,
        None,
        Some(vec![2]),
        None,
    ];
    c.bench_function("classify 6-4", move |b| b.iter(|| classify_as_vec(input.iter(), |curo| curo.is_some())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
