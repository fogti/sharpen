use super::*;
use alloc::vec;

#[derive(Debug, PartialEq, Eq)]
struct Element(usize);

impl Node for Element {
    fn push_child(&mut self, child: Element) {
        self.0 += child.0;
    }
    fn reverse(&mut self) {}
}

#[test]
fn test_rollup_tree() {
    let input = vec![Element(0), Element(1), Element(2), Element(3), Element(4)];

    let mut mapping = BTreeMap::new();
    mapping.insert(1, 0);
    mapping.insert(3, 2);
    mapping.insert(4, 1);

    let result: Vec<_> = rollup_tree(input.into_iter(), &mapping)
        .expect("valid mapping")
        .collect();
    assert_eq!(result, vec![Element(5), Element(5),]);
}
