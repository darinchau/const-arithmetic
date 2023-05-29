//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod equal;
mod add;
mod integer;

pub use hex::*;
pub use equal::HexEqual;
pub use integer::{Integer, IsInteger, Add};


#[cfg(test)]
mod test {

}