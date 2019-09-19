#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod classify_;
pub use classify_::{classify, classify_as_vec, Classify};
