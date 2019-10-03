use super::*;
use alloc::{vec, vec::Vec};

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
    type ChildrenIter = alloc::vec::IntoIter<CplxElement>;
    fn take_children(&mut self) -> Self::ChildrenIter {
        core::mem::replace(&mut self.children, vec![]).into_iter()
    }
}

#[test]
fn test_collapse_complex_tree() {
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

    assert_eq!(
        collapse_tree(input.into_iter(), CollapseTreeOrd::TopDown),
        CollapsedTree {
            elems: vec![
                CplxElement::new(0),
                CplxElement::new(1),
                CplxElement::new(2),
                CplxElement::new(3),
                CplxElement::new(4),
                CplxElement::new(6),
                CplxElement::new(5),
                CplxElement::new(7),
                CplxElement::new(8)
            ],
            mapping: [(1, 0), (2, 0), (4, 3), (5, 4), (6, 3), (7, 3)]
                .into_iter()
                .copied()
                .collect(),
        }
    );
}

#[test]
fn test_collapse_bottomup_complex_tree() {
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

    assert_eq!(
        collapse_tree(input.into_iter(), CollapseTreeOrd::BottomUp),
        CollapsedTree {
            elems: vec![
                CplxElement::new(1),
                CplxElement::new(2),
                CplxElement::new(0),
                CplxElement::new(6),
                CplxElement::new(4),
                CplxElement::new(5),
                CplxElement::new(7),
                CplxElement::new(3),
                CplxElement::new(8)
            ],
            mapping: [(0, 2), (1, 2), (3, 4), (4, 7), (5, 7), (6, 7)]
                .into_iter()
                .copied()
                .collect(),
        }
    );
}
