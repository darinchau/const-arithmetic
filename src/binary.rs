//! Implements all binary types

use super::hex::{_0, _1};

/// Indicates binary types.
pub trait Binary {
    const VALUE: bool;
}
impl Binary for _0 { const VALUE: bool = false; }
impl Binary for _1 { const VALUE: bool = true; }

pub trait AssertTrue {}
impl AssertTrue for _1 {}

pub trait AssertFalse {}
impl AssertFalse for _0 {}

/// The NOT logic gate
pub trait BinNot { type Output: Binary; }
impl BinNot for _0 { type Output = _1; }
impl BinNot for _1 { type Output = _0; }

/// The AND logic gate
pub trait BinAnd<B: Binary> { type Output: Binary; }
impl BinAnd<_0> for _0 {type Output = _0;}
impl BinAnd<_0> for _1 {type Output = _0;}
impl BinAnd<_1> for _0 {type Output = _0;}
impl BinAnd<_1> for _1 {type Output = _1;}

/// The OR logic gate
pub trait BinOr<B: Binary> { type Output: Binary; }
impl BinOr<_0> for _0 {type Output = _0;}
impl BinOr<_0> for _1 {type Output = _1;}
impl BinOr<_1> for _0 {type Output = _1;}
impl BinOr<_1> for _1 {type Output = _1;}

/// The NOR logic gate
pub trait BinNor<B: Binary> { type Output: Binary; }
impl BinNor<_0> for _0 {type Output = _1;}
impl BinNor<_0> for _1 {type Output = _0;}
impl BinNor<_1> for _0 {type Output = _0;}
impl BinNor<_1> for _1 {type Output = _0;}


/// The NXOR logic gate, or EQUAL operator
pub trait BinEq<B: Binary> { type Output: Binary; }
impl BinEq<_0> for _0 {type Output = _1;}
impl BinEq<_0> for _1 {type Output = _0;}
impl BinEq<_1> for _0 {type Output = _0;}
impl BinEq<_1> for _1 {type Output = _1;}

/// A 8-input AND gate
pub(crate) trait BinMultiAnd<B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary, B7: Binary> { type Output: Binary; }
impl BinMultiAnd<_1, _1, _1, _1, _1, _1, _1> for _1 { type Output = _1; }
impl BinMultiAnd<_0, _1, _1, _1, _1, _1, _1> for _1 { type Output = _0; }
impl<B0: Binary> BinMultiAnd<B0, _0, _1, _1, _1, _1, _1> for _1 { type Output = _0; }
impl<B0: Binary, B1: Binary> BinMultiAnd<B0, B1, _0, _1, _1, _1, _1> for _1 { type Output = _0; }
impl<B0: Binary, B1: Binary, B2: Binary> BinMultiAnd<B0, B1, B2, _0, _1, _1, _1> for _1 { type Output = _0; }
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary> BinMultiAnd<B0, B1, B2, B3, _0, _1, _1> for _1 { type Output = _0; }
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary> BinMultiAnd<B0, B1, B2, B3, B4, _0, _1> for _1 { type Output = _0; }
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary> BinMultiAnd<B0, B1, B2, B3, B4, B5, _0> for _1 { type Output = _0; } 
impl<B0: Binary, B1: Binary, B2: Binary, B3: Binary, B4: Binary, B5: Binary, B6: Binary> BinMultiAnd<B0, B1, B2, B3, B4, B5, B6> for _0 { type Output = _0; }
