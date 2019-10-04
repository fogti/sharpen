use super::*;
use alloc::{collections::BTreeMap, vec, vec::Vec};

#[derive(Debug, PartialEq, Eq)]
struct Element(usize);

impl Node for Element {
    fn push_child(&mut self, child: Element) {
        self.0 += child.0;
    }
}

impl ReversableNode for Element {
    fn reverse(&mut self) {}
}

#[test]
fn test_rollup_tree() {
    let input = vec![Element(0), Element(1), Element(2), Element(3), Element(4)];

    let mut mapping = BTreeMap::new();
    mapping.insert(1, 0);
    mapping.insert(3, 2);
    mapping.insert(4, 1);

    let result: Vec<_> = rollup_tree(input.into_iter(), mapping)
        .expect("valid mapping")
        .collect();
    assert_eq!(result, vec![Element(5), Element(5)]);
}

#[derive(Debug, PartialEq, Eq)]
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
    fn push_child(&mut self, child: Self) {
        self.children.push(child);
    }
}

impl ReversableNode for CplxElement {
    fn reverse(&mut self) {
        self.children.reverse();
    }
}

#[test]
fn test_rollup_complex_tree() {
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

    let result: Vec<_> = rollup_tree(input.into_iter(), mapping)
        .expect("invalid mapping")
        .collect();
    assert_eq!(
        result,
        vec![
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
        ]
    );
}

#[test]
fn test_rollup_bottomup_complex_tree() {
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

    let result: Vec<_> = rollup_tree_bottomup(input.into_iter(), mapping)
        .expect("invalid mapping")
        .collect();

    assert_eq!(
        result,
        vec![
            CplxElement {
                data: 2,
                children: vec![CplxElement::new(0), CplxElement::new(1)]
            },
            CplxElement {
                data: 7,
                children: vec![
                    CplxElement {
                        data: 5,
                        children: vec![CplxElement::new(3), CplxElement::new(4)]
                    },
                    CplxElement::new(6),
                ],
            },
            CplxElement::new(8)
        ]
    );
}
