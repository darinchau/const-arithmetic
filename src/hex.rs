//! Implements the base hexadecimal type

/// This denotes one single hexadecimal - half a byte
pub trait Hex {
    const NUMBER: u32;
}

/// This denotes the number 0
pub struct _0;
impl Hex for _0 { const NUMBER: u32 = 0; }

/// This denotes the number 1
pub struct _1;
impl Hex for _1 { const NUMBER: u32 = 1; }

/// This denotes the number 2
pub struct _2;
impl Hex for _2 { const NUMBER: u32 = 2; }

/// This denotes the number 3
pub struct _3;
impl Hex for _3 { const NUMBER: u32 = 3; }

/// This denotes the number 4
pub struct _4;
impl Hex for _4 { const NUMBER: u32 = 4; }

/// This denotes the number 5
pub struct _5;
impl Hex for _5 { const NUMBER: u32 = 5; }

/// This denotes the number 6
pub struct _6;
impl Hex for _6 { const NUMBER: u32 = 6; }

/// This denotes the number 7
pub struct _7;
impl Hex for _7 { const NUMBER: u32 = 7; }

/// This denotes the number 8
pub struct _8;
impl Hex for _8 { const NUMBER: u32 = 8; }

/// This denotes the number 9
pub struct _9;
impl Hex for _9 { const NUMBER: u32 = 9; }

/// This denotes the number A
pub struct _A;
impl Hex for _A { const NUMBER: u32 = 10; }

/// This denotes the number B
pub struct _B;
impl Hex for _B { const NUMBER: u32 = 11; }

/// This denotes the number C
pub struct _C;
impl Hex for _C { const NUMBER: u32 = 12; }

/// This denotes the number D
pub struct _D;
impl Hex for _D { const NUMBER: u32 = 13; }

/// This denotes the number E
pub struct _E;
impl Hex for _E { const NUMBER: u32 = 14; }

/// This denotes the number F
pub struct _F;
impl Hex for _F { const NUMBER: u32 = 15; }

/// This is an internal implementation of addition of one other number
pub trait HexAdd<H: Hex> { type Output: Hex; type Carry: Hex; }
impl HexAdd<_0> for _0 { type Output = _0; type Carry = _0; }
impl HexAdd<_0> for _1 { type Output = _1; type Carry = _0; }
impl HexAdd<_0> for _2 { type Output = _2; type Carry = _0; }
impl HexAdd<_0> for _3 { type Output = _3; type Carry = _0; }
impl HexAdd<_0> for _4 { type Output = _4; type Carry = _0; }
impl HexAdd<_0> for _5 { type Output = _5; type Carry = _0; }
impl HexAdd<_0> for _6 { type Output = _6; type Carry = _0; }
impl HexAdd<_0> for _7 { type Output = _7; type Carry = _0; }
impl HexAdd<_0> for _8 { type Output = _8; type Carry = _0; }
impl HexAdd<_0> for _9 { type Output = _9; type Carry = _0; }
impl HexAdd<_0> for _A { type Output = _A; type Carry = _0; }
impl HexAdd<_0> for _B { type Output = _B; type Carry = _0; }
impl HexAdd<_0> for _C { type Output = _C; type Carry = _0; }
impl HexAdd<_0> for _D { type Output = _D; type Carry = _0; }
impl HexAdd<_0> for _E { type Output = _E; type Carry = _0; }
impl HexAdd<_0> for _F { type Output = _F; type Carry = _0; }
impl HexAdd<_1> for _0 { type Output = _1; type Carry = _0; }
impl HexAdd<_1> for _1 { type Output = _2; type Carry = _0; }
impl HexAdd<_1> for _2 { type Output = _3; type Carry = _0; }
impl HexAdd<_1> for _3 { type Output = _4; type Carry = _0; }
impl HexAdd<_1> for _4 { type Output = _5; type Carry = _0; }
impl HexAdd<_1> for _5 { type Output = _6; type Carry = _0; }
impl HexAdd<_1> for _6 { type Output = _7; type Carry = _0; }
impl HexAdd<_1> for _7 { type Output = _8; type Carry = _0; }
impl HexAdd<_1> for _8 { type Output = _9; type Carry = _0; }
impl HexAdd<_1> for _9 { type Output = _A; type Carry = _0; }
impl HexAdd<_1> for _A { type Output = _B; type Carry = _0; }
impl HexAdd<_1> for _B { type Output = _C; type Carry = _0; }
impl HexAdd<_1> for _C { type Output = _D; type Carry = _0; }
impl HexAdd<_1> for _D { type Output = _E; type Carry = _0; }
impl HexAdd<_1> for _E { type Output = _F; type Carry = _0; }
impl HexAdd<_1> for _F { type Output = _0; type Carry = _1; }
impl HexAdd<_2> for _0 { type Output = _2; type Carry = _0; }
impl HexAdd<_2> for _1 { type Output = _3; type Carry = _0; }
impl HexAdd<_2> for _2 { type Output = _4; type Carry = _0; }
impl HexAdd<_2> for _3 { type Output = _5; type Carry = _0; }
impl HexAdd<_2> for _4 { type Output = _6; type Carry = _0; }
impl HexAdd<_2> for _5 { type Output = _7; type Carry = _0; }
impl HexAdd<_2> for _6 { type Output = _8; type Carry = _0; }
impl HexAdd<_2> for _7 { type Output = _9; type Carry = _0; }
impl HexAdd<_2> for _8 { type Output = _A; type Carry = _0; }
impl HexAdd<_2> for _9 { type Output = _B; type Carry = _0; }
impl HexAdd<_2> for _A { type Output = _C; type Carry = _0; }
impl HexAdd<_2> for _B { type Output = _D; type Carry = _0; }
impl HexAdd<_2> for _C { type Output = _E; type Carry = _0; }
impl HexAdd<_2> for _D { type Output = _F; type Carry = _0; }
impl HexAdd<_2> for _E { type Output = _0; type Carry = _1; }
impl HexAdd<_2> for _F { type Output = _1; type Carry = _1; }
impl HexAdd<_3> for _0 { type Output = _3; type Carry = _0; }
impl HexAdd<_3> for _1 { type Output = _4; type Carry = _0; }
impl HexAdd<_3> for _2 { type Output = _5; type Carry = _0; }
impl HexAdd<_3> for _3 { type Output = _6; type Carry = _0; }
impl HexAdd<_3> for _4 { type Output = _7; type Carry = _0; }
impl HexAdd<_3> for _5 { type Output = _8; type Carry = _0; }
impl HexAdd<_3> for _6 { type Output = _9; type Carry = _0; }
impl HexAdd<_3> for _7 { type Output = _A; type Carry = _0; }
impl HexAdd<_3> for _8 { type Output = _B; type Carry = _0; }
impl HexAdd<_3> for _9 { type Output = _C; type Carry = _0; }
impl HexAdd<_3> for _A { type Output = _D; type Carry = _0; }
impl HexAdd<_3> for _B { type Output = _E; type Carry = _0; }
impl HexAdd<_3> for _C { type Output = _F; type Carry = _0; }
impl HexAdd<_3> for _D { type Output = _0; type Carry = _1; }
impl HexAdd<_3> for _E { type Output = _1; type Carry = _1; }
impl HexAdd<_3> for _F { type Output = _2; type Carry = _1; }
impl HexAdd<_4> for _0 { type Output = _4; type Carry = _0; }
impl HexAdd<_4> for _1 { type Output = _5; type Carry = _0; }
impl HexAdd<_4> for _2 { type Output = _6; type Carry = _0; }
impl HexAdd<_4> for _3 { type Output = _7; type Carry = _0; }
impl HexAdd<_4> for _4 { type Output = _8; type Carry = _0; }
impl HexAdd<_4> for _5 { type Output = _9; type Carry = _0; }
impl HexAdd<_4> for _6 { type Output = _A; type Carry = _0; }
impl HexAdd<_4> for _7 { type Output = _B; type Carry = _0; }
impl HexAdd<_4> for _8 { type Output = _C; type Carry = _0; }
impl HexAdd<_4> for _9 { type Output = _D; type Carry = _0; }
impl HexAdd<_4> for _A { type Output = _E; type Carry = _0; }
impl HexAdd<_4> for _B { type Output = _F; type Carry = _0; }
impl HexAdd<_4> for _C { type Output = _0; type Carry = _1; }
impl HexAdd<_4> for _D { type Output = _1; type Carry = _1; }
impl HexAdd<_4> for _E { type Output = _2; type Carry = _1; }
impl HexAdd<_4> for _F { type Output = _3; type Carry = _1; }
impl HexAdd<_5> for _0 { type Output = _5; type Carry = _0; }
impl HexAdd<_5> for _1 { type Output = _6; type Carry = _0; }
impl HexAdd<_5> for _2 { type Output = _7; type Carry = _0; }
impl HexAdd<_5> for _3 { type Output = _8; type Carry = _0; }
impl HexAdd<_5> for _4 { type Output = _9; type Carry = _0; }
impl HexAdd<_5> for _5 { type Output = _A; type Carry = _0; }
impl HexAdd<_5> for _6 { type Output = _B; type Carry = _0; }
impl HexAdd<_5> for _7 { type Output = _C; type Carry = _0; }
impl HexAdd<_5> for _8 { type Output = _D; type Carry = _0; }
impl HexAdd<_5> for _9 { type Output = _E; type Carry = _0; }
impl HexAdd<_5> for _A { type Output = _F; type Carry = _0; }
impl HexAdd<_5> for _B { type Output = _0; type Carry = _1; }
impl HexAdd<_5> for _C { type Output = _1; type Carry = _1; }
impl HexAdd<_5> for _D { type Output = _2; type Carry = _1; }
impl HexAdd<_5> for _E { type Output = _3; type Carry = _1; }
impl HexAdd<_5> for _F { type Output = _4; type Carry = _1; }
impl HexAdd<_6> for _0 { type Output = _6; type Carry = _0; }
impl HexAdd<_6> for _1 { type Output = _7; type Carry = _0; }
impl HexAdd<_6> for _2 { type Output = _8; type Carry = _0; }
impl HexAdd<_6> for _3 { type Output = _9; type Carry = _0; }
impl HexAdd<_6> for _4 { type Output = _A; type Carry = _0; }
impl HexAdd<_6> for _5 { type Output = _B; type Carry = _0; }
impl HexAdd<_6> for _6 { type Output = _C; type Carry = _0; }
impl HexAdd<_6> for _7 { type Output = _D; type Carry = _0; }
impl HexAdd<_6> for _8 { type Output = _E; type Carry = _0; }
impl HexAdd<_6> for _9 { type Output = _F; type Carry = _0; }
impl HexAdd<_6> for _A { type Output = _0; type Carry = _1; }
impl HexAdd<_6> for _B { type Output = _1; type Carry = _1; }
impl HexAdd<_6> for _C { type Output = _2; type Carry = _1; }
impl HexAdd<_6> for _D { type Output = _3; type Carry = _1; }
impl HexAdd<_6> for _E { type Output = _4; type Carry = _1; }
impl HexAdd<_6> for _F { type Output = _5; type Carry = _1; }
impl HexAdd<_7> for _0 { type Output = _7; type Carry = _0; }
impl HexAdd<_7> for _1 { type Output = _8; type Carry = _0; }
impl HexAdd<_7> for _2 { type Output = _9; type Carry = _0; }
impl HexAdd<_7> for _3 { type Output = _A; type Carry = _0; }
impl HexAdd<_7> for _4 { type Output = _B; type Carry = _0; }
impl HexAdd<_7> for _5 { type Output = _C; type Carry = _0; }
impl HexAdd<_7> for _6 { type Output = _D; type Carry = _0; }
impl HexAdd<_7> for _7 { type Output = _E; type Carry = _0; }
impl HexAdd<_7> for _8 { type Output = _F; type Carry = _0; }
impl HexAdd<_7> for _9 { type Output = _0; type Carry = _1; }
impl HexAdd<_7> for _A { type Output = _1; type Carry = _1; }
impl HexAdd<_7> for _B { type Output = _2; type Carry = _1; }
impl HexAdd<_7> for _C { type Output = _3; type Carry = _1; }
impl HexAdd<_7> for _D { type Output = _4; type Carry = _1; }
impl HexAdd<_7> for _E { type Output = _5; type Carry = _1; }
impl HexAdd<_7> for _F { type Output = _6; type Carry = _1; }
impl HexAdd<_8> for _0 { type Output = _8; type Carry = _0; }
impl HexAdd<_8> for _1 { type Output = _9; type Carry = _0; }
impl HexAdd<_8> for _2 { type Output = _A; type Carry = _0; }
impl HexAdd<_8> for _3 { type Output = _B; type Carry = _0; }
impl HexAdd<_8> for _4 { type Output = _C; type Carry = _0; }
impl HexAdd<_8> for _5 { type Output = _D; type Carry = _0; }
impl HexAdd<_8> for _6 { type Output = _E; type Carry = _0; }
impl HexAdd<_8> for _7 { type Output = _F; type Carry = _0; }
impl HexAdd<_8> for _8 { type Output = _0; type Carry = _1; }
impl HexAdd<_8> for _9 { type Output = _1; type Carry = _1; }
impl HexAdd<_8> for _A { type Output = _2; type Carry = _1; }
impl HexAdd<_8> for _B { type Output = _3; type Carry = _1; }
impl HexAdd<_8> for _C { type Output = _4; type Carry = _1; }
impl HexAdd<_8> for _D { type Output = _5; type Carry = _1; }
impl HexAdd<_8> for _E { type Output = _6; type Carry = _1; }
impl HexAdd<_8> for _F { type Output = _7; type Carry = _1; }
impl HexAdd<_9> for _0 { type Output = _9; type Carry = _0; }
impl HexAdd<_9> for _1 { type Output = _A; type Carry = _0; }
impl HexAdd<_9> for _2 { type Output = _B; type Carry = _0; }
impl HexAdd<_9> for _3 { type Output = _C; type Carry = _0; }
impl HexAdd<_9> for _4 { type Output = _D; type Carry = _0; }
impl HexAdd<_9> for _5 { type Output = _E; type Carry = _0; }
impl HexAdd<_9> for _6 { type Output = _F; type Carry = _0; }
impl HexAdd<_9> for _7 { type Output = _0; type Carry = _1; }
impl HexAdd<_9> for _8 { type Output = _1; type Carry = _1; }
impl HexAdd<_9> for _9 { type Output = _2; type Carry = _1; }
impl HexAdd<_9> for _A { type Output = _3; type Carry = _1; }
impl HexAdd<_9> for _B { type Output = _4; type Carry = _1; }
impl HexAdd<_9> for _C { type Output = _5; type Carry = _1; }
impl HexAdd<_9> for _D { type Output = _6; type Carry = _1; }
impl HexAdd<_9> for _E { type Output = _7; type Carry = _1; }
impl HexAdd<_9> for _F { type Output = _8; type Carry = _1; }
impl HexAdd<_A> for _0 { type Output = _A; type Carry = _0; }
impl HexAdd<_A> for _1 { type Output = _B; type Carry = _0; }
impl HexAdd<_A> for _2 { type Output = _C; type Carry = _0; }
impl HexAdd<_A> for _3 { type Output = _D; type Carry = _0; }
impl HexAdd<_A> for _4 { type Output = _E; type Carry = _0; }
impl HexAdd<_A> for _5 { type Output = _F; type Carry = _0; }
impl HexAdd<_A> for _6 { type Output = _0; type Carry = _1; }
impl HexAdd<_A> for _7 { type Output = _1; type Carry = _1; }
impl HexAdd<_A> for _8 { type Output = _2; type Carry = _1; }
impl HexAdd<_A> for _9 { type Output = _3; type Carry = _1; }
impl HexAdd<_A> for _A { type Output = _4; type Carry = _1; }
impl HexAdd<_A> for _B { type Output = _5; type Carry = _1; }
impl HexAdd<_A> for _C { type Output = _6; type Carry = _1; }
impl HexAdd<_A> for _D { type Output = _7; type Carry = _1; }
impl HexAdd<_A> for _E { type Output = _8; type Carry = _1; }
impl HexAdd<_A> for _F { type Output = _9; type Carry = _1; }
impl HexAdd<_B> for _0 { type Output = _B; type Carry = _0; }
impl HexAdd<_B> for _1 { type Output = _C; type Carry = _0; }
impl HexAdd<_B> for _2 { type Output = _D; type Carry = _0; }
impl HexAdd<_B> for _3 { type Output = _E; type Carry = _0; }
impl HexAdd<_B> for _4 { type Output = _F; type Carry = _0; }
impl HexAdd<_B> for _5 { type Output = _0; type Carry = _1; }
impl HexAdd<_B> for _6 { type Output = _1; type Carry = _1; }
impl HexAdd<_B> for _7 { type Output = _2; type Carry = _1; }
impl HexAdd<_B> for _8 { type Output = _3; type Carry = _1; }
impl HexAdd<_B> for _9 { type Output = _4; type Carry = _1; }
impl HexAdd<_B> for _A { type Output = _5; type Carry = _1; }
impl HexAdd<_B> for _B { type Output = _6; type Carry = _1; }
impl HexAdd<_B> for _C { type Output = _7; type Carry = _1; }
impl HexAdd<_B> for _D { type Output = _8; type Carry = _1; }
impl HexAdd<_B> for _E { type Output = _9; type Carry = _1; }
impl HexAdd<_B> for _F { type Output = _A; type Carry = _1; }
impl HexAdd<_C> for _0 { type Output = _C; type Carry = _0; }
impl HexAdd<_C> for _1 { type Output = _D; type Carry = _0; }
impl HexAdd<_C> for _2 { type Output = _E; type Carry = _0; }
impl HexAdd<_C> for _3 { type Output = _F; type Carry = _0; }
impl HexAdd<_C> for _4 { type Output = _0; type Carry = _1; }
impl HexAdd<_C> for _5 { type Output = _1; type Carry = _1; }
impl HexAdd<_C> for _6 { type Output = _2; type Carry = _1; }
impl HexAdd<_C> for _7 { type Output = _3; type Carry = _1; }
impl HexAdd<_C> for _8 { type Output = _4; type Carry = _1; }
impl HexAdd<_C> for _9 { type Output = _5; type Carry = _1; }
impl HexAdd<_C> for _A { type Output = _6; type Carry = _1; }
impl HexAdd<_C> for _B { type Output = _7; type Carry = _1; }
impl HexAdd<_C> for _C { type Output = _8; type Carry = _1; }
impl HexAdd<_C> for _D { type Output = _9; type Carry = _1; }
impl HexAdd<_C> for _E { type Output = _A; type Carry = _1; }
impl HexAdd<_C> for _F { type Output = _B; type Carry = _1; }
impl HexAdd<_D> for _0 { type Output = _D; type Carry = _0; }
impl HexAdd<_D> for _1 { type Output = _E; type Carry = _0; }
impl HexAdd<_D> for _2 { type Output = _F; type Carry = _0; }
impl HexAdd<_D> for _3 { type Output = _0; type Carry = _1; }
impl HexAdd<_D> for _4 { type Output = _1; type Carry = _1; }
impl HexAdd<_D> for _5 { type Output = _2; type Carry = _1; }
impl HexAdd<_D> for _6 { type Output = _3; type Carry = _1; }
impl HexAdd<_D> for _7 { type Output = _4; type Carry = _1; }
impl HexAdd<_D> for _8 { type Output = _5; type Carry = _1; }
impl HexAdd<_D> for _9 { type Output = _6; type Carry = _1; }
impl HexAdd<_D> for _A { type Output = _7; type Carry = _1; }
impl HexAdd<_D> for _B { type Output = _8; type Carry = _1; }
impl HexAdd<_D> for _C { type Output = _9; type Carry = _1; }
impl HexAdd<_D> for _D { type Output = _A; type Carry = _1; }
impl HexAdd<_D> for _E { type Output = _B; type Carry = _1; }
impl HexAdd<_D> for _F { type Output = _C; type Carry = _1; }
impl HexAdd<_E> for _0 { type Output = _E; type Carry = _0; }
impl HexAdd<_E> for _1 { type Output = _F; type Carry = _0; }
impl HexAdd<_E> for _2 { type Output = _0; type Carry = _1; }
impl HexAdd<_E> for _3 { type Output = _1; type Carry = _1; }
impl HexAdd<_E> for _4 { type Output = _2; type Carry = _1; }
impl HexAdd<_E> for _5 { type Output = _3; type Carry = _1; }
impl HexAdd<_E> for _6 { type Output = _4; type Carry = _1; }
impl HexAdd<_E> for _7 { type Output = _5; type Carry = _1; }
impl HexAdd<_E> for _8 { type Output = _6; type Carry = _1; }
impl HexAdd<_E> for _9 { type Output = _7; type Carry = _1; }
impl HexAdd<_E> for _A { type Output = _8; type Carry = _1; }
impl HexAdd<_E> for _B { type Output = _9; type Carry = _1; }
impl HexAdd<_E> for _C { type Output = _A; type Carry = _1; }
impl HexAdd<_E> for _D { type Output = _B; type Carry = _1; }
impl HexAdd<_E> for _E { type Output = _C; type Carry = _1; }
impl HexAdd<_E> for _F { type Output = _D; type Carry = _1; }
impl HexAdd<_F> for _0 { type Output = _F; type Carry = _0; }
impl HexAdd<_F> for _1 { type Output = _0; type Carry = _1; }
impl HexAdd<_F> for _2 { type Output = _1; type Carry = _1; }
impl HexAdd<_F> for _3 { type Output = _2; type Carry = _1; }
impl HexAdd<_F> for _4 { type Output = _3; type Carry = _1; }
impl HexAdd<_F> for _5 { type Output = _4; type Carry = _1; }
impl HexAdd<_F> for _6 { type Output = _5; type Carry = _1; }
impl HexAdd<_F> for _7 { type Output = _6; type Carry = _1; }
impl HexAdd<_F> for _8 { type Output = _7; type Carry = _1; }
impl HexAdd<_F> for _9 { type Output = _8; type Carry = _1; }
impl HexAdd<_F> for _A { type Output = _9; type Carry = _1; }
impl HexAdd<_F> for _B { type Output = _A; type Carry = _1; }
impl HexAdd<_F> for _C { type Output = _B; type Carry = _1; }
impl HexAdd<_F> for _D { type Output = _C; type Carry = _1; }
impl HexAdd<_F> for _E { type Output = _D; type Carry = _1; }
impl HexAdd<_F> for _F { type Output = _E; type Carry = _1; }

/// This is an internal implementation of addition of three number
pub trait HexAdd3<H1: Hex, H2: Hex> { type Output: Hex; type Carry: Hex; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _0 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _1 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _2 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _3 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _4 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _5 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _6 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _7 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _8 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _9 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _A where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _B where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _C where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _D where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _E where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_0, H> for _F where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _0 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _1 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _2 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _3 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _4 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _5 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _6 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _7 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _8 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _9 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _A where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _B where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _C where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _D where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _E where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_1, H> for _F where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _0 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _1 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _2 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _3 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _4 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _5 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _6 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _7 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _8 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _9 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _A where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _B where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _C where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _D where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _E where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_2, H> for _F where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _0 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _1 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _2 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _3 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _4 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _5 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _6 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _7 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _8 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _9 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _A where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _B where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _C where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _D where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _E where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_3, H> for _F where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _0 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _1 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _2 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _3 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _4 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _5 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _6 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _7 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _8 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _9 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _A where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _B where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _C where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _D where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _E where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_4, H> for _F where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _0 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _1 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _2 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _3 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _4 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _5 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _6 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _7 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _8 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _9 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _A where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _B where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _C where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _D where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _E where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_5, H> for _F where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _0 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _1 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _2 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _3 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _4 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _5 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _6 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _7 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _8 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _9 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _A where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _B where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _C where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _D where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _E where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_6, H> for _F where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _0 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _1 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _2 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _3 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _4 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _5 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _6 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _7 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _8 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _9 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _A where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _B where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _C where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _D where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _E where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_7, H> for _F where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _0 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _1 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _2 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _3 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _4 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _5 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _6 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _7 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _8 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _9 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _A where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _B where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _C where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _D where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _E where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_8, H> for _F where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _0 where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _1 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _2 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _3 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _4 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _5 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _6 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _7 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _8 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _9 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _A where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _B where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _C where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _D where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _E where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_9, H> for _F where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _0 where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _1 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _2 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _3 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _4 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _5 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _6 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _7 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _8 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _9 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _A where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _B where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _C where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _D where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _E where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_A, H> for _F where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _0 where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _1 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _2 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _3 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _4 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _5 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _6 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _7 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _8 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _9 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _A where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _B where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _C where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _D where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _E where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_B, H> for _F where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _0 where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _1 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _2 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _3 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _4 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _5 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _6 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _7 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _8 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _9 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _A where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _B where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _C where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _D where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _E where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_C, H> for _F where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _0 where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _1 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _2 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _3 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _4 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _5 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _6 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _7 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _8 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _9 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _A where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _B where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _C where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _D where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _E where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_D, H> for _F where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _0 where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _1 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _2 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _3 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _4 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _5 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _6 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _7 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _8 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _9 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _A where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _B where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _C where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _D where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _E where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_E, H> for _F where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _0 where H: HexAdd<_F, Output = R, Carry = C0>, C0: HexAdd<_0, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _1 where H: HexAdd<_0, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _2 where H: HexAdd<_1, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _3 where H: HexAdd<_2, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _4 where H: HexAdd<_3, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _5 where H: HexAdd<_4, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _6 where H: HexAdd<_5, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _7 where H: HexAdd<_6, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _8 where H: HexAdd<_7, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _9 where H: HexAdd<_8, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _A where H: HexAdd<_9, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _B where H: HexAdd<_A, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _C where H: HexAdd<_B, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _D where H: HexAdd<_C, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _E where H: HexAdd<_D, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }
impl<H: Hex, R: Hex, C0: Hex, Cr: Hex, C_: Hex> HexAdd3<_F, H> for _F where H: HexAdd<_E, Output = R, Carry = C0>, C0: HexAdd<_1, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = R; type Carry = Cr; }

/// This is an internal implementation of addition of 4 number
pub trait HexAdd4<H1: Hex, H2: Hex, H3: Hex> { type Output: Hex; type Carry: Hex; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _0 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_0, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _1 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_1, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _2 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_2, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _3 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_3, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _4 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _5 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _6 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _7 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _8 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_8, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _9 where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_9, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _A where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_A, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _B where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_B, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _C where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_C, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _D where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_D, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _E where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_E, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, R1, C1, C2, Result, Cr, C_> HexAdd4<H1, H2, H3> for _F where H1: Hex, H2: Hex, H3: Hex, R1: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R1, Carry = C1>, R1: HexAdd<_F, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }

/// This is an internal implementation of addition of 5 number
pub trait HexAdd5<H1: Hex, H2: Hex, H3: Hex, H4: Hex> { type Output: Hex; type Carry: Hex; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _0 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_0, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _1 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_1, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _2 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_2, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _3 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_3, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _4 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_4, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _5 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_5, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _6 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_6, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _7 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_7, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _8 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_8, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _9 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_9, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _A where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_A, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _B where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_B, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _C where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_C, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _D where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_D, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _E where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_E, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, R, C1, C2, Result, Cr, C_> HexAdd5<H1, H2, H3, H4> for _F where H1: Hex, H2: Hex, H3: Hex, H4: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd3<H2, H3, Output = R, Carry = C1>, R: HexAdd3<_F, H4, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }

/// This is an internal implementation of addition of 6 number
pub trait HexAdd6<H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex> { type Output: Hex; type Carry: Hex; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _0 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_0, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _1 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_1, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _2 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_2, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _3 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_3, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _4 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_4, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _5 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_5, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _6 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_6, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _7 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_7, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _8 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_8, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _9 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_9, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _A where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_A, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _B where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_B, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _C where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_C, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _D where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_D, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _E where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_E, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, R, C1, C2, Result, Cr, C_> HexAdd6<H1, H2, H3, H4, H5> for _F where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd4<H2, H3, H4, Output = R, Carry = C1>, R: HexAdd3<_F, H5, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }

/// This is an internal implementation of addition of 7 number
pub trait HexAdd7<H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex> { type Output: Hex; type Carry: Hex; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _0 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_0, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _1 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_1, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _2 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_2, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _3 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_3, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _4 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_4, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _5 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_5, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _6 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_6, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _7 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_7, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _8 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_8, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _9 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_9, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _A where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_A, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _B where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_B, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _C where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_C, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _D where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_D, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _E where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_E, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, R, C1, C2, Result, Cr, C_> HexAdd7<H1, H2, H3, H4, H5, H6> for _F where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd5<H2, H3, H4, H5, Output = R, Carry = C1>, R: HexAdd3<_F, H6, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }

/// This is an internal implementation of addition of 8 number
pub trait HexAdd8<H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex> { type Output: Hex; type Carry: Hex; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _0 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_0, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _1 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_1, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _2 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_2, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _3 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_3, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _4 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_4, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _5 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_5, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _6 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_6, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _7 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_7, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _8 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_8, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _9 where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_9, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _A where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_A, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _B where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_B, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _C where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_C, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _D where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_D, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _E where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_E, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }
impl<H1, H2, H3, H4, H5, H6, H7, R, C1, C2, Result, Cr, C_> HexAdd8<H1, H2, H3, H4, H5, H6, H7> for _F where H1: Hex, H2: Hex, H3: Hex, H4: Hex, H5: Hex, H6: Hex, H7: Hex, R: Hex, Result: Hex, C1: Hex, C2: Hex, Cr: Hex, C_: Hex, H1: HexAdd6<H2, H3, H4, H5, H6, Output = R, Carry = C1>, R: HexAdd3<_F, H7, Output = Result, Carry = C2>, C1: HexAdd<C2, Output = Cr, Carry = C_>, C_: HexAssertEqual<_0> { type Output = Result; type Carry = Cr; }

/// This trait is the equal trait on hex numbers. False evaluates to _0 and true evaluates to _1
pub trait HexEqual<H: Hex> { type Output: Hex; }
impl HexEqual<_0> for _0 { type Output = _1; }
impl HexEqual<_0> for _1 { type Output = _0; }
impl HexEqual<_0> for _2 { type Output = _0; }
impl HexEqual<_0> for _3 { type Output = _0; }
impl HexEqual<_0> for _4 { type Output = _0; }
impl HexEqual<_0> for _5 { type Output = _0; }
impl HexEqual<_0> for _6 { type Output = _0; }
impl HexEqual<_0> for _7 { type Output = _0; }
impl HexEqual<_0> for _8 { type Output = _0; }
impl HexEqual<_0> for _9 { type Output = _0; }
impl HexEqual<_0> for _A { type Output = _0; }
impl HexEqual<_0> for _B { type Output = _0; }
impl HexEqual<_0> for _C { type Output = _0; }
impl HexEqual<_0> for _D { type Output = _0; }
impl HexEqual<_0> for _E { type Output = _0; }
impl HexEqual<_0> for _F { type Output = _0; }
impl HexEqual<_1> for _0 { type Output = _0; }
impl HexEqual<_1> for _1 { type Output = _1; }
impl HexEqual<_1> for _2 { type Output = _0; }
impl HexEqual<_1> for _3 { type Output = _0; }
impl HexEqual<_1> for _4 { type Output = _0; }
impl HexEqual<_1> for _5 { type Output = _0; }
impl HexEqual<_1> for _6 { type Output = _0; }
impl HexEqual<_1> for _7 { type Output = _0; }
impl HexEqual<_1> for _8 { type Output = _0; }
impl HexEqual<_1> for _9 { type Output = _0; }
impl HexEqual<_1> for _A { type Output = _0; }
impl HexEqual<_1> for _B { type Output = _0; }
impl HexEqual<_1> for _C { type Output = _0; }
impl HexEqual<_1> for _D { type Output = _0; }
impl HexEqual<_1> for _E { type Output = _0; }
impl HexEqual<_1> for _F { type Output = _0; }
impl HexEqual<_2> for _0 { type Output = _0; }
impl HexEqual<_2> for _1 { type Output = _0; }
impl HexEqual<_2> for _2 { type Output = _1; }
impl HexEqual<_2> for _3 { type Output = _0; }
impl HexEqual<_2> for _4 { type Output = _0; }
impl HexEqual<_2> for _5 { type Output = _0; }
impl HexEqual<_2> for _6 { type Output = _0; }
impl HexEqual<_2> for _7 { type Output = _0; }
impl HexEqual<_2> for _8 { type Output = _0; }
impl HexEqual<_2> for _9 { type Output = _0; }
impl HexEqual<_2> for _A { type Output = _0; }
impl HexEqual<_2> for _B { type Output = _0; }
impl HexEqual<_2> for _C { type Output = _0; }
impl HexEqual<_2> for _D { type Output = _0; }
impl HexEqual<_2> for _E { type Output = _0; }
impl HexEqual<_2> for _F { type Output = _0; }
impl HexEqual<_3> for _0 { type Output = _0; }
impl HexEqual<_3> for _1 { type Output = _0; }
impl HexEqual<_3> for _2 { type Output = _0; }
impl HexEqual<_3> for _3 { type Output = _1; }
impl HexEqual<_3> for _4 { type Output = _0; }
impl HexEqual<_3> for _5 { type Output = _0; }
impl HexEqual<_3> for _6 { type Output = _0; }
impl HexEqual<_3> for _7 { type Output = _0; }
impl HexEqual<_3> for _8 { type Output = _0; }
impl HexEqual<_3> for _9 { type Output = _0; }
impl HexEqual<_3> for _A { type Output = _0; }
impl HexEqual<_3> for _B { type Output = _0; }
impl HexEqual<_3> for _C { type Output = _0; }
impl HexEqual<_3> for _D { type Output = _0; }
impl HexEqual<_3> for _E { type Output = _0; }
impl HexEqual<_3> for _F { type Output = _0; }
impl HexEqual<_4> for _0 { type Output = _0; }
impl HexEqual<_4> for _1 { type Output = _0; }
impl HexEqual<_4> for _2 { type Output = _0; }
impl HexEqual<_4> for _3 { type Output = _0; }
impl HexEqual<_4> for _4 { type Output = _1; }
impl HexEqual<_4> for _5 { type Output = _0; }
impl HexEqual<_4> for _6 { type Output = _0; }
impl HexEqual<_4> for _7 { type Output = _0; }
impl HexEqual<_4> for _8 { type Output = _0; }
impl HexEqual<_4> for _9 { type Output = _0; }
impl HexEqual<_4> for _A { type Output = _0; }
impl HexEqual<_4> for _B { type Output = _0; }
impl HexEqual<_4> for _C { type Output = _0; }
impl HexEqual<_4> for _D { type Output = _0; }
impl HexEqual<_4> for _E { type Output = _0; }
impl HexEqual<_4> for _F { type Output = _0; }
impl HexEqual<_5> for _0 { type Output = _0; }
impl HexEqual<_5> for _1 { type Output = _0; }
impl HexEqual<_5> for _2 { type Output = _0; }
impl HexEqual<_5> for _3 { type Output = _0; }
impl HexEqual<_5> for _4 { type Output = _0; }
impl HexEqual<_5> for _5 { type Output = _1; }
impl HexEqual<_5> for _6 { type Output = _0; }
impl HexEqual<_5> for _7 { type Output = _0; }
impl HexEqual<_5> for _8 { type Output = _0; }
impl HexEqual<_5> for _9 { type Output = _0; }
impl HexEqual<_5> for _A { type Output = _0; }
impl HexEqual<_5> for _B { type Output = _0; }
impl HexEqual<_5> for _C { type Output = _0; }
impl HexEqual<_5> for _D { type Output = _0; }
impl HexEqual<_5> for _E { type Output = _0; }
impl HexEqual<_5> for _F { type Output = _0; }
impl HexEqual<_6> for _0 { type Output = _0; }
impl HexEqual<_6> for _1 { type Output = _0; }
impl HexEqual<_6> for _2 { type Output = _0; }
impl HexEqual<_6> for _3 { type Output = _0; }
impl HexEqual<_6> for _4 { type Output = _0; }
impl HexEqual<_6> for _5 { type Output = _0; }
impl HexEqual<_6> for _6 { type Output = _1; }
impl HexEqual<_6> for _7 { type Output = _0; }
impl HexEqual<_6> for _8 { type Output = _0; }
impl HexEqual<_6> for _9 { type Output = _0; }
impl HexEqual<_6> for _A { type Output = _0; }
impl HexEqual<_6> for _B { type Output = _0; }
impl HexEqual<_6> for _C { type Output = _0; }
impl HexEqual<_6> for _D { type Output = _0; }
impl HexEqual<_6> for _E { type Output = _0; }
impl HexEqual<_6> for _F { type Output = _0; }
impl HexEqual<_7> for _0 { type Output = _0; }
impl HexEqual<_7> for _1 { type Output = _0; }
impl HexEqual<_7> for _2 { type Output = _0; }
impl HexEqual<_7> for _3 { type Output = _0; }
impl HexEqual<_7> for _4 { type Output = _0; }
impl HexEqual<_7> for _5 { type Output = _0; }
impl HexEqual<_7> for _6 { type Output = _0; }
impl HexEqual<_7> for _7 { type Output = _1; }
impl HexEqual<_7> for _8 { type Output = _0; }
impl HexEqual<_7> for _9 { type Output = _0; }
impl HexEqual<_7> for _A { type Output = _0; }
impl HexEqual<_7> for _B { type Output = _0; }
impl HexEqual<_7> for _C { type Output = _0; }
impl HexEqual<_7> for _D { type Output = _0; }
impl HexEqual<_7> for _E { type Output = _0; }
impl HexEqual<_7> for _F { type Output = _0; }
impl HexEqual<_8> for _0 { type Output = _0; }
impl HexEqual<_8> for _1 { type Output = _0; }
impl HexEqual<_8> for _2 { type Output = _0; }
impl HexEqual<_8> for _3 { type Output = _0; }
impl HexEqual<_8> for _4 { type Output = _0; }
impl HexEqual<_8> for _5 { type Output = _0; }
impl HexEqual<_8> for _6 { type Output = _0; }
impl HexEqual<_8> for _7 { type Output = _0; }
impl HexEqual<_8> for _8 { type Output = _1; }
impl HexEqual<_8> for _9 { type Output = _0; }
impl HexEqual<_8> for _A { type Output = _0; }
impl HexEqual<_8> for _B { type Output = _0; }
impl HexEqual<_8> for _C { type Output = _0; }
impl HexEqual<_8> for _D { type Output = _0; }
impl HexEqual<_8> for _E { type Output = _0; }
impl HexEqual<_8> for _F { type Output = _0; }
impl HexEqual<_9> for _0 { type Output = _0; }
impl HexEqual<_9> for _1 { type Output = _0; }
impl HexEqual<_9> for _2 { type Output = _0; }
impl HexEqual<_9> for _3 { type Output = _0; }
impl HexEqual<_9> for _4 { type Output = _0; }
impl HexEqual<_9> for _5 { type Output = _0; }
impl HexEqual<_9> for _6 { type Output = _0; }
impl HexEqual<_9> for _7 { type Output = _0; }
impl HexEqual<_9> for _8 { type Output = _0; }
impl HexEqual<_9> for _9 { type Output = _1; }
impl HexEqual<_9> for _A { type Output = _0; }
impl HexEqual<_9> for _B { type Output = _0; }
impl HexEqual<_9> for _C { type Output = _0; }
impl HexEqual<_9> for _D { type Output = _0; }
impl HexEqual<_9> for _E { type Output = _0; }
impl HexEqual<_9> for _F { type Output = _0; }
impl HexEqual<_A> for _0 { type Output = _0; }
impl HexEqual<_A> for _1 { type Output = _0; }
impl HexEqual<_A> for _2 { type Output = _0; }
impl HexEqual<_A> for _3 { type Output = _0; }
impl HexEqual<_A> for _4 { type Output = _0; }
impl HexEqual<_A> for _5 { type Output = _0; }
impl HexEqual<_A> for _6 { type Output = _0; }
impl HexEqual<_A> for _7 { type Output = _0; }
impl HexEqual<_A> for _8 { type Output = _0; }
impl HexEqual<_A> for _9 { type Output = _0; }
impl HexEqual<_A> for _A { type Output = _1; }
impl HexEqual<_A> for _B { type Output = _0; }
impl HexEqual<_A> for _C { type Output = _0; }
impl HexEqual<_A> for _D { type Output = _0; }
impl HexEqual<_A> for _E { type Output = _0; }
impl HexEqual<_A> for _F { type Output = _0; }
impl HexEqual<_B> for _0 { type Output = _0; }
impl HexEqual<_B> for _1 { type Output = _0; }
impl HexEqual<_B> for _2 { type Output = _0; }
impl HexEqual<_B> for _3 { type Output = _0; }
impl HexEqual<_B> for _4 { type Output = _0; }
impl HexEqual<_B> for _5 { type Output = _0; }
impl HexEqual<_B> for _6 { type Output = _0; }
impl HexEqual<_B> for _7 { type Output = _0; }
impl HexEqual<_B> for _8 { type Output = _0; }
impl HexEqual<_B> for _9 { type Output = _0; }
impl HexEqual<_B> for _A { type Output = _0; }
impl HexEqual<_B> for _B { type Output = _1; }
impl HexEqual<_B> for _C { type Output = _0; }
impl HexEqual<_B> for _D { type Output = _0; }
impl HexEqual<_B> for _E { type Output = _0; }
impl HexEqual<_B> for _F { type Output = _0; }
impl HexEqual<_C> for _0 { type Output = _0; }
impl HexEqual<_C> for _1 { type Output = _0; }
impl HexEqual<_C> for _2 { type Output = _0; }
impl HexEqual<_C> for _3 { type Output = _0; }
impl HexEqual<_C> for _4 { type Output = _0; }
impl HexEqual<_C> for _5 { type Output = _0; }
impl HexEqual<_C> for _6 { type Output = _0; }
impl HexEqual<_C> for _7 { type Output = _0; }
impl HexEqual<_C> for _8 { type Output = _0; }
impl HexEqual<_C> for _9 { type Output = _0; }
impl HexEqual<_C> for _A { type Output = _0; }
impl HexEqual<_C> for _B { type Output = _0; }
impl HexEqual<_C> for _C { type Output = _1; }
impl HexEqual<_C> for _D { type Output = _0; }
impl HexEqual<_C> for _E { type Output = _0; }
impl HexEqual<_C> for _F { type Output = _0; }
impl HexEqual<_D> for _0 { type Output = _0; }
impl HexEqual<_D> for _1 { type Output = _0; }
impl HexEqual<_D> for _2 { type Output = _0; }
impl HexEqual<_D> for _3 { type Output = _0; }
impl HexEqual<_D> for _4 { type Output = _0; }
impl HexEqual<_D> for _5 { type Output = _0; }
impl HexEqual<_D> for _6 { type Output = _0; }
impl HexEqual<_D> for _7 { type Output = _0; }
impl HexEqual<_D> for _8 { type Output = _0; }
impl HexEqual<_D> for _9 { type Output = _0; }
impl HexEqual<_D> for _A { type Output = _0; }
impl HexEqual<_D> for _B { type Output = _0; }
impl HexEqual<_D> for _C { type Output = _0; }
impl HexEqual<_D> for _D { type Output = _1; }
impl HexEqual<_D> for _E { type Output = _0; }
impl HexEqual<_D> for _F { type Output = _0; }
impl HexEqual<_E> for _0 { type Output = _0; }
impl HexEqual<_E> for _1 { type Output = _0; }
impl HexEqual<_E> for _2 { type Output = _0; }
impl HexEqual<_E> for _3 { type Output = _0; }
impl HexEqual<_E> for _4 { type Output = _0; }
impl HexEqual<_E> for _5 { type Output = _0; }
impl HexEqual<_E> for _6 { type Output = _0; }
impl HexEqual<_E> for _7 { type Output = _0; }
impl HexEqual<_E> for _8 { type Output = _0; }
impl HexEqual<_E> for _9 { type Output = _0; }
impl HexEqual<_E> for _A { type Output = _0; }
impl HexEqual<_E> for _B { type Output = _0; }
impl HexEqual<_E> for _C { type Output = _0; }
impl HexEqual<_E> for _D { type Output = _0; }
impl HexEqual<_E> for _E { type Output = _1; }
impl HexEqual<_E> for _F { type Output = _0; }
impl HexEqual<_F> for _0 { type Output = _0; }
impl HexEqual<_F> for _1 { type Output = _0; }
impl HexEqual<_F> for _2 { type Output = _0; }
impl HexEqual<_F> for _3 { type Output = _0; }
impl HexEqual<_F> for _4 { type Output = _0; }
impl HexEqual<_F> for _5 { type Output = _0; }
impl HexEqual<_F> for _6 { type Output = _0; }
impl HexEqual<_F> for _7 { type Output = _0; }
impl HexEqual<_F> for _8 { type Output = _0; }
impl HexEqual<_F> for _9 { type Output = _0; }
impl HexEqual<_F> for _A { type Output = _0; }
impl HexEqual<_F> for _B { type Output = _0; }
impl HexEqual<_F> for _C { type Output = _0; }
impl HexEqual<_F> for _D { type Output = _0; }
impl HexEqual<_F> for _E { type Output = _0; }
impl HexEqual<_F> for _F { type Output = _1; }

/// This trait is the equal trait on hex numbers with an assertion quality. False ones are simply not implemented
pub trait HexAssertEqual<H: Hex> { }
impl HexAssertEqual<_0> for _0 { }
impl HexAssertEqual<_1> for _1 { }
impl HexAssertEqual<_2> for _2 { }
impl HexAssertEqual<_3> for _3 { }
impl HexAssertEqual<_4> for _4 { }
impl HexAssertEqual<_5> for _5 { }
impl HexAssertEqual<_6> for _6 { }
impl HexAssertEqual<_7> for _7 { }
impl HexAssertEqual<_8> for _8 { }
impl HexAssertEqual<_9> for _9 { }
impl HexAssertEqual<_A> for _A { }
impl HexAssertEqual<_B> for _B { }
impl HexAssertEqual<_C> for _C { }
impl HexAssertEqual<_D> for _D { }
impl HexAssertEqual<_E> for _E { }
impl HexAssertEqual<_F> for _F { }

/// This represents 15 - x: the hexadecimal complement
pub trait HexNot { type Output: Hex; }
impl HexNot for _0 { type Output = _F; }
impl HexNot for _1 { type Output = _E; }
impl HexNot for _2 { type Output = _D; }
impl HexNot for _3 { type Output = _C; }
impl HexNot for _4 { type Output = _B; }
impl HexNot for _5 { type Output = _A; }
impl HexNot for _6 { type Output = _9; }
impl HexNot for _7 { type Output = _8; }
impl HexNot for _8 { type Output = _7; }
impl HexNot for _9 { type Output = _6; }
impl HexNot for _A { type Output = _5; }
impl HexNot for _B { type Output = _4; }
impl HexNot for _C { type Output = _3; }
impl HexNot for _D { type Output = _2; }
impl HexNot for _E { type Output = _1; }
impl HexNot for _F { type Output = _0; }

/// This is an internal implementation of multiplication of two hexadecimal
pub trait HexMul<H: Hex> { type Output: Hex; type Carry: Hex; }
impl HexMul<_0> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _1 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _2 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _3 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _4 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _5 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _6 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _7 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _8 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _9 { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _A { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _B { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _C { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _D { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _E { type Output = _0; type Carry = _0; }
impl HexMul<_0> for _F { type Output = _0; type Carry = _0; }
impl HexMul<_1> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_1> for _1 { type Output = _1; type Carry = _0; }
impl HexMul<_1> for _2 { type Output = _2; type Carry = _0; }
impl HexMul<_1> for _3 { type Output = _3; type Carry = _0; }
impl HexMul<_1> for _4 { type Output = _4; type Carry = _0; }
impl HexMul<_1> for _5 { type Output = _5; type Carry = _0; }
impl HexMul<_1> for _6 { type Output = _6; type Carry = _0; }
impl HexMul<_1> for _7 { type Output = _7; type Carry = _0; }
impl HexMul<_1> for _8 { type Output = _8; type Carry = _0; }
impl HexMul<_1> for _9 { type Output = _9; type Carry = _0; }
impl HexMul<_1> for _A { type Output = _A; type Carry = _0; }
impl HexMul<_1> for _B { type Output = _B; type Carry = _0; }
impl HexMul<_1> for _C { type Output = _C; type Carry = _0; }
impl HexMul<_1> for _D { type Output = _D; type Carry = _0; }
impl HexMul<_1> for _E { type Output = _E; type Carry = _0; }
impl HexMul<_1> for _F { type Output = _F; type Carry = _0; }
impl HexMul<_2> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_2> for _1 { type Output = _2; type Carry = _0; }
impl HexMul<_2> for _2 { type Output = _4; type Carry = _0; }
impl HexMul<_2> for _3 { type Output = _6; type Carry = _0; }
impl HexMul<_2> for _4 { type Output = _8; type Carry = _0; }
impl HexMul<_2> for _5 { type Output = _A; type Carry = _0; }
impl HexMul<_2> for _6 { type Output = _C; type Carry = _0; }
impl HexMul<_2> for _7 { type Output = _E; type Carry = _0; }
impl HexMul<_2> for _8 { type Output = _0; type Carry = _1; }
impl HexMul<_2> for _9 { type Output = _2; type Carry = _1; }
impl HexMul<_2> for _A { type Output = _4; type Carry = _1; }
impl HexMul<_2> for _B { type Output = _6; type Carry = _1; }
impl HexMul<_2> for _C { type Output = _8; type Carry = _1; }
impl HexMul<_2> for _D { type Output = _A; type Carry = _1; }
impl HexMul<_2> for _E { type Output = _C; type Carry = _1; }
impl HexMul<_2> for _F { type Output = _E; type Carry = _1; }
impl HexMul<_3> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_3> for _1 { type Output = _3; type Carry = _0; }
impl HexMul<_3> for _2 { type Output = _6; type Carry = _0; }
impl HexMul<_3> for _3 { type Output = _9; type Carry = _0; }
impl HexMul<_3> for _4 { type Output = _C; type Carry = _0; }
impl HexMul<_3> for _5 { type Output = _F; type Carry = _0; }
impl HexMul<_3> for _6 { type Output = _2; type Carry = _1; }
impl HexMul<_3> for _7 { type Output = _5; type Carry = _1; }
impl HexMul<_3> for _8 { type Output = _8; type Carry = _1; }
impl HexMul<_3> for _9 { type Output = _B; type Carry = _1; }
impl HexMul<_3> for _A { type Output = _E; type Carry = _1; }
impl HexMul<_3> for _B { type Output = _1; type Carry = _2; }
impl HexMul<_3> for _C { type Output = _4; type Carry = _2; }
impl HexMul<_3> for _D { type Output = _7; type Carry = _2; }
impl HexMul<_3> for _E { type Output = _A; type Carry = _2; }
impl HexMul<_3> for _F { type Output = _D; type Carry = _2; }
impl HexMul<_4> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_4> for _1 { type Output = _4; type Carry = _0; }
impl HexMul<_4> for _2 { type Output = _8; type Carry = _0; }
impl HexMul<_4> for _3 { type Output = _C; type Carry = _0; }
impl HexMul<_4> for _4 { type Output = _0; type Carry = _1; }
impl HexMul<_4> for _5 { type Output = _4; type Carry = _1; }
impl HexMul<_4> for _6 { type Output = _8; type Carry = _1; }
impl HexMul<_4> for _7 { type Output = _C; type Carry = _1; }
impl HexMul<_4> for _8 { type Output = _0; type Carry = _2; }
impl HexMul<_4> for _9 { type Output = _4; type Carry = _2; }
impl HexMul<_4> for _A { type Output = _8; type Carry = _2; }
impl HexMul<_4> for _B { type Output = _C; type Carry = _2; }
impl HexMul<_4> for _C { type Output = _0; type Carry = _3; }
impl HexMul<_4> for _D { type Output = _4; type Carry = _3; }
impl HexMul<_4> for _E { type Output = _8; type Carry = _3; }
impl HexMul<_4> for _F { type Output = _C; type Carry = _3; }
impl HexMul<_5> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_5> for _1 { type Output = _5; type Carry = _0; }
impl HexMul<_5> for _2 { type Output = _A; type Carry = _0; }
impl HexMul<_5> for _3 { type Output = _F; type Carry = _0; }
impl HexMul<_5> for _4 { type Output = _4; type Carry = _1; }
impl HexMul<_5> for _5 { type Output = _9; type Carry = _1; }
impl HexMul<_5> for _6 { type Output = _E; type Carry = _1; }
impl HexMul<_5> for _7 { type Output = _3; type Carry = _2; }
impl HexMul<_5> for _8 { type Output = _8; type Carry = _2; }
impl HexMul<_5> for _9 { type Output = _D; type Carry = _2; }
impl HexMul<_5> for _A { type Output = _2; type Carry = _3; }
impl HexMul<_5> for _B { type Output = _7; type Carry = _3; }
impl HexMul<_5> for _C { type Output = _C; type Carry = _3; }
impl HexMul<_5> for _D { type Output = _1; type Carry = _4; }
impl HexMul<_5> for _E { type Output = _6; type Carry = _4; }
impl HexMul<_5> for _F { type Output = _B; type Carry = _4; }
impl HexMul<_6> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_6> for _1 { type Output = _6; type Carry = _0; }
impl HexMul<_6> for _2 { type Output = _C; type Carry = _0; }
impl HexMul<_6> for _3 { type Output = _2; type Carry = _1; }
impl HexMul<_6> for _4 { type Output = _8; type Carry = _1; }
impl HexMul<_6> for _5 { type Output = _E; type Carry = _1; }
impl HexMul<_6> for _6 { type Output = _4; type Carry = _2; }
impl HexMul<_6> for _7 { type Output = _A; type Carry = _2; }
impl HexMul<_6> for _8 { type Output = _0; type Carry = _3; }
impl HexMul<_6> for _9 { type Output = _6; type Carry = _3; }
impl HexMul<_6> for _A { type Output = _C; type Carry = _3; }
impl HexMul<_6> for _B { type Output = _2; type Carry = _4; }
impl HexMul<_6> for _C { type Output = _8; type Carry = _4; }
impl HexMul<_6> for _D { type Output = _E; type Carry = _4; }
impl HexMul<_6> for _E { type Output = _4; type Carry = _5; }
impl HexMul<_6> for _F { type Output = _A; type Carry = _5; }
impl HexMul<_7> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_7> for _1 { type Output = _7; type Carry = _0; }
impl HexMul<_7> for _2 { type Output = _E; type Carry = _0; }
impl HexMul<_7> for _3 { type Output = _5; type Carry = _1; }
impl HexMul<_7> for _4 { type Output = _C; type Carry = _1; }
impl HexMul<_7> for _5 { type Output = _3; type Carry = _2; }
impl HexMul<_7> for _6 { type Output = _A; type Carry = _2; }
impl HexMul<_7> for _7 { type Output = _1; type Carry = _3; }
impl HexMul<_7> for _8 { type Output = _8; type Carry = _3; }
impl HexMul<_7> for _9 { type Output = _F; type Carry = _3; }
impl HexMul<_7> for _A { type Output = _6; type Carry = _4; }
impl HexMul<_7> for _B { type Output = _D; type Carry = _4; }
impl HexMul<_7> for _C { type Output = _4; type Carry = _5; }
impl HexMul<_7> for _D { type Output = _B; type Carry = _5; }
impl HexMul<_7> for _E { type Output = _2; type Carry = _6; }
impl HexMul<_7> for _F { type Output = _9; type Carry = _6; }
impl HexMul<_8> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_8> for _1 { type Output = _8; type Carry = _0; }
impl HexMul<_8> for _2 { type Output = _0; type Carry = _1; }
impl HexMul<_8> for _3 { type Output = _8; type Carry = _1; }
impl HexMul<_8> for _4 { type Output = _0; type Carry = _2; }
impl HexMul<_8> for _5 { type Output = _8; type Carry = _2; }
impl HexMul<_8> for _6 { type Output = _0; type Carry = _3; }
impl HexMul<_8> for _7 { type Output = _8; type Carry = _3; }
impl HexMul<_8> for _8 { type Output = _0; type Carry = _4; }
impl HexMul<_8> for _9 { type Output = _8; type Carry = _4; }
impl HexMul<_8> for _A { type Output = _0; type Carry = _5; }
impl HexMul<_8> for _B { type Output = _8; type Carry = _5; }
impl HexMul<_8> for _C { type Output = _0; type Carry = _6; }
impl HexMul<_8> for _D { type Output = _8; type Carry = _6; }
impl HexMul<_8> for _E { type Output = _0; type Carry = _7; }
impl HexMul<_8> for _F { type Output = _8; type Carry = _7; }
impl HexMul<_9> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_9> for _1 { type Output = _9; type Carry = _0; }
impl HexMul<_9> for _2 { type Output = _2; type Carry = _1; }
impl HexMul<_9> for _3 { type Output = _B; type Carry = _1; }
impl HexMul<_9> for _4 { type Output = _4; type Carry = _2; }
impl HexMul<_9> for _5 { type Output = _D; type Carry = _2; }
impl HexMul<_9> for _6 { type Output = _6; type Carry = _3; }
impl HexMul<_9> for _7 { type Output = _F; type Carry = _3; }
impl HexMul<_9> for _8 { type Output = _8; type Carry = _4; }
impl HexMul<_9> for _9 { type Output = _1; type Carry = _5; }
impl HexMul<_9> for _A { type Output = _A; type Carry = _5; }
impl HexMul<_9> for _B { type Output = _3; type Carry = _6; }
impl HexMul<_9> for _C { type Output = _C; type Carry = _6; }
impl HexMul<_9> for _D { type Output = _5; type Carry = _7; }
impl HexMul<_9> for _E { type Output = _E; type Carry = _7; }
impl HexMul<_9> for _F { type Output = _7; type Carry = _8; }
impl HexMul<_A> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_A> for _1 { type Output = _A; type Carry = _0; }
impl HexMul<_A> for _2 { type Output = _4; type Carry = _1; }
impl HexMul<_A> for _3 { type Output = _E; type Carry = _1; }
impl HexMul<_A> for _4 { type Output = _8; type Carry = _2; }
impl HexMul<_A> for _5 { type Output = _2; type Carry = _3; }
impl HexMul<_A> for _6 { type Output = _C; type Carry = _3; }
impl HexMul<_A> for _7 { type Output = _6; type Carry = _4; }
impl HexMul<_A> for _8 { type Output = _0; type Carry = _5; }
impl HexMul<_A> for _9 { type Output = _A; type Carry = _5; }
impl HexMul<_A> for _A { type Output = _4; type Carry = _6; }
impl HexMul<_A> for _B { type Output = _E; type Carry = _6; }
impl HexMul<_A> for _C { type Output = _8; type Carry = _7; }
impl HexMul<_A> for _D { type Output = _2; type Carry = _8; }
impl HexMul<_A> for _E { type Output = _C; type Carry = _8; }
impl HexMul<_A> for _F { type Output = _6; type Carry = _9; }
impl HexMul<_B> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_B> for _1 { type Output = _B; type Carry = _0; }
impl HexMul<_B> for _2 { type Output = _6; type Carry = _1; }
impl HexMul<_B> for _3 { type Output = _1; type Carry = _2; }
impl HexMul<_B> for _4 { type Output = _C; type Carry = _2; }
impl HexMul<_B> for _5 { type Output = _7; type Carry = _3; }
impl HexMul<_B> for _6 { type Output = _2; type Carry = _4; }
impl HexMul<_B> for _7 { type Output = _D; type Carry = _4; }
impl HexMul<_B> for _8 { type Output = _8; type Carry = _5; }
impl HexMul<_B> for _9 { type Output = _3; type Carry = _6; }
impl HexMul<_B> for _A { type Output = _E; type Carry = _6; }
impl HexMul<_B> for _B { type Output = _9; type Carry = _7; }
impl HexMul<_B> for _C { type Output = _4; type Carry = _8; }
impl HexMul<_B> for _D { type Output = _F; type Carry = _8; }
impl HexMul<_B> for _E { type Output = _A; type Carry = _9; }
impl HexMul<_B> for _F { type Output = _5; type Carry = _A; }
impl HexMul<_C> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_C> for _1 { type Output = _C; type Carry = _0; }
impl HexMul<_C> for _2 { type Output = _8; type Carry = _1; }
impl HexMul<_C> for _3 { type Output = _4; type Carry = _2; }
impl HexMul<_C> for _4 { type Output = _0; type Carry = _3; }
impl HexMul<_C> for _5 { type Output = _C; type Carry = _3; }
impl HexMul<_C> for _6 { type Output = _8; type Carry = _4; }
impl HexMul<_C> for _7 { type Output = _4; type Carry = _5; }
impl HexMul<_C> for _8 { type Output = _0; type Carry = _6; }
impl HexMul<_C> for _9 { type Output = _C; type Carry = _6; }
impl HexMul<_C> for _A { type Output = _8; type Carry = _7; }
impl HexMul<_C> for _B { type Output = _4; type Carry = _8; }
impl HexMul<_C> for _C { type Output = _0; type Carry = _9; }
impl HexMul<_C> for _D { type Output = _C; type Carry = _9; }
impl HexMul<_C> for _E { type Output = _8; type Carry = _A; }
impl HexMul<_C> for _F { type Output = _4; type Carry = _B; }
impl HexMul<_D> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_D> for _1 { type Output = _D; type Carry = _0; }
impl HexMul<_D> for _2 { type Output = _A; type Carry = _1; }
impl HexMul<_D> for _3 { type Output = _7; type Carry = _2; }
impl HexMul<_D> for _4 { type Output = _4; type Carry = _3; }
impl HexMul<_D> for _5 { type Output = _1; type Carry = _4; }
impl HexMul<_D> for _6 { type Output = _E; type Carry = _4; }
impl HexMul<_D> for _7 { type Output = _B; type Carry = _5; }
impl HexMul<_D> for _8 { type Output = _8; type Carry = _6; }
impl HexMul<_D> for _9 { type Output = _5; type Carry = _7; }
impl HexMul<_D> for _A { type Output = _2; type Carry = _8; }
impl HexMul<_D> for _B { type Output = _F; type Carry = _8; }
impl HexMul<_D> for _C { type Output = _C; type Carry = _9; }
impl HexMul<_D> for _D { type Output = _9; type Carry = _A; }
impl HexMul<_D> for _E { type Output = _6; type Carry = _B; }
impl HexMul<_D> for _F { type Output = _3; type Carry = _C; }
impl HexMul<_E> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_E> for _1 { type Output = _E; type Carry = _0; }
impl HexMul<_E> for _2 { type Output = _C; type Carry = _1; }
impl HexMul<_E> for _3 { type Output = _A; type Carry = _2; }
impl HexMul<_E> for _4 { type Output = _8; type Carry = _3; }
impl HexMul<_E> for _5 { type Output = _6; type Carry = _4; }
impl HexMul<_E> for _6 { type Output = _4; type Carry = _5; }
impl HexMul<_E> for _7 { type Output = _2; type Carry = _6; }
impl HexMul<_E> for _8 { type Output = _0; type Carry = _7; }
impl HexMul<_E> for _9 { type Output = _E; type Carry = _7; }
impl HexMul<_E> for _A { type Output = _C; type Carry = _8; }
impl HexMul<_E> for _B { type Output = _A; type Carry = _9; }
impl HexMul<_E> for _C { type Output = _8; type Carry = _A; }
impl HexMul<_E> for _D { type Output = _6; type Carry = _B; }
impl HexMul<_E> for _E { type Output = _4; type Carry = _C; }
impl HexMul<_E> for _F { type Output = _2; type Carry = _D; }
impl HexMul<_F> for _0 { type Output = _0; type Carry = _0; }
impl HexMul<_F> for _1 { type Output = _F; type Carry = _0; }
impl HexMul<_F> for _2 { type Output = _E; type Carry = _1; }
impl HexMul<_F> for _3 { type Output = _D; type Carry = _2; }
impl HexMul<_F> for _4 { type Output = _C; type Carry = _3; }
impl HexMul<_F> for _5 { type Output = _B; type Carry = _4; }
impl HexMul<_F> for _6 { type Output = _A; type Carry = _5; }
impl HexMul<_F> for _7 { type Output = _9; type Carry = _6; }
impl HexMul<_F> for _8 { type Output = _8; type Carry = _7; }
impl HexMul<_F> for _9 { type Output = _7; type Carry = _8; }
impl HexMul<_F> for _A { type Output = _6; type Carry = _9; }
impl HexMul<_F> for _B { type Output = _5; type Carry = _A; }
impl HexMul<_F> for _C { type Output = _4; type Carry = _B; }
impl HexMul<_F> for _D { type Output = _3; type Carry = _C; }
impl HexMul<_F> for _E { type Output = _2; type Carry = _D; }
impl HexMul<_F> for _F { type Output = _1; type Carry = _E; }
