#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sharpen::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element(usize);

impl tree_rollup::Node for Element {
    fn push_child(&mut self, child: Element) {
        self.0 += child.0;
    }
    fn reverse(&mut self) {}
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CplxElement {
    data: usize,
    children: Vec<CplxElement>,
}

impl CplxElement {
    fn new(data: usize) -> Self {
        Self {
            data,
            children: vec![],
        }
    }
}

impl tree_rollup::Node for CplxElement {
    fn push_child(&mut self, child: Self) {
        self.children.push(child);
    }
    fn reverse(&mut self) {
        self.children.reverse();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = vec![Element(0), Element(1), Element(2), Element(3), Element(4)];
    let mapping = vec![(1, 0), (3, 2), (4, 1)];

    c.bench_function("tree-rollup 5-2", move |b| {
        b.iter(|| {
            let _result: Vec<_> = rollup_tree(input.clone().into_iter(), mapping.iter().copied())
                .expect("valid mapping")
                .collect();
        })
    });

    let input = vec![Element(0), Element(1), Element(2), Element(3), Element(4)];
    let mapping = vec![(0, 1), (2, 3), (1, 4)];

    c.bench_function("tree-rollup-bottomup 5-2", move |b| {
        b.iter(|| {
            let _result: Vec<_> =
                rollup_tree_bottomup(input.clone().into_iter(), mapping.iter().copied())
                    .expect("valid mapping")
                    .collect();
        })
    });

    let input = vec![
        CplxElement::new(0),
        CplxElement::new(1),
        CplxElement::new(2),
        CplxElement::new(3),
        CplxElement::new(4),
        CplxElement::new(5),
        CplxElement::new(6),
        CplxElement::new(7),
        CplxElement::new(8),
    ];
    let mapping = vec![(1, 0), (2, 0), (4, 3), (5, 3), (6, 4), (7, 3)];

    c.bench_function("tree-rollup-cplx 9-3", move |b| {
        b.iter_with_large_drop(|| {
            rollup_tree(input.clone().into_iter(), mapping.iter().copied())
                .expect("invalid mapping")
        })
    });

    let input = vec![
        CplxElement::new(0),
        CplxElement::new(1),
        CplxElement::new(2),
        CplxElement::new(3),
        CplxElement::new(4),
        CplxElement::new(5),
        CplxElement::new(6),
        CplxElement::new(7),
        CplxElement::new(8),
    ];
    let mapping = vec![(0, 2), (1, 2), (3, 5), (4, 5), (5, 7), (6, 7)];

    c.bench_function("tree-rollup-bottomup-cplx 9-3", move |b| {
        b.iter_with_large_drop(|| {
            rollup_tree_bottomup(input.clone().into_iter(), mapping.iter().copied())
                .expect("invalid mapping")
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
