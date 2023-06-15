use crate::hex::*;
use crate::binary::*;
use super::{IsInteger, TypedInteger};
use std::marker::PhantomData;

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

pub(crate) trait _Div<K: IsInteger> { type Output: IsInteger; type Remainder: IsInteger; }

// We can have a comparison of two binary numbers
struct BinaryNumber<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B30: Binary, B31: Binary> {
    _m0: PhantomData<B0>, 
    _m1: PhantomData<B1>, 
    _m2: PhantomData<B2>, 
    _m3: PhantomData<B3>, 
    _m4: PhantomData<B4>, 
    _m5: PhantomData<B5>, 
    _m6: PhantomData<B6>, 
    _m7: PhantomData<B7>, 
    _m8: PhantomData<B8>, 
    _m9: PhantomData<B9>, 
    _m10: PhantomData<B10>, 
    _m11: PhantomData<B11>, 
    _m12: PhantomData<B12>, 
    _m13: PhantomData<B13>, 
    _m14: PhantomData<B14>, 
    _m15: PhantomData<B15>, 
    _m16: PhantomData<B16>, 
    _m17: PhantomData<B17>, 
    _m18: PhantomData<B18>, 
    _m19: PhantomData<B19>, 
    _m20: PhantomData<B20>, 
    _m21: PhantomData<B21>, 
    _m22: PhantomData<B22>, 
    _m23: PhantomData<B23>, 
    _m24: PhantomData<B24>, 
    _m25: PhantomData<B25>, 
    _m26: PhantomData<B26>, 
    _m27: PhantomData<B27>, 
    _m28: PhantomData<B28>, 
    _m29: PhantomData<B29>, 
    _m30: PhantomData<B30>, 
    _m31: PhantomData<B31>, 
}

trait IsBinaryNumber {
    type B0: Binary; 
    type B1: Binary; 
    type B2: Binary; 
    type B3: Binary; 
    type B4: Binary; 
    type B5: Binary; 
    type B6: Binary; 
    type B7: Binary; 
    type B8: Binary; 
    type B9: Binary; 
    type B10: Binary; 
    type B11: Binary; 
    type B12: Binary; 
    type B13: Binary; 
    type B14: Binary; 
    type B15: Binary; 
    type B16: Binary; 
    type B17: Binary; 
    type B18: Binary; 
    type B19: Binary; 
    type B20: Binary; 
    type B21: Binary; 
    type B22: Binary; 
    type B23: Binary; 
    type B24: Binary; 
    type B25: Binary; 
    type B26: Binary; 
    type B27: Binary; 
    type B28: Binary; 
    type B29: Binary; 
    type B30: Binary; 
    type B31: Binary; 
}

impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B30: Binary, B31: Binary> 
IsBinaryNumber for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31> {
    type B0 = B0; 
    type B1 = B1; 
    type B2 = B2; 
    type B3 = B3; 
    type B4 = B4; 
    type B5 = B5; 
    type B6 = B6; 
    type B7 = B7; 
    type B8 = B8; 
    type B9 = B9; 
    type B10 = B10; 
    type B11 = B11; 
    type B12 = B12; 
    type B13 = B13; 
    type B14 = B14; 
    type B15 = B15; 
    type B16 = B16; 
    type B17 = B17; 
    type B18 = B18; 
    type B19 = B19; 
    type B20 = B20; 
    type B21 = B21; 
    type B22 = B22; 
    type B23 = B23; 
    type B24 = B24; 
    type B25 = B25; 
    type B26 = B26; 
    type B27 = B27; 
    type B28 = B28; 
    type B29 = B29; 
    type B30 = B30; 
    type B31 = B31; 
}

// A no-compromise conversion from hex to binary
impl<H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> IsBinaryNumber for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> {
    type B0 = H0::Bit0; 
    type B1 = H0::Bit1; 
    type B2 = H0::Bit2; 
    type B3 = H0::Bit3; 
    type B4 = H1::Bit0; 
    type B5 = H1::Bit1; 
    type B6 = H1::Bit2; 
    type B7 = H1::Bit3; 
    type B8 = H2::Bit0; 
    type B9 = H2::Bit1; 
    type B10 = H2::Bit2; 
    type B11 = H2::Bit3; 
    type B12 = H3::Bit0; 
    type B13 = H3::Bit1; 
    type B14 = H3::Bit2; 
    type B15 = H3::Bit3; 
    type B16 = H4::Bit0; 
    type B17 = H4::Bit1; 
    type B18 = H4::Bit2; 
    type B19 = H4::Bit3; 
    type B20 = H5::Bit0; 
    type B21 = H5::Bit1; 
    type B22 = H5::Bit2; 
    type B23 = H5::Bit3; 
    type B24 = H6::Bit0; 
    type B25 = H6::Bit1; 
    type B26 = H6::Bit2; 
    type B27 = H6::Bit3; 
    type B28 = H7::Bit0; 
    type B29 = H7::Bit1; 
    type B30 = H7::Bit2; 
    type B31 = H7::Bit3; 
}

// Implement greater equal for Binary numbers
trait _AssertGeq<B: IsBinaryNumber> {}
impl<B1: Binary, C1: Binary, B2: Binary, C2: Binary, B3: Binary, C3: Binary, B4: Binary, C4: Binary, B5: Binary, C5: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<_0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<_1, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B2: Binary, C2: Binary, B3: Binary, C3: Binary, B4: Binary, C4: Binary, B5: Binary, C5: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, _0, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, _1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B3: Binary, C3: Binary, B4: Binary, C4: Binary, B5: Binary, C5: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, _0, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, _1, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B4: Binary, C4: Binary, B5: Binary, C5: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, _0, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, _1, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B5: Binary, C5: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, _0, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, _1, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B6: Binary, C6: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, _0, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, _1, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B7: Binary, C7: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, _0, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, _1, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B8: Binary, C8: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, _0, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, _1, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B9: Binary, C9: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, _0, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, _1, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B10: Binary, C10: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, _0, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, _1, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B11: Binary, C11: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, _0, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, _1, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B12: Binary, C12: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, _0, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, _1, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B13: Binary, C13: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, _0, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, _1, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B14: Binary, C14: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, _0, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, _1, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B15: Binary, C15: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, _0, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, _1, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B16: Binary, C16: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, _0, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, _1, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B17: Binary, C17: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, _0, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, _1, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B18: Binary, C18: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, _0, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, _1, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B19: Binary, C19: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, _0, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, _1, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B20: Binary, C20: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, _0, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, _1, C20, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B21: Binary, C21: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, _0, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, _1, C21, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B22: Binary, C22: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, _0, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, _1, C22, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B23: Binary, C23: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, _0, B23, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, _1, C23, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B24: Binary, C24: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, _0, B24, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, _1, C24, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B25: Binary, C25: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, _0, B25, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, _1, C25, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B26: Binary, C26: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, _0, B26, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, _1, C26, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B27: Binary, C27: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, _0, B27, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, _1, C27, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B28: Binary, C28: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, _0, B28, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, _1, C28, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B29: Binary, C29: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, _0, B29, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, _1, C29, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B30: Binary, C30: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, _0, B30, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, _1, C30, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B31: Binary, C31: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, _0, B31>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, _1, C31> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B30: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _0>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _1> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B30: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _0>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _0> {}
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary, B8: Binary, B9: Binary, B10: Binary, B11: Binary, B12: Binary, B13: Binary, B14: Binary, B15: Binary, B16: Binary, B17: Binary, B18: Binary, B19: Binary, B20: Binary, B21: Binary, B22: Binary, B23: Binary, B24: Binary, B25: Binary, B26: Binary, B27: Binary, B28: Binary, B29: Binary, B30: Binary> _AssertGeq<BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _1>> for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, _1> {}

// Binary division psuedocode
// def long_division(H, K):
//    quotient = 0
//    h = H
//    for j in range(31, -1, -1):
//         if h >= (K << j):
//             quotient += 1 << j
//             h -= (K << j)
//    remainder = h
//    return quotient, remainder
//
// and here is the inner blob
trait _InnerDiv<H: IsInteger, K: IsInteger>
