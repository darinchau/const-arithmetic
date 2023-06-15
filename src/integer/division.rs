use std::marker::PhantomData;
use super::*;

// =====================================
// = Inner implementation for division =
// =====================================

pub(crate) trait _Div<K: IsInteger> { type Output: IsInteger; type Remainder: IsInteger; }

// Conversion from binary to hex
trait HexMe<B1: Binary, B2: Binary, B3: Binary> { type Output: Hex; }
impl HexMe<_0, _0, _0> for _0 { type Output = _0; }
impl HexMe<_0, _0, _0> for _1 { type Output = _1; }
impl HexMe<_1, _0, _0> for _0 { type Output = _2; }
impl HexMe<_1, _0, _0> for _1 { type Output = _3; }
impl HexMe<_0, _1, _0> for _0 { type Output = _4; }
impl HexMe<_0, _1, _0> for _1 { type Output = _5; }
impl HexMe<_1, _1, _0> for _0 { type Output = _6; }
impl HexMe<_1, _1, _0> for _1 { type Output = _7; }
impl HexMe<_0, _0, _1> for _0 { type Output = _8; }
impl HexMe<_0, _0, _1> for _1 { type Output = _9; }
impl HexMe<_1, _0, _1> for _0 { type Output = _A; }
impl HexMe<_1, _0, _1> for _1 { type Output = _B; }
impl HexMe<_0, _1, _1> for _0 { type Output = _C; }
impl HexMe<_0, _1, _1> for _1 { type Output = _D; }
impl HexMe<_1, _1, _1> for _0 { type Output = _E; }
impl HexMe<_1, _1, _1> for _1 { type Output = _F; }

// We can have a comparison of two binary numbers
#[derive(Clone, Copy, PartialEq)]
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

impl<
B0: Binary, B1: Binary, B2: Binary, B3: Binary, 
B4: Binary, B5: Binary, B6: Binary, B7: Binary, 
B8: Binary, B9: Binary, B10: Binary, B11: Binary, 
B12: Binary, B13: Binary, B14: Binary, B15: Binary, 
B16: Binary, B17: Binary, B18: Binary, B19: Binary, 
B20: Binary, B21: Binary, B22: Binary, B23: Binary, 
B24: Binary, B25: Binary, B26: Binary, B27: Binary, 
B28: Binary, B29: Binary, B30: Binary, B31: Binary,
H0: Hex, H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> 
IsInteger for BinaryNumber<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28, B29, B30, B31> where
B0: HexMe<B1, B2, B3, Output = H0>,
B4: HexMe<B5, B6, B7, Output = H1>,
B8: HexMe<B9, B10, B11, Output = H2>,
B12: HexMe<B13, B14, B15, Output = H3>,
B16: HexMe<B16, B18, B19, Output = H4>,
B20: HexMe<B21, B22, B23, Output = H5>,
B24: HexMe<B25, B26, B27, Output = H6>,
B28: HexMe<B29, B30, B31, Output = H7>,
{
    type Hex0 = H0;
    type Hex1 = H1;
    type Hex2 = H2;
    type Hex3 = H3;
    type Hex4 = H4;
    type Hex5 = H5;
    type Hex6 = H6;
    type Hex7 = H7;
}


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
// where H is h, L is the overflow part of K after bitshifting, and K is the actual bitshifted part
// If L is ever greater than zero, then the if statement must be false
trait _InnerDiv<L: IsInteger, K: IsInteger> { type Hout: IsInteger; type Q: Binary; }
