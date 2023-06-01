//! Implements all binary types

use super::hex::{_0, _1, Hex};

/// Indicates binary types.
pub trait Binary {
    const VALUE: bool;
}
impl Binary for _0 { const VALUE: bool = false; }
impl Binary for _1 { const VALUE: bool = true; }

/// Asserts that a binary equals to _1
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// 
/// fn bin_assert_true<B>(_b: B) where
/// B: Binary,
/// B: AssertTrue
/// {}
/// 
/// bin_assert_true(_1);
/// ```
pub trait AssertTrue {}
impl AssertTrue for _1 {}

/// Asserts that a binary equals to _0
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_assert_false<B>(_b: B) where
/// B: Binary,
/// B: AssertFalse
/// {}
/// 
/// bin_assert_false(_0);
/// ```
pub trait AssertFalse {}
impl AssertFalse for _0 {}

/// The NOT logic gate
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_not<B, R>(_b: B, _r: R) where
/// B: Binary,
/// R: Binary,
/// B: BinNot<Output = R>
/// {}
/// 
/// bin_not(_0, True);
/// bin_not(_1, False);
/// ```
pub trait BinNot { type Output: Binary; }
impl BinNot for _0 { type Output = _1; }
impl BinNot for _1 { type Output = _0; }

/// The AND logic gate
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_and<B1, B2, R>(_h1: B1, _h2: B2, _r: R) where
/// B1: Binary,
/// B2: Binary,
/// R: Binary,
/// B1: BinAnd<B2, Output = R>
/// {}
/// 
/// bin_and(_0, _0, False);
/// bin_and(_1, _0, False);
/// bin_and(_0, _1, False);
/// bin_and(_1, _1, True);
/// ```
pub trait BinAnd<B: Binary> { type Output: Binary; }
impl BinAnd<_0> for _0 {type Output = _0;}
impl BinAnd<_0> for _1 {type Output = _0;}
impl BinAnd<_1> for _0 {type Output = _0;}
impl BinAnd<_1> for _1 {type Output = _1;}

/// The OR logic gate
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_or<B1, B2, R>(_h1: B1, _h2: B2, _r: R) where
/// B1: Binary,
/// B2: Binary,
/// R: Binary,
/// B1: BinOr<B2, Output = R>
/// {}
/// 
/// bin_or(_0, _0, False);
/// bin_or(_1, _0, True);
/// bin_or(_0, _1, True);
/// bin_or(_1, _1, True);
/// ```
pub trait BinOr<B: Binary> { type Output: Binary; }
impl BinOr<_0> for _0 {type Output = _0;}
impl BinOr<_0> for _1 {type Output = _1;}
impl BinOr<_1> for _0 {type Output = _1;}
impl BinOr<_1> for _1 {type Output = _1;}

/// The NOR logic gate
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_nor<B1, B2, R>(_h1: B1, _h2: B2, _r: R) where
/// B1: Binary,
/// B2: Binary,
/// R: Binary,
/// B1: BinNor<B2, Output = R>
/// {}
/// 
/// bin_nor(_0, _0, True);
/// bin_nor(_1, _0, False);
/// bin_nor(_0, _1, False);
/// bin_nor(_1, _1, False);
/// ```
pub trait BinNor<B: Binary> { type Output: Binary; }
impl BinNor<_0> for _0 {type Output = _1;}
impl BinNor<_0> for _1 {type Output = _0;}
impl BinNor<_1> for _0 {type Output = _0;}
impl BinNor<_1> for _1 {type Output = _0;}


/// The NXOR logic gate, or EQUAL operator
/// 
/// Example
/// ```
/// use const_arithmetic::*;
/// let True = _1;
/// let False = _0;
/// 
/// fn bin_nxor<B1, B2, R>(_h1: B1, _h2: B2, _r: R) where
/// B1: Binary,
/// B2: Binary,
/// R: Binary,
/// B1: BinEq<B2, Output = R>
/// {}
/// 
/// bin_nxor(_0, _0, True);
/// bin_nxor(_1, _0, False);
/// bin_nxor(_0, _1, False);
/// bin_nxor(_1, _1, True);
/// ```
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
