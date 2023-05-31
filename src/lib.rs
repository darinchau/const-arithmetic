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

// For test purposes
fn div_equal<H, K, Q, R>(_a: H, _b: K, _c: Q, _d: R) where H: IsInteger, K: IsInteger, Q: IsInteger, R: IsInteger, H: Div<K, Output = Q, Remainder = R> {}
fn mul_equal<H, K, R, S>(_p: H, _q: K, _r: R, _s: S) where H: IsInteger, K: IsInteger, R: IsInteger, S: IsInteger, H: Mul<K, Output = S, Overflow = R> {}
fn less_than<H, K, R>(_p: H, _q: K) where H: IsInteger, K: IsInteger, R: Binary, H: TypedLessThan<K, Output = R>, R: AssertTrue {}
fn add_equal<H, K, S>(_a0: H, _a1: K, _a2: S) where H: IsInteger, K: IsInteger, S: IsInteger, H: Add<K, Output = S> {}
fn sub_equal<H, K, D>(_a0: H, _a1: K, _a2: D) where H: IsInteger, K: IsInteger, D: IsInteger, H: Sub<K, Output = D> {}
