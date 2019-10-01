use alloc::{collections::BTreeMap, vec::Vec};

#[cfg(test)]
mod tests;

/**
Roll up a tree given as a flat structure and a mapping {from child to parent}
into a hierarchical structure.

Invariants for the arguments:
* Any parent must come before it's children inside `input`.
* The largest index in `mapping` should be inside of the bounds of `input`.
* `mapping` keys (child) must be greater than the associated value (parent)

Return value:
* None: Detected duplicated usage of id's (probably the `mapping` was invalid).
* Some(collected): The rolled-up tree, with only top-level children left at top-level.
**/
#[inline]
pub fn rollup_tree<T, I>(input: I, mapping: &Mapping) -> Option<impl Iterator<Item = T>>
where
    T: Node,
    I: IntoIterator<Item = T>,
{
    rollup_tree_intern(input.into_iter().map(Some).collect(), mapping)
}

pub trait Node {
    /// Add a child to a `parent=self`;
    /// NOTE: the children are pushed in reverse order
    fn push_child(&mut self, child: Self);

    /// Reverse the order of the children
    /// to match the input order
    fn reverse(&mut self);
}

type Mapping = BTreeMap<usize, usize>;

fn rollup_tree_intern<T: Node>(
    mut v: Vec<Option<T>>,
    mapping: &Mapping,
) -> Option<impl Iterator<Item = T>> {
    for (child_id, parent_id) in mapping.iter().rev() {
        let mut child: T = core::mem::replace(v.get_mut(*child_id)?, None)?;
        child.reverse();
        Node::push_child(v.get_mut(*parent_id)?.as_mut()?, child);
    }
    Some(
        v.into_iter()
            .filter_map(core::convert::identity)
            .map(|mut i| {
                i.reverse();
                i
            }),
    )
}
