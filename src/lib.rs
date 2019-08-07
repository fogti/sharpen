#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

pub mod classify_;

pub use classify_::{classify, classify_as_vec, Classify};
