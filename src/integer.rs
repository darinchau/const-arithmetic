//! This implements the integer trait which denotes a 32 bit unsigned integer

use super::hex::*;
use std::marker::PhantomData;
use super::{HexAdd, HexAdd3};

/// A struct which generics represents an unique integer from 0 to 2 ** 32 - 1
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

    pub const fn number() -> u32 {
        H0::NUMBER + 16 * H1::NUMBER + 256 * H2::NUMBER + 4096 * H3::NUMBER + 65536 * H4::NUMBER + 1048576 * H5::NUMBER + 16777216 * H6::NUMBER + 268435456 * H7::NUMBER
    }
}

/// A trait that denotes whether something is an integer
pub trait IsInteger {
    type Hex0: Hex;
    type Hex1: Hex;
    type Hex2: Hex;
    type Hex3: Hex;
    type Hex4: Hex;
    type Hex5: Hex;
    type Hex6: Hex;
    type Hex7: Hex;

    fn number(&self) -> u32;
}
impl<H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> IsInteger for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> {
    type Hex0 = H0;
    type Hex1 = H1;
    type Hex2 = H2;
    type Hex3 = H3;
    type Hex4 = H4;
    type Hex5 = H5;
    type Hex6 = H6;
    type Hex7 = H7;

    fn number(&self) -> u32 {
        H0::NUMBER + 16 * H1::NUMBER + 256 * H2::NUMBER + 4096 * H3::NUMBER + 65536 * H4::NUMBER + 1048576 * H5::NUMBER + 16777216 * H6::NUMBER + 268435456 * H7::NUMBER
    }
}

/// A trait that asserts two integers are equal
pub trait TypedAssertEqual<N: IsInteger> {}
impl<N: IsInteger> TypedAssertEqual<N> for TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, N::Hex4, N::Hex5, N::Hex6, N::Hex7> {}

/// Denotes integer addition. If this says C7 does not implement HexAssertEq, this means it overflowed.
pub trait Add<N: IsInteger> { type Output: IsInteger; }
impl<N,
    H0: Hex, R0: Hex, C0: Hex,
    H1: Hex, R1: Hex, C1: Hex,
    H2: Hex, R2: Hex, C2: Hex,
    H3: Hex, R3: Hex, C3: Hex,
    H4: Hex, R4: Hex, C4: Hex,
    H5: Hex, R5: Hex, C5: Hex,
    H6: Hex, R6: Hex, C6: Hex,
    H7: Hex, R7: Hex, C7: Hex
> Add<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
    N: IsInteger,
    H0: HexAdd<N::Hex0, Output = R0, Carry = C0>,
    H1: HexAdd3<N::Hex1, C0, Output = R1, Carry = C1>,
    H2: HexAdd3<N::Hex2, C1, Output = R2, Carry = C2>,
    H3: HexAdd3<N::Hex3, C2, Output = R3, Carry = C3>,
    H4: HexAdd3<N::Hex4, C3, Output = R4, Carry = C4>,
    H5: HexAdd3<N::Hex5, C4, Output = R5, Carry = C5>,
    H6: HexAdd3<N::Hex6, C5, Output = R6, Carry = C6>,
    H7: HexAdd3<N::Hex7, C6, Output = R7, Carry = C7>,
    C7: HexAssertEqual<_0>
{
    type Output = TypedInteger<R0, R1, R2, R3, R4, R5, R6, R7>;
}

/// Denotes integer subtraction. If this says C7 does not implement HexAssertEq, this means it underflowed.
pub trait Sub<N: IsInteger> { type Output: IsInteger; }
impl<N: IsInteger,
    H0: Hex, R0: Hex, C0: Hex, X0: Hex,
    H1: Hex, R1: Hex, C1: Hex, X1: Hex,
    H2: Hex, R2: Hex, C2: Hex, X2: Hex,
    H3: Hex, R3: Hex, C3: Hex, X3: Hex,
    H4: Hex, R4: Hex, C4: Hex, X4: Hex,
    H5: Hex, R5: Hex, C5: Hex, X5: Hex,
    H6: Hex, R6: Hex, C6: Hex, X6: Hex,
    H7: Hex, R7: Hex, C7: Hex, X7: Hex,
> Sub<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
    // The implementation takes advantage of overflowing, a - b = a + (2 ** 32 - 1 - b) + 1 = a - b + 2 ** 32 (mod 2 **32)
    N::Hex0: HexNot<Output = X0>,
    N::Hex1: HexNot<Output = X1>,
    N::Hex2: HexNot<Output = X2>,
    N::Hex3: HexNot<Output = X3>,
    N::Hex4: HexNot<Output = X4>,
    N::Hex5: HexNot<Output = X5>,
    N::Hex6: HexNot<Output = X6>,
    N::Hex7: HexNot<Output = X7>,
    H0: HexAdd3<X0, _1, Output = R0, Carry = C0>,
    H1: HexAdd3<X1, C0, Output = R1, Carry = C1>,
    H2: HexAdd3<X2, C1, Output = R2, Carry = C2>,
    H3: HexAdd3<X3, C2, Output = R3, Carry = C3>,
    H4: HexAdd3<X4, C3, Output = R4, Carry = C4>,
    H5: HexAdd3<X5, C4, Output = R5, Carry = C5>,
    H6: HexAdd3<X6, C5, Output = R6, Carry = C6>,
    H7: HexAdd3<X7, C6, Output = R7, Carry = C7>,
    C7: HexAssertEqual<_1>
{
    type Output = TypedInteger<R0, R1, R2, R3, R4, R5, R6, R7>;
}

/// Implements the 16-bit multiplication. This ensures no overflow happen. 
/// To ease implementation we multiply the first 4 hex of the Integer by the last 4 hex
/// This makes it an unary operation on integers
pub trait FoldMul { type Output: IsInteger; }
impl<
H0: Hex, H1: Hex, H2: Hex, H3: Hex, 
K0: Hex, K1: Hex, K2: Hex, K3: Hex,
X00: Hex, C00: Hex,
X01: Hex, C01: Hex,
X02: Hex, C02: Hex,
X03: Hex, C03: Hex,
X10: Hex, C10: Hex,
X11: Hex, C11: Hex,
X12: Hex, C12: Hex,
X13: Hex, C13: Hex,
X20: Hex, C20: Hex,
X21: Hex, C21: Hex,
X22: Hex, C22: Hex,
X23: Hex, C23: Hex,
X30: Hex, C30: Hex,
X31: Hex, C31: Hex,
X32: Hex, C32: Hex,
X33: Hex, C33: Hex,
C1: Hex, C2: Hex, C3: Hex, C4: Hex, C5: Hex, C6: Hex, C_: Hex,
R1: Hex, R2: Hex, R3: Hex, R4: Hex, R5: Hex, R6: Hex, R7: Hex,
> FoldMul for TypedInteger<H0, H1, H2, H3, K0, K1, K2, K3> where
// Multiplication part
H0: HexMul<K0, Output = X00, Carry = C00>,
H0: HexMul<K1, Output = X01, Carry = C01>,
H0: HexMul<K2, Output = X02, Carry = C02>,
H0: HexMul<K3, Output = X03, Carry = C03>,
H1: HexMul<K0, Output = X10, Carry = C10>,
H1: HexMul<K1, Output = X11, Carry = C11>,
H1: HexMul<K2, Output = X12, Carry = C12>,
H1: HexMul<K3, Output = X13, Carry = C13>,
H2: HexMul<K0, Output = X20, Carry = C20>,
H2: HexMul<K1, Output = X21, Carry = C21>,
H2: HexMul<K2, Output = X22, Carry = C22>,
H2: HexMul<K3, Output = X23, Carry = C23>,
H3: HexMul<K0, Output = X30, Carry = C30>,
H3: HexMul<K1, Output = X31, Carry = C31>,
H3: HexMul<K2, Output = X32, Carry = C32>,
H3: HexMul<K3, Output = X33, Carry = C33>,
// Addition part
C00: HexAdd3<X01, X10, Output = R1, Carry = C1>,
C01: HexAdd6<X02, C10, X11, X20, C1, Output = R2, Carry = C2>,
C02: HexAdd8<X03, X12, C11, C20, X21, X30, C2, Output = R3, Carry = C3>,
C03: HexAdd8<C12, X13, X22, C21, C30, X31, C3, Output = R4, Carry = C4>,
C13: HexAdd6<C22, X23, X32, C31, C4, Output = R5, Carry = C5>,
C23: HexAdd4<C32, X33, C5, Output = R6, Carry = C6>,
C33: HexAdd<C6, Output = R7, Carry = C_>,
C_: HexAssertEqual<_0>
{
    type Output = TypedInteger<X00, R1, R2, R3, R4, R5, R6, R7>;
}

/// A multiplication of 32 bit number and 32 bit number can be stored safely in a 64 bit number. We represent them as lower 32 bits and upper 32 bits
pub trait Mul<N: IsInteger> {
    type Output: IsInteger;
    type Overflow: IsInteger;
}
impl<N: IsInteger,
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
N0: IsInteger, N1: IsInteger, N2: IsInteger, N3: IsInteger,
R4: Hex, R5: Hex, R6: Hex, R7: Hex, R8: Hex, R9: Hex, R10: Hex, R11: Hex, R12: Hex,
C4: Hex, C5: Hex, C6: Hex, C7: Hex, C8: Hex, C9: Hex, C10: Hex, C11: Hex, C_: Hex
> Mul<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, H0, H1, H2, H3>: FoldMul<Output = N0>,
TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, H4, H5, H6, H7>: FoldMul<Output = N1>,
TypedInteger<N::Hex4, N::Hex5, N::Hex6, N::Hex7, H0, H1, H2, H3>: FoldMul<Output = N2>,
TypedInteger<N::Hex4, N::Hex5, N::Hex6, N::Hex7, H4, H5, H6, H7>: FoldMul<Output = N3>,
N0::Hex4: HexAdd3<N1::Hex0, N2::Hex0, Output = R4, Carry = C4>,
N0::Hex5: HexAdd4<N1::Hex1, N2::Hex1, C4, Output = R5, Carry = C5>,
N0::Hex6: HexAdd4<N1::Hex2, N2::Hex2, C5, Output = R6, Carry = C6>,
N0::Hex7: HexAdd4<N1::Hex3, N2::Hex3, C6, Output = R7, Carry = C7>,
N3::Hex0: HexAdd4<N1::Hex4, N2::Hex4, C7, Output = R8, Carry = C8>,
N3::Hex1: HexAdd4<N1::Hex5, N2::Hex5, C8, Output = R9, Carry = C9>,
N3::Hex2: HexAdd4<N1::Hex6, N2::Hex6, C9, Output = R10, Carry = C10>,
N3::Hex3: HexAdd4<N1::Hex7, N2::Hex7, C10, Output = R11, Carry = C11>,
N3::Hex4: HexAdd<C11, Output = R12, Carry = C_>,
C_: HexAssertEqual<_0>
{
    type Output = TypedInteger<N0::Hex0, N0::Hex1, N0::Hex2, N0::Hex3, R4, R5, R6, R7>;
    type Overflow = TypedInteger<R8, R9, R10, R11, R12, N3::Hex5, N3::Hex6, N3::Hex7>;
}
