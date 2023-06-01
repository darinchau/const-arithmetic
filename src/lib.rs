//! This crate is dedicated to implementing integer arithmetic that can be verified and used during compilation time

mod hex;
mod integer;
mod binary;

pub use hex::{_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, Hex, HexAdd, HexAdd3, HexMul, HexEqual, HexAssertEqual};
pub use binary::{Binary, BinAnd, BinEq, BinNor, BinNot, BinOr, AssertFalse, AssertTrue};
pub use integer::{TypedAssertEqual, TypedEqual, TypedGeq, TypedGreaterThan, TypedLessThan, TypedLeq, TypedInteger, TypedAdd, TypedSub, TypedMul, TypedDiv, IsInteger};

#[doc(hidden)]
pub use const_arith_macros_178::{parse_integer_inner, typed_assert_eq_inner};

/// Turns an integer (u32) into a typed integer object.
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// // Now `a` is a Typed integer object
/// 
/// // The expression must be a literal integer smaller than 4294967296 = 2**32
/// // let a = parse_integer!(-1); // This does not compile
/// // let a = parse_integer!(999999999999999999999999999999999); // This does not compile
/// const hiya: u32 = 5;
/// // let a = parse_integer!(hiya); // This does not compile
/// ```
#[macro_export]
macro_rules! parse_integer {
    ($s:expr) => {
        parse_integer_inner!($s)
    };
}

#[macro_export]
/// This asserts the typed integer provided is the same as the number you provided
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// typed_assert_eq!(a, 3);
/// ```
macro_rules! typed_assert_eq {
    ($a:ident, $val:expr) => {{
        typed_assert_eq_inner!($val);
        asserting(&$a);
    }};
}

#[cfg(test)]
mod test;
