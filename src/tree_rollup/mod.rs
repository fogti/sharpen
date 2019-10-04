use alloc::vec::Vec;

#[cfg(test)]
mod tests;

pub trait Node {
    /// Add a child to a `parent=self`;
    ///
    /// NOTE if used with [`rollup_tree`]:
    /// the children are pushed in reverse order and pushing is finished with a call to [`reverse`](ReversableNode::reverse)
    fn push_child(&mut self, child: Self);
}

pub trait ReversableNode: Node {
    /// Reverse the order of the children
    /// to match the input order
    fn reverse(&mut self);
}

/**
Roll up a tree given as a flat structure and a mapping {from child to parent}
into a hierarchical structure.

Invariants for the arguments:
* Any parent must come before it's children inside `input`.
* The largest index in `mapping` should be inside of the bounds of `input`.
* `mapping` keys (child) must be greater than the associated value (parent)
* `mapping` keys should be sorted

Return value:
* None: Detected duplicated usage of id's (probably the `mapping` was invalid).
* Some(iter): The rolled-up tree, with only top-level children left at top-level.
**/
#[inline]
pub fn rollup_tree<T, I, M>(input: I, mapping: M) -> Option<impl Iterator<Item = T>>
where
    T: ReversableNode,
    I: IntoIterator<Item = T>,
    M: IntoIterator<Item = (usize, usize)>,
    M::IntoIter: core::iter::DoubleEndedIterator,
{
    rollup_tree_intern(input.into_iter().map(Some).collect(), mapping.into_iter())
}

fn rollup_tree_intern<T, M>(mut v: Vec<Option<T>>, mapping: M) -> Option<impl Iterator<Item = T>>
where
    T: ReversableNode,
    M: Iterator<Item = (usize, usize)> + core::iter::DoubleEndedIterator,
{
    for (child_id, parent_id) in mapping.rev() {
        let mut child: T = core::mem::replace(v.get_mut(child_id)?, None)?;
        child.reverse();
        Node::push_child(v.get_mut(parent_id)?.as_mut()?, child);
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

/**
Roll up a tree given as a flat structure and a mapping {from child to parent}
into a hierarchical structure.

Invariants for the arguments:
* Any parent must come **after** it's children inside `input`.
* The largest index in `mapping` should be inside of the bounds of `input`.
* `mapping` keys (child) must be smaller than the associated value (parent)
* `mapping` keys should be sorted

Return value:
* None: Detected duplicated usage of id's (probably the `mapping` was invalid).
* Some(iter): The rolled-up tree, with only top-level children left at top-level.
**/
#[inline]
pub fn rollup_tree_bottomup<T, I, M>(input: I, mapping: M) -> Option<impl Iterator<Item = T>>
where
    T: Node,
    I: IntoIterator<Item = T>,
    M: IntoIterator<Item = (usize, usize)>,
{
    rollup_tree_bottomup_intern(input.into_iter().map(Some).collect(), mapping.into_iter())
}

fn rollup_tree_bottomup_intern<T, M>(
    mut v: Vec<Option<T>>,
    mapping: M,
) -> Option<impl Iterator<Item = T>>
where
    T: Node,
    M: Iterator<Item = (usize, usize)>,
{
    for (child_id, parent_id) in mapping {
        let child: T = core::mem::replace(v.get_mut(child_id)?, None)?;
        Node::push_child(v.get_mut(parent_id)?.as_mut()?, child);
    }
    Some(v.into_iter().filter_map(core::convert::identity))
}
