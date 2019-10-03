use alloc::{collections::BTreeMap, vec::Vec};

#[cfg(test)]
mod tests;

pub trait Node: Sized {
    type ChildrenIter: Iterator<Item = Self>;
    fn take_children(&mut self) -> Self::ChildrenIter;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollapsedTree<T> {
    pub elems: Vec<T>,
    /// mapping from child to parent index
    pub mapping: BTreeMap<usize, usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CollapseTreeOrd {
    TopDown,
    BottomUp,
}

/**
Collapse a tree into a flat structure and a mapping.
This is the opposite function of [`rollup_tree`](crate::tree_rollup::rollup_tree).
**/
pub fn collapse_tree<T, InIter>(input: InIter, order: CollapseTreeOrd) -> CollapsedTree<T>
where
    T: Node,
    InIter: Iterator<Item = T>,
{
    let mut ret: CollapsedTree<T> = CollapsedTree {
        elems: Vec::with_capacity({
            let s_h = input.size_hint();
            s_h.1.unwrap_or(s_h.0)
        }),
        mapping: BTreeMap::new(),
    };
    let is_topdown = order == CollapseTreeOrd::TopDown;

    for mut i in input {
        // recursion
        let CollapsedTree {
            elems: subs_elems,
            mapping: subs_mapping,
        } = collapse_tree(i.take_children(), order);

        let mut i = Some(i);
        if is_topdown {
            // push current element
            ret.elems.push(i.take().unwrap());
        }

        // element positions for c2p mapping
        let (subs_cnt, subs_start_id) = (subs_elems.len(), ret.elems.len());

        if subs_cnt != 0 {
            // insert sub elements
            ret.elems.extend(subs_elems.into_iter());

            // update c2p mapping
            let cur_id = if is_topdown {
                subs_start_id - 1
            } else {
                ret.elems.len()
            };
            for i in 0..subs_cnt {
                ret.mapping.insert(
                    i + subs_start_id,
                    subs_mapping
                        .get(&i)
                        .map(|parent| *parent + subs_start_id)
                        .unwrap_or(cur_id),
                );
            }
        }

        if !is_topdown {
            // push current element
            ret.elems.push(i.take().unwrap());
        }
    }

    ret.elems.shrink_to_fit();
    ret
}
