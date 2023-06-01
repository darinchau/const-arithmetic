//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod integer;
mod binary;

pub use hex::{_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, Hex, HexAdd, HexAdd3, HexMul, HexEqual, HexAssertEqual};
pub use binary::{Binary, BinAnd, BinEq, BinNor, BinNot, BinOr, AssertFalse, AssertTrue};
pub use integer::{TypedAssertEqual, TypedEqual, TypedGeq, TypedGreaterThan, TypedLessThan, TypedLeq, TypedInteger, TypedAdd, TypedSub, TypedMul, TypedDiv, IsInteger};
pub use const_arith_macros::parse_integer_inner;


#[cfg(test)]
mod test;
