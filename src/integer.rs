//! This implements the integer trait which denotes a 32 bit unsigned integer


use super::binary::*;
use super::hex::*;
use std::marker::PhantomData;

mod impls;
use impls::*;

/// A struct which generics represents an unique integer from 0 to 2 ** 32 - 1
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// // a has type TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>
/// ```
#[derive(Clone, Copy)]
pub struct TypedInteger<H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> {
    _m0: PhantomData<H0>,
    _m1: PhantomData<H1>,
    _m2: PhantomData<H2>,
    _m3: PhantomData<H3>,
    _m4: PhantomData<H4>,
    _m5: PhantomData<H5>,
    _m6: PhantomData<H6>,
    _m7: PhantomData<H7>
}

impl<H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> {
    pub const fn new() -> Self {
        TypedInteger { 
            _m0: PhantomData, 
            _m1: PhantomData, 
            _m2: PhantomData, 
            _m3: PhantomData,
            _m4: PhantomData, 
            _m5: PhantomData, 
            _m6: PhantomData, 
            _m7: PhantomData 
        }
    }

    /// Returns the value of the Typed integer
    /// 
    /// Example
    /// ```
    /// use const_arithmetic::*;
    /// let a = parse_integer!(3);
    /// assert_eq!(a.number(), 3);
    /// ```
    pub const fn number() -> u32 {
        H0::NUMBER + 16 * H1::NUMBER + 256 * H2::NUMBER + 4096 * H3::NUMBER + 65536 * H4::NUMBER + 1048576 * H5::NUMBER + 16777216 * H6::NUMBER + 268435456 * H7::NUMBER
    }
}

/// A trait that denotes whether something is an integer
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a is 3
/// fn is_3<T>(_a: T) where
/// T: IsInteger,
/// T: TypedAssertEqual<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>>
/// {}
/// 
/// is_3(a);
/// ```
pub trait IsInteger: Copy {
    type Hex0: Hex; type Hex1: Hex; type Hex2: Hex; type Hex3: Hex; type Hex4: Hex; type Hex5: Hex; type Hex6: Hex; type Hex7: Hex;
    fn number() -> u32;
}

impl<H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> IsInteger for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> {
    type Hex0 = H0; type Hex1 = H1; type Hex2 = H2; type Hex3 = H3; type Hex4 = H4; type Hex5 = H5; type Hex6 = H6; type Hex7 = H7;
    fn number() -> u32 {
        H0::NUMBER + 16 * H1::NUMBER + 256 * H2::NUMBER + 4096 * H3::NUMBER + 65536 * H4::NUMBER + 1048576 * H5::NUMBER + 16777216 * H6::NUMBER + 268435456 * H7::NUMBER
    }
}

/// A trait that asserts two integers are equal
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a is 3
/// fn is_3<T>(_a: T) where
/// T: IsInteger,
/// T: TypedAssertEqual<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>>
/// {}
/// 
/// is_3(a);
/// ```
pub trait TypedAssertEqual<N: IsInteger> {}
impl<N: IsInteger> TypedAssertEqual<N> for TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, N::Hex4, N::Hex5, N::Hex6, N::Hex7> {}

/// A trait that returns a binary depending on whether two integers are equal
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a <= 5 but not a < 3
/// fn example<T, R, S>(_a: T) where
/// T: IsInteger,
/// R: Binary,
/// S: Binary,
/// T: TypedEqual<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>, Output = R>,
/// R: AssertTrue,
/// T: TypedEqual<TypedInteger<_5, _0, _0, _0, _0, _0, _0, _0>, Output = S>,
/// S: AssertFalse
/// {}
/// 
/// example(a);
/// ```
pub trait TypedEqual<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Binary> TypedEqual<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
N: _Equal<TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>, Output = R> {
    type Output = R;
}

/// A trait that returns a binary depending on whether a < b
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a <= 5 but not a < 3
/// fn example<T, R, S>(_a: T) where
/// T: IsInteger,
/// R: Binary,
/// S: Binary,
/// T: TypedLessThan<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>, Output = R>,
/// R: AssertFalse,
/// T: TypedLessThan<TypedInteger<_5, _0, _0, _0, _0, _0, _0, _0>, Output = S>,
/// S: AssertTrue
/// {}
/// 
/// example(a);
/// ```
pub trait TypedLessThan<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Binary> TypedLessThan<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _LessThan<N, Output = R> {
    type Output = R;
}

/// A trait that returns a binary depending on whether two integers are less or equal
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a <= 5 but not a < 3
/// fn example<T, R, S>(_a: T) where
/// T: IsInteger,
/// R: Binary,
/// S: Binary,
/// T: TypedLeq<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>, Output = R>,
/// R: AssertTrue,
/// T: TypedLeq<TypedInteger<_5, _0, _0, _0, _0, _0, _0, _0>, Output = S>,
/// S: AssertTrue
/// {}
/// 
/// example(a);
/// ```
pub trait TypedLeq<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Binary> TypedLeq<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Leq<N, Output = R> {
    type Output = R;
}

/// A trait that returns a binary depending on whether two integers are greater or equal
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a <= 5 but not a < 3
/// fn example<T, R, S>(_a: T) where
/// T: IsInteger,
/// R: Binary,
/// S: Binary,
/// T: TypedGeq<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>, Output = R>,
/// R: AssertTrue,
/// T: TypedGeq<TypedInteger<_5, _0, _0, _0, _0, _0, _0, _0>, Output = S>,
/// S: AssertFalse
/// {}
/// 
/// example(a);
/// ```
pub trait TypedGeq<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Binary> TypedGeq<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Geq<N, Output = R> {
    type Output = R;
}

/// A trait that returns a binary depending on whether a > b
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// 
/// // This verifies that a <= 5 but not a < 3
/// fn example<T, R, S>(_a: T) where
/// T: IsInteger,
/// R: Binary,
/// S: Binary,
/// T: TypedGreaterThan<TypedInteger<_3, _0, _0, _0, _0, _0, _0, _0>, Output = R>,
/// R: AssertFalse,
/// T: TypedGreaterThan<TypedInteger<_1, _0, _0, _0, _0, _0, _0, _0>, Output = S>,
/// S: AssertTrue
/// {}
/// 
/// example(a);
/// ```
pub trait TypedGreaterThan<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Binary> TypedGreaterThan<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _GreaterThan<N, Output = R> {
    type Output = R;
}

/// Denotes integer addition. If this says C7 does not implement HexAssertEq, this means it overflowed.
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(3);
/// let b = parse_integer!(5);
/// let c = parse_integer!(8);
/// 
/// // This verifies that 3 + 5 = 8
/// fn example<P, Q, R>(_p: P, _q: Q, _r: R) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// P: TypedAdd<Q, Output = R>
/// {}
/// 
/// example(a, b, c);
/// 
/// // This is another way of implementing the above
/// fn example2<P, Q, R, S, T>(_p: P, _q: Q, _r: R) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// S: IsInteger,
/// T: Binary,
/// P: TypedAdd<Q, Output = S>,
/// R: TypedEqual<S, Output = T>,
/// T: AssertTrue {}
/// example2(a, b, c);
/// ```
pub trait TypedAdd<N: IsInteger> { type Output: IsInteger; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: IsInteger> TypedAdd<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Add<N, Output = R> {
    type Output = R;
}

/// Denotes integer subtraction. If this says C7 does not implement HexAssertEq, this means it underflowed.
///
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(7);
/// let b = parse_integer!(4);
/// let c = parse_integer!(3);
/// 
/// // This verifies that 7 - 4 = 3
/// fn example<P, Q, R>(_p: P, _q: Q, _r: R) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// P: TypedSub<Q, Output = R>
/// {}
/// 
/// example(a, b, c);
/// ```
pub trait TypedSub<N: IsInteger> { type Output: IsInteger; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: IsInteger> TypedSub<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Sub<N, Output = R> {
    type Output = R;
}



/// A multiplication of 32 bit number and 32 bit number can be stored safely in a 64 bit number. We represent them as lower 32 bits and upper 32 bits
///
/// Example
/// ```
/// use const_arithmetic::*;
/// let a = parse_integer!(6);
/// let b = parse_integer!(4);
/// let c = parse_integer!(24);
/// 
/// // This verifies that 6 * 4 = 24
/// fn example<P, Q, R>(_p: P, _q: Q, _r: R) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// P: TypedMul<Q, Output = R>
/// {}
/// 
/// example(a, b, c);
/// 
/// // If we are handling really big integers we can get the overflowed bits as follows
/// // 1234567890 * 987654321 = 1219326311126352690 = 283896529 * (2**32) + 3623437106
/// let a = parse_integer!(1234567890);
/// let b = parse_integer!(987654321);
/// let overflow = parse_integer!(283896529);
/// let result = parse_integer!(3623437106);
/// 
/// // This verifies that 7 - 4 = 3
/// fn example2<P, Q, R, S>(_p: P, _q: Q, _r: R, _s: S) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// S: IsInteger,
/// P: TypedMul<Q, Output = R, Overflow = S>
/// {}
/// 
/// example2(a, b, result, overflow);
/// ```
pub trait TypedMul<N: IsInteger> { type Output: IsInteger; type Overflow: IsInteger; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: IsInteger, O: IsInteger> TypedMul<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Mul<N, Output = R, Overflow = O> {
    type Output = R;
    type Overflow = O;
}


// TODO: We can actually implement bitshift to optimize division

/// Returns the Quotient of H/K for H: Div<K, Output: ...>
/// 
/// Note about implementation detail: This is an expanded version of long division - it takes O(1) steps but the constant is quite big unfortunately
/// (to be precise its 32 multiplications plus a negligible amount of addition and subtraction and other stuff)
/// If there were many divisions, the compile time increases quite a bit
/// 
/// Example
/// ```
/// // (This doc test takes about 5 seconds to complete on my computer)
/// use const_arithmetic::*;
/// let a = parse_integer!(25);
/// let b = parse_integer!(4);
/// let quotient = parse_integer!(6);
/// let modulus = parse_integer!(1);
/// 
/// // This verifies that 25/4 = 6 ... 1
/// fn example<P, Q, R, S>(_p: P, _q: Q, _r: R, _s: S) where
/// P: IsInteger,
/// Q: IsInteger,
/// R: IsInteger,
/// S: IsInteger,
/// P: TypedDiv<Q, Output = R, Remainder = S>
/// {}
/// 
/// example(a, b, quotient, modulus);
pub trait TypedDiv<K: IsInteger> { type Output: IsInteger; type Remainder: IsInteger; }
impl <N: IsInteger, H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: IsInteger, O: IsInteger> TypedDiv<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Div<N, Output = R, Remainder = O> {
    type Output = R;
    type Remainder = O;
}
