//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod integer;

#[cfg(test)]
mod test;

// pub use hex::{_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, Hex, HexAdd1, HexAdd2, HexEqual};
pub use hex::*;
pub use integer::{TypedInteger, IsInteger, Add, Sub, Mul};
pub use const_arith_macros::parse_integer;

#[allow(unused_imports)]
use const_arith_macros::typed_assert_eq_inner;
/// This asserts the typed integer provided is the same as the number you provided
#[macro_export]
macro_rules! typed_assert_eq {
    ($a:ident, $val:expr) => {{
        typed_assert_eq_inner!($val);
        asserting(&$a);
    }};
}