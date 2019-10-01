#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod classify_;
pub mod tree_rollup;

pub use crate::{
    classify_::{classify, classify_as_vec, Classify},
    tree_rollup::rollup_tree,
};
