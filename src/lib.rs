//! This crate is dedicated to reimplementing integers using less recursion than trait-eval. We are essentially implementing a computer anyway

mod hex;
mod integer;
mod binary;

#[cfg(test)]
mod test;

pub use hex::*;
pub use binary::*;
pub use integer::*;
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
