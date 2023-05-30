//! This implements the integer trait which denotes a 32 bit unsigned integer

use super::hex::*;
use std::marker::PhantomData;
use super::{HexAdd1, HexAdd2};

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
}

/// Denotes integer addition.
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
    H0: HexAdd1<N::Hex0, Output = R0, Carry = C0>,
    H1: HexAdd2<N::Hex1, C0, Output = R1, Carry = C1>,
    H2: HexAdd2<N::Hex2, C1, Output = R2, Carry = C2>,
    H3: HexAdd2<N::Hex3, C2, Output = R3, Carry = C3>,
    H4: HexAdd2<N::Hex4, C3, Output = R4, Carry = C4>,
    H5: HexAdd2<N::Hex5, C4, Output = R5, Carry = C5>,
    H6: HexAdd2<N::Hex6, C5, Output = R6, Carry = C6>,
    H7: HexAdd2<N::Hex7, C6, Output = R7, Carry = C7> 
{
    type Output = TypedInteger<R0, R1, R2, R3, R4, R5, R6, R7>;
}

/// Denotes integer subtraction.
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
    N::Hex0: HexNot<Output = X0>,
    N::Hex1: HexNot<Output = X1>,
    N::Hex2: HexNot<Output = X2>,
    N::Hex3: HexNot<Output = X3>,
    N::Hex4: HexNot<Output = X4>,
    N::Hex5: HexNot<Output = X5>,
    N::Hex6: HexNot<Output = X6>,
    N::Hex7: HexNot<Output = X7>,
    H0: HexAdd1<X0, Output = R0, Carry = C0>,
    H1: HexAdd2<X1, C0, Output = R1, Carry = C1>,
    H2: HexAdd2<X2, C1, Output = R2, Carry = C2>,
    H3: HexAdd2<X3, C2, Output = R3, Carry = C3>,
    H4: HexAdd2<X4, C3, Output = R4, Carry = C4>,
    H5: HexAdd2<X5, C4, Output = R5, Carry = C5>,
    H6: HexAdd2<X6, C5, Output = R6, Carry = C6>,
    H7: HexAdd2<X7, C6, Output = R7, Carry = C7> 
{
    type Output = TypedInteger<R0, R1, R2, R3, R4, R5, R6, R7>;
}
