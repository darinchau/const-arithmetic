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


// Inner implementation for division
// Helper stuff for division
// Helper if clauses. Condition: If<TrueValue: T, FalseValue: T, Output = T>
pub(crate) trait If<T: IsInteger, F: IsInteger> { type Output: IsInteger; }
impl<T: IsInteger, F: IsInteger> If<T, F> for _0 { type Output = F; }
impl<T: IsInteger, F: IsInteger> If<T, F> for _1 { type Output = T; }

// Helper types for division
type _Zero = TypedInteger<_0, _0, _0, _0, _0, _0, _0, _0>;

// One single iteration of long div
pub(crate) trait DivInner<H: IsInteger, Q: IsInteger, TT: IsInteger> { type Hout: IsInteger; type Qout: IsInteger; }
impl<H, Q, H0, H1, H2, H3, H4, H5, H6, H7, J16, A, O, C, D, MinusMe, Bx, By, Ho, Qo> DivInner<H, Q, J16> for TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7> where
H: IsInteger, 
Q: IsInteger,
O: IsInteger,
Ho: IsInteger,
Qo: IsInteger,
H0: Hex,
H1: Hex, 
H2: Hex, 
H3: Hex, 
H4: Hex, 
H5: Hex, 
H6: Hex, 
H7: Hex, 
J16: IsInteger, 
A: IsInteger, 
C: IsInteger, 
D: IsInteger,
Bx: Binary,
By: Binary,
MinusMe: Binary,
// minus_me = h >= 16 ** j * K
// h -= 16 ** j * K * minus_me
// quotient += 16 ** j * minus_me
//
// This becomes
//
// 16**j -> J16
// J16 * K -> A, overflow = O
// Overflow == 0 -> Bx
// H >= A -> By
// Bx and By -> MinusMe
// A if MinusMe else 0 -> C
// H - C -> Hout
// J16 if MinusMe else 0 -> D
// Q + D -> Qout
J16: _Mul<TypedInteger<H0, H1, H2, H3, H4, H5, H6, H7>, Output = A, Overflow = O>,
O: _Equal<_Zero, Output = Bx>,
A: _Leq<H, Output = By>,
Bx: BinAnd<By, Output = MinusMe>, 
MinusMe: If<A, _Zero, Output = C>, 
MinusMe: If<J16, _Zero, Output = D>, 
H: _Sub<C, Output = Ho>, 
D: _Add<Q, Output = Qo> {
    type Hout = Ho;
    type Qout = Qo;
}

pub(crate) trait _Div<K: IsInteger> { type Output: IsInteger; type Remainder: IsInteger; }
impl< K: IsInteger, 
Hx0: Hex, Hx1: Hex, Hx2: Hex, Hx3: Hex, Hx4: Hex, Hx5: Hex, Hx6: Hex, Hx7: Hex, 
H1: IsInteger, Q1: IsInteger, 
H2: IsInteger, Q2: IsInteger, 
H3: IsInteger, Q3: IsInteger, 
H4: IsInteger, Q4: IsInteger, 
H5: IsInteger, Q5: IsInteger, 
H6: IsInteger, Q6: IsInteger, 
H7: IsInteger, Q7: IsInteger, 
H8: IsInteger, Q8: IsInteger, 
H9: IsInteger, Q9: IsInteger, 
H10: IsInteger, Q10: IsInteger, 
H11: IsInteger, Q11: IsInteger, 
H12: IsInteger, Q12: IsInteger, 
H13: IsInteger, Q13: IsInteger, 
H14: IsInteger, Q14: IsInteger, 
H15: IsInteger, Q15: IsInteger, 
H16: IsInteger, Q16: IsInteger, 
H17: IsInteger, Q17: IsInteger, 
H18: IsInteger, Q18: IsInteger, 
H19: IsInteger, Q19: IsInteger, 
H20: IsInteger, Q20: IsInteger, 
H21: IsInteger, Q21: IsInteger, 
H22: IsInteger, Q22: IsInteger, 
H23: IsInteger, Q23: IsInteger, 
H24: IsInteger, Q24: IsInteger, 
H25: IsInteger, Q25: IsInteger, 
H26: IsInteger, Q26: IsInteger, 
H27: IsInteger, Q27: IsInteger, 
H28: IsInteger, Q28: IsInteger, 
H29: IsInteger, Q29: IsInteger, 
H30: IsInteger, Q30: IsInteger, 
H31: IsInteger, Q31: IsInteger, 
H32: IsInteger, Q32: IsInteger, 
// For special case
B: Binary, Eq1: Binary, Eq0: Binary,
H33: IsInteger, Q33: IsInteger,
HResult: IsInteger, QResult: IsInteger
> _Div<K> for TypedInteger<Hx0, Hx1, Hx2, Hx3, Hx4, Hx5, Hx6, Hx7> where
// Here is some psuedocode in python
// # Actual psuedocode for division
// def long_division(H, K):
//     quotient = 0
//     h = H
//     for j in range(7, -1, -1):
//         while h >= 16 ** j * K:
//             h -= 16 ** j * K
//             quotient += 16 ** j
//     remainder = h
//     return quotient, remainder

// # Implementation psuedocode for Division
// def long_division(H, K):
//     quotient = 0
//     h = H
//     for j in (7, 6, 5, 4, 3, 2, 1, 0):
//         for x in (8, 4, 2, 1):
//             minus_me = h >= 16 ** j * x * K
//             h -= 16 ** j * x * K * minus_me
//             quotient += 16 ** j * x * minus_me
//     remainder = h
//     return quotient, remainder

// Make sure k is not 0
K: _GreaterThan<_Zero, Output = B>,
B: AssertTrue,

// Long division
K: DivInner<TypedInteger<Hx0, Hx1, Hx2, Hx3, Hx4, Hx5, Hx6, Hx7>, _Zero, TypedInteger<_0, _0, _0, _0, _0, _0, _0, _8>, Hout = H1, Qout = Q1>, 
K: DivInner<H1, Q1, TypedInteger<_0, _0, _0, _0, _0, _0, _0, _4>, Hout = H2, Qout = Q2>, 
K: DivInner<H2, Q2, TypedInteger<_0, _0, _0, _0, _0, _0, _0, _2>, Hout = H3, Qout = Q3>, 
K: DivInner<H3, Q3, TypedInteger<_0, _0, _0, _0, _0, _0, _0, _1>, Hout = H4, Qout = Q4>, 
K: DivInner<H4, Q4, TypedInteger<_0, _0, _0, _0, _0, _0, _8, _0>, Hout = H5, Qout = Q5>, 
K: DivInner<H5, Q5, TypedInteger<_0, _0, _0, _0, _0, _0, _4, _0>, Hout = H6, Qout = Q6>,
K: DivInner<H6, Q6, TypedInteger<_0, _0, _0, _0, _0, _0, _2, _0>, Hout = H7, Qout = Q7>, 
K: DivInner<H7, Q7, TypedInteger<_0, _0, _0, _0, _0, _0, _1, _0>, Hout = H8, Qout = Q8>, 
K: DivInner<H8, Q8, TypedInteger<_0, _0, _0, _0, _0, _8, _0, _0>, Hout = H9, Qout = Q9>, 
K: DivInner<H9, Q9, TypedInteger<_0, _0, _0, _0, _0, _4, _0, _0>, Hout = H10, Qout = Q10>, 
K: DivInner<H10, Q10, TypedInteger<_0, _0, _0, _0, _0, _2, _0, _0>, Hout = H11, Qout = Q11>, 
K: DivInner<H11, Q11, TypedInteger<_0, _0, _0, _0, _0, _1, _0, _0>, Hout = H12, Qout = Q12>, 
K: DivInner<H12, Q12, TypedInteger<_0, _0, _0, _0, _8, _0, _0, _0>, Hout = H13, Qout = Q13>, 
K: DivInner<H13, Q13, TypedInteger<_0, _0, _0, _0, _4, _0, _0, _0>, Hout = H14, Qout = Q14>, 
K: DivInner<H14, Q14, TypedInteger<_0, _0, _0, _0, _2, _0, _0, _0>, Hout = H15, Qout = Q15>, 
K: DivInner<H15, Q15, TypedInteger<_0, _0, _0, _0, _1, _0, _0, _0>, Hout = H16, Qout = Q16>, 
K: DivInner<H16, Q16, TypedInteger<_0, _0, _0, _8, _0, _0, _0, _0>, Hout = H17, Qout = Q17>, 
K: DivInner<H17, Q17, TypedInteger<_0, _0, _0, _4, _0, _0, _0, _0>, Hout = H18, Qout = Q18>, 
K: DivInner<H18, Q18, TypedInteger<_0, _0, _0, _2, _0, _0, _0, _0>, Hout = H19, Qout = Q19>, 
K: DivInner<H19, Q19, TypedInteger<_0, _0, _0, _1, _0, _0, _0, _0>, Hout = H20, Qout = Q20>, 
K: DivInner<H20, Q20, TypedInteger<_0, _0, _8, _0, _0, _0, _0, _0>, Hout = H21, Qout = Q21>, 
K: DivInner<H21, Q21, TypedInteger<_0, _0, _4, _0, _0, _0, _0, _0>, Hout = H22, Qout = Q22>, 
K: DivInner<H22, Q22, TypedInteger<_0, _0, _2, _0, _0, _0, _0, _0>, Hout = H23, Qout = Q23>, 
K: DivInner<H23, Q23, TypedInteger<_0, _0, _1, _0, _0, _0, _0, _0>, Hout = H24, Qout = Q24>, 
K: DivInner<H24, Q24, TypedInteger<_0, _8, _0, _0, _0, _0, _0, _0>, Hout = H25, Qout = Q25>, 
K: DivInner<H25, Q25, TypedInteger<_0, _4, _0, _0, _0, _0, _0, _0>, Hout = H26, Qout = Q26>, 
K: DivInner<H26, Q26, TypedInteger<_0, _2, _0, _0, _0, _0, _0, _0>, Hout = H27, Qout = Q27>, 
K: DivInner<H27, Q27, TypedInteger<_0, _1, _0, _0, _0, _0, _0, _0>, Hout = H28, Qout = Q28>, 
K: DivInner<H28, Q28, TypedInteger<_8, _0, _0, _0, _0, _0, _0, _0>, Hout = H29, Qout = Q29>, 
K: DivInner<H29, Q29, TypedInteger<_4, _0, _0, _0, _0, _0, _0, _0>, Hout = H30, Qout = Q30>, 
K: DivInner<H30, Q30, TypedInteger<_2, _0, _0, _0, _0, _0, _0, _0>, Hout = H31, Qout = Q31>, 
K: DivInner<H31, Q31, TypedInteger<_1, _0, _0, _0, _0, _0, _0, _0>, Hout = H32, Qout = Q32>,
// There is a special case we need to implement: if k = 1 then stuff goes wrong
// If  k = 1 then h//k = h, h%k = 0
K: _Equal<TypedInteger<_1, _0, _0, _0, _0, _0, _0, _0>, Output = Eq1>,
Eq1: If<_Zero, H32, Output = H33>,
Eq1: If<TypedInteger<Hx0, Hx1, Hx2, Hx3, Hx4, Hx5, Hx6, Hx7>, Q32, Output = Q33>,
// There is a special case we need to implement: if H = 0 then stuff goes wrong. Order matters here because 0/1 == (0, 0)
TypedInteger<Hx0, Hx1, Hx2, Hx3, Hx4, Hx5, Hx6, Hx7>: _Equal<_Zero, Output = Eq0>,
Eq0: If<_Zero, H33, Output = HResult>,
Eq0: If<_Zero, Q33, Output = QResult>,
{ 
    type Output = QResult; type Remainder = HResult; 
}
