//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod integer;
mod binary;

pub use hex::*;
pub use binary::*;
pub use integer::*;
pub use const_arith_macros::parse_integer;

#[cfg(test)]
mod test;
