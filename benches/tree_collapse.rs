#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sharpen::{
    tree_collapse::{CollapseTreeOrd, Node},
    *,
};

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

impl Node for CplxElement {
    type ChildrenIter = std::vec::IntoIter<CplxElement>;
    fn take_children(&mut self) -> Self::ChildrenIter {
        core::mem::replace(&mut self.children, vec![]).into_iter()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = vec![
        CplxElement {
            data: 0,
            children: vec![CplxElement::new(1), CplxElement::new(2)],
        },
        CplxElement {
            data: 3,
            children: vec![
                CplxElement {
                    data: 4,
                    children: vec![CplxElement::new(6)],
                },
                CplxElement::new(5),
                CplxElement::new(7),
            ],
        },
        CplxElement::new(8),
    ];

    let use_input = input.clone();
    c.bench_function("tree-collapse 3-9", move |b| {
        b.iter_with_large_drop(|| {
            collapse_tree(use_input.clone().into_iter(), CollapseTreeOrd::TopDown)
        })
    });

    let use_input = input;
    c.bench_function("tree-collapse-bottomup 3-9", move |b| {
        b.iter_with_large_drop(|| {
            collapse_tree(use_input.clone().into_iter(), CollapseTreeOrd::BottomUp)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
