#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sharpen::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input: &[Option<&[u8]>] = &[
        Some(&[0, 0, 1]),
        Some(&[0, 1]),
        None,
        None,
        Some(&[2]),
        None,
    ];
    c.bench_function("classify 6-4", move |b| {
        b.iter(|| classify_as_vec(input.iter(), |curo| curo.is_some()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
