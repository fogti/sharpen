#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sharpen::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element(usize);

impl tree_rollup::Node for Element {
    fn push_child(&mut self, child: Element) {
        self.0 += child.0;
    }
    fn reverse(&mut self) {}
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = vec![Element(0), Element(1), Element(2), Element(3), Element(4)];

    let mut mapping = BTreeMap::new();
    mapping.insert(1, 0);
    mapping.insert(3, 2);
    mapping.insert(4, 1);

    c.bench_function("tree-rollup 5-2", move |b| {
        b.iter(|| {
            let _result: Vec<_> = rollup_tree(input.clone().into_iter(), &mapping)
                .expect("valid mapping")
                .collect();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
