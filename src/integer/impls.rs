use crate::hex::*;
use crate::binary::*;
use super::{IsInteger, TypedInteger};

// ==================================
// = Inner implementation for equal =
// ==================================
pub(crate) trait _Equal<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, 
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary,
R: Binary
> _Equal<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
H0: HexEqual<N::Hex0, Output = B0>,
H1: HexEqual<N::Hex1, Output = B1>,
H2: HexEqual<N::Hex2, Output = B2>,
H3: HexEqual<N::Hex3, Output = B3>,
H4: HexEqual<N::Hex4, Output = B4>,
H5: HexEqual<N::Hex5, Output = B5>,
H6: HexEqual<N::Hex6, Output = B6>,
H7: HexEqual<N::Hex7, Output = B7>,
B0: BinMultiAnd<B1, B2, B3, B4, B5, B6, B7, Output = R> {
    type Output = R;
}

// ======================================
// = Inner implementation for less than =
// ======================================
pub(crate) trait _LessThan<N: IsInteger> { type Output: Binary; }
impl<N: IsInteger,
    H0: Hex, R0: Hex, C0: Hex, X0: Hex,
    H1: Hex, R1: Hex, C1: Hex, X1: Hex,
    H2: Hex, R2: Hex, C2: Hex, X2: Hex,
    H3: Hex, R3: Hex, C3: Hex, X3: Hex,
    H4: Hex, R4: Hex, C4: Hex, X4: Hex,
    H5: Hex, R5: Hex, C5: Hex, X5: Hex,
    H6: Hex, R6: Hex, C6: Hex, X6: Hex,
    H7: Hex, R7: Hex, C7: Hex, X7: Hex,
    R_: Binary, Eq: Binary, NotEq: Binary, R: Binary,
> _LessThan<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
    // The implementation is a < b <=> a - b < 0 <=> a - b underflows
    N: _Equal<TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>, Output = Eq>,
    Eq: BinNot<Output = NotEq>,
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
    C7: HexEqual<_0, Output = R_>,
    R_: BinAnd<NotEq, Output = R>
{
    type Output = R;
}

// ================================
// = Inner implementation for leq =
// ================================
pub(crate) trait _Leq<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, 
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
B0: Binary, B1: Binary, B2: Binary
> _Leq<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _LessThan<N, Output = B0>,
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Equal<N, Output = B1>,
B0: BinOr<B1, Output = B2>
{
    type Output = B2;
}

// ================================
// = Inner implementation for geq =
// ================================
pub(crate) trait _Geq<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, 
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
B0: Binary, B1: Binary
> _Geq<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _LessThan<N, Output = B0>,
B0: BinNot<Output = B1>
{
    type Output = B1;
}

// =========================================
// = Inner implementation for greater than =
// =========================================
pub(crate) trait _GreaterThan<N: IsInteger> { type Output: Binary; }
impl <N: IsInteger, 
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
B0: Binary, B1: Binary, B2: Binary
> _GreaterThan<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _LessThan<N, Output = B0>,
TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>: _Equal<N, Output = B1>,
B0: BinNor<B1, Output = B2>
{
    type Output = B2;
}

// =====================================
// = Inner implementation for addition =
// =====================================
pub trait _Add<N: IsInteger> { type Output: IsInteger; }
impl<N,
    H0: Hex, R0: Hex, C0: Hex,
    H1: Hex, R1: Hex, C1: Hex,
    H2: Hex, R2: Hex, C2: Hex,
    H3: Hex, R3: Hex, C3: Hex,
    H4: Hex, R4: Hex, C4: Hex,
    H5: Hex, R5: Hex, C5: Hex,
    H6: Hex, R6: Hex, C6: Hex,
    H7: Hex, R7: Hex, C7: Hex
> _Add<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
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

// ========================================
// = Inner implementation for subtraction =
// ========================================
pub trait _Sub<N: IsInteger> { type Output: IsInteger; }
impl<N: IsInteger,
    H0: Hex, R0: Hex, C0: Hex, X0: Hex,
    H1: Hex, R1: Hex, C1: Hex, X1: Hex,
    H2: Hex, R2: Hex, C2: Hex, X2: Hex,
    H3: Hex, R3: Hex, C3: Hex, X3: Hex,
    H4: Hex, R4: Hex, C4: Hex, X4: Hex,
    H5: Hex, R5: Hex, C5: Hex, X5: Hex,
    H6: Hex, R6: Hex, C6: Hex, X6: Hex,
    H7: Hex, R7: Hex, C7: Hex, X7: Hex,
> _Sub<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
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

// ===========================================
// = Inner implementation for multiplication =
// ===========================================

// Implements the 16-bit multiplication. This ensures no overflow happen. 
// To ease implementation we multiply the first 4 hex of the Integer by the last 4 hex
// This makes it an unary operation on integers
pub(crate) trait _FoldMul { type Output: IsInteger; }
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
> _FoldMul for TypedInteger<H0, H1, H2, H3, K0, K1, K2, K3> where
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

// So for the normal multiplication, we multiply 16 bits by 16 bits, so nothing overflows
// Then stitch everything together
// This is the generic implementation
pub(crate) trait _Mul<N: IsInteger> {
    type Output: IsInteger;
    type Overflow: IsInteger;
}
impl<N: IsInteger,
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex,
N0: IsInteger, N1: IsInteger, N2: IsInteger, N3: IsInteger,
R4: Hex, R5: Hex, R6: Hex, R7: Hex, R8: Hex, R9: Hex, R10: Hex, R11: Hex, R12: Hex,
C4: Hex, C5: Hex, C6: Hex, C7: Hex, C8: Hex, C9: Hex, C10: Hex, C11: Hex, C_: Hex
> _Mul<N> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, H0, H1, H2, H3>: _FoldMul<Output = N0>,
TypedInteger<N::Hex0, N::Hex1, N::Hex2, N::Hex3, H4, H5, H6, H7>: _FoldMul<Output = N1>,
TypedInteger<N::Hex4, N::Hex5, N::Hex6, N::Hex7, H0, H1, H2, H3>: _FoldMul<Output = N2>,
TypedInteger<N::Hex4, N::Hex5, N::Hex6, N::Hex7, H4, H5, H6, H7>: _FoldMul<Output = N3>,
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


// =====================================
// = Inner implementation for division =
// =====================================

// How about a hex-based long division algorithm
