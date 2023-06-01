//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod integer;
mod binary;

pub use hex::{_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, Hex};
pub use binary::{Binary, BinAnd, BinEq, BinNor, BinNot, BinOr, AssertFalse, AssertTrue};
pub use integer::{
    TypedAssertEqual, 
    TypedEqual, 
    TypedGeq, 
    TypedGreaterThan, 
    TypedLessThan, 
    TypedLeq, 
    TypedInteger, 
    TypedAdd, 
    TypedSub, 
    TypedMul, 
    TypedDiv, 
    IsInteger
};
pub use const_arith_macros::parse_integer;

#[cfg(test)]
mod test;

// For test purposes
fn div_equal<H, K, Q, R>(_a: H, _b: K, _c: Q, _d: R) where H: IsInteger, K: IsInteger, Q: IsInteger, R: IsInteger, H: TypedDiv<K, Output = Q, Remainder = R> {}
fn mul_equal<H, K, R, S>(_p: H, _q: K, _r: R, _s: S) where H: IsInteger, K: IsInteger, R: IsInteger, S: IsInteger, H: TypedMul<K, Output = S, Overflow = R> {}
fn less_than<H, K, R>(_p: H, _q: K) where H: IsInteger, K: IsInteger, R: Binary, H: TypedLessThan<K, Output = R>, R: AssertTrue {}
fn add_equal<H, K, S>(_a0: H, _a1: K, _a2: S) where H: IsInteger, K: IsInteger, S: IsInteger, H: TypedAdd<K, Output = S> {}
fn sub_equal<H, K, D>(_a0: H, _a1: K, _a2: D) where H: IsInteger, K: IsInteger, D: IsInteger, H: TypedSub<K, Output = D> {}
