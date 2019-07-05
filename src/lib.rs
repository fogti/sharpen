#![no_std]
#![cfg_attr(test, feature(test))]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate test;

pub mod classify_;

pub use classify_::{classify, classify_as_vec, Classify};
