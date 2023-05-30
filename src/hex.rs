//! Implements the base hexadecimal type

/// This denotes one single hexadecimal - half a byte
pub trait Hex {}

/// This denotes the number 0
pub struct _0;
impl Hex for _0 {}

/// This denotes the number 1
pub struct _1;
impl Hex for _1 {}

/// This denotes the number 2
pub struct _2;
impl Hex for _2 {}

/// This denotes the number 3
pub struct _3;
impl Hex for _3 {}

/// This denotes the number 4
pub struct _4;
impl Hex for _4 {}

/// This denotes the number 5
pub struct _5;
impl Hex for _5 {}

/// This denotes the number 6
pub struct _6;
impl Hex for _6 {}

/// This denotes the number 7
pub struct _7;
impl Hex for _7 {}

/// This denotes the number 8
pub struct _8;
impl Hex for _8 {}

/// This denotes the number 9
pub struct _9;
impl Hex for _9 {}

/// This denotes the number A
pub struct _A;
impl Hex for _A {}

/// This denotes the number B
pub struct _B;
impl Hex for _B {}

/// This denotes the number C
pub struct _C;
impl Hex for _C {}

/// This denotes the number D
pub struct _D;
impl Hex for _D {}

/// This denotes the number E
pub struct _E;
impl Hex for _E {}

/// This denotes the number F
pub struct _F;
impl Hex for _F {}

/// This is an internal implementation of addition without carryover
pub trait HexAdd1<H: Hex> { type Output: Hex; type Carry: Hex; }
impl HexAdd1<_0> for _0 { type Output = _0; type Carry = _0; }
impl HexAdd1<_0> for _1 { type Output = _1; type Carry = _0; }
impl HexAdd1<_0> for _2 { type Output = _2; type Carry = _0; }
impl HexAdd1<_0> for _3 { type Output = _3; type Carry = _0; }
impl HexAdd1<_0> for _4 { type Output = _4; type Carry = _0; }
impl HexAdd1<_0> for _5 { type Output = _5; type Carry = _0; }
impl HexAdd1<_0> for _6 { type Output = _6; type Carry = _0; }
impl HexAdd1<_0> for _7 { type Output = _7; type Carry = _0; }
impl HexAdd1<_0> for _8 { type Output = _8; type Carry = _0; }
impl HexAdd1<_0> for _9 { type Output = _9; type Carry = _0; }
impl HexAdd1<_0> for _A { type Output = _A; type Carry = _0; }
impl HexAdd1<_0> for _B { type Output = _B; type Carry = _0; }
impl HexAdd1<_0> for _C { type Output = _C; type Carry = _0; }
impl HexAdd1<_0> for _D { type Output = _D; type Carry = _0; }
impl HexAdd1<_0> for _E { type Output = _E; type Carry = _0; }
impl HexAdd1<_0> for _F { type Output = _F; type Carry = _0; }
impl HexAdd1<_1> for _0 { type Output = _1; type Carry = _0; }
impl HexAdd1<_1> for _1 { type Output = _2; type Carry = _0; }
impl HexAdd1<_1> for _2 { type Output = _3; type Carry = _0; }
impl HexAdd1<_1> for _3 { type Output = _4; type Carry = _0; }
impl HexAdd1<_1> for _4 { type Output = _5; type Carry = _0; }
impl HexAdd1<_1> for _5 { type Output = _6; type Carry = _0; }
impl HexAdd1<_1> for _6 { type Output = _7; type Carry = _0; }
impl HexAdd1<_1> for _7 { type Output = _8; type Carry = _0; }
impl HexAdd1<_1> for _8 { type Output = _9; type Carry = _0; }
impl HexAdd1<_1> for _9 { type Output = _A; type Carry = _0; }
impl HexAdd1<_1> for _A { type Output = _B; type Carry = _0; }
impl HexAdd1<_1> for _B { type Output = _C; type Carry = _0; }
impl HexAdd1<_1> for _C { type Output = _D; type Carry = _0; }
impl HexAdd1<_1> for _D { type Output = _E; type Carry = _0; }
impl HexAdd1<_1> for _E { type Output = _F; type Carry = _0; }
impl HexAdd1<_1> for _F { type Output = _0; type Carry = _1; }
impl HexAdd1<_2> for _0 { type Output = _2; type Carry = _0; }
impl HexAdd1<_2> for _1 { type Output = _3; type Carry = _0; }
impl HexAdd1<_2> for _2 { type Output = _4; type Carry = _0; }
impl HexAdd1<_2> for _3 { type Output = _5; type Carry = _0; }
impl HexAdd1<_2> for _4 { type Output = _6; type Carry = _0; }
impl HexAdd1<_2> for _5 { type Output = _7; type Carry = _0; }
impl HexAdd1<_2> for _6 { type Output = _8; type Carry = _0; }
impl HexAdd1<_2> for _7 { type Output = _9; type Carry = _0; }
impl HexAdd1<_2> for _8 { type Output = _A; type Carry = _0; }
impl HexAdd1<_2> for _9 { type Output = _B; type Carry = _0; }
impl HexAdd1<_2> for _A { type Output = _C; type Carry = _0; }
impl HexAdd1<_2> for _B { type Output = _D; type Carry = _0; }
impl HexAdd1<_2> for _C { type Output = _E; type Carry = _0; }
impl HexAdd1<_2> for _D { type Output = _F; type Carry = _0; }
impl HexAdd1<_2> for _E { type Output = _0; type Carry = _1; }
impl HexAdd1<_2> for _F { type Output = _1; type Carry = _1; }
impl HexAdd1<_3> for _0 { type Output = _3; type Carry = _0; }
impl HexAdd1<_3> for _1 { type Output = _4; type Carry = _0; }
impl HexAdd1<_3> for _2 { type Output = _5; type Carry = _0; }
impl HexAdd1<_3> for _3 { type Output = _6; type Carry = _0; }
impl HexAdd1<_3> for _4 { type Output = _7; type Carry = _0; }
impl HexAdd1<_3> for _5 { type Output = _8; type Carry = _0; }
impl HexAdd1<_3> for _6 { type Output = _9; type Carry = _0; }
impl HexAdd1<_3> for _7 { type Output = _A; type Carry = _0; }
impl HexAdd1<_3> for _8 { type Output = _B; type Carry = _0; }
impl HexAdd1<_3> for _9 { type Output = _C; type Carry = _0; }
impl HexAdd1<_3> for _A { type Output = _D; type Carry = _0; }
impl HexAdd1<_3> for _B { type Output = _E; type Carry = _0; }
impl HexAdd1<_3> for _C { type Output = _F; type Carry = _0; }
impl HexAdd1<_3> for _D { type Output = _0; type Carry = _1; }
impl HexAdd1<_3> for _E { type Output = _1; type Carry = _1; }
impl HexAdd1<_3> for _F { type Output = _2; type Carry = _1; }
impl HexAdd1<_4> for _0 { type Output = _4; type Carry = _0; }
impl HexAdd1<_4> for _1 { type Output = _5; type Carry = _0; }
impl HexAdd1<_4> for _2 { type Output = _6; type Carry = _0; }
impl HexAdd1<_4> for _3 { type Output = _7; type Carry = _0; }
impl HexAdd1<_4> for _4 { type Output = _8; type Carry = _0; }
impl HexAdd1<_4> for _5 { type Output = _9; type Carry = _0; }
impl HexAdd1<_4> for _6 { type Output = _A; type Carry = _0; }
impl HexAdd1<_4> for _7 { type Output = _B; type Carry = _0; }
impl HexAdd1<_4> for _8 { type Output = _C; type Carry = _0; }
impl HexAdd1<_4> for _9 { type Output = _D; type Carry = _0; }
impl HexAdd1<_4> for _A { type Output = _E; type Carry = _0; }
impl HexAdd1<_4> for _B { type Output = _F; type Carry = _0; }
impl HexAdd1<_4> for _C { type Output = _0; type Carry = _1; }
impl HexAdd1<_4> for _D { type Output = _1; type Carry = _1; }
impl HexAdd1<_4> for _E { type Output = _2; type Carry = _1; }
impl HexAdd1<_4> for _F { type Output = _3; type Carry = _1; }
impl HexAdd1<_5> for _0 { type Output = _5; type Carry = _0; }
impl HexAdd1<_5> for _1 { type Output = _6; type Carry = _0; }
impl HexAdd1<_5> for _2 { type Output = _7; type Carry = _0; }
impl HexAdd1<_5> for _3 { type Output = _8; type Carry = _0; }
impl HexAdd1<_5> for _4 { type Output = _9; type Carry = _0; }
impl HexAdd1<_5> for _5 { type Output = _A; type Carry = _0; }
impl HexAdd1<_5> for _6 { type Output = _B; type Carry = _0; }
impl HexAdd1<_5> for _7 { type Output = _C; type Carry = _0; }
impl HexAdd1<_5> for _8 { type Output = _D; type Carry = _0; }
impl HexAdd1<_5> for _9 { type Output = _E; type Carry = _0; }
impl HexAdd1<_5> for _A { type Output = _F; type Carry = _0; }
impl HexAdd1<_5> for _B { type Output = _0; type Carry = _1; }
impl HexAdd1<_5> for _C { type Output = _1; type Carry = _1; }
impl HexAdd1<_5> for _D { type Output = _2; type Carry = _1; }
impl HexAdd1<_5> for _E { type Output = _3; type Carry = _1; }
impl HexAdd1<_5> for _F { type Output = _4; type Carry = _1; }
impl HexAdd1<_6> for _0 { type Output = _6; type Carry = _0; }
impl HexAdd1<_6> for _1 { type Output = _7; type Carry = _0; }
impl HexAdd1<_6> for _2 { type Output = _8; type Carry = _0; }
impl HexAdd1<_6> for _3 { type Output = _9; type Carry = _0; }
impl HexAdd1<_6> for _4 { type Output = _A; type Carry = _0; }
impl HexAdd1<_6> for _5 { type Output = _B; type Carry = _0; }
impl HexAdd1<_6> for _6 { type Output = _C; type Carry = _0; }
impl HexAdd1<_6> for _7 { type Output = _D; type Carry = _0; }
impl HexAdd1<_6> for _8 { type Output = _E; type Carry = _0; }
impl HexAdd1<_6> for _9 { type Output = _F; type Carry = _0; }
impl HexAdd1<_6> for _A { type Output = _0; type Carry = _1; }
impl HexAdd1<_6> for _B { type Output = _1; type Carry = _1; }
impl HexAdd1<_6> for _C { type Output = _2; type Carry = _1; }
impl HexAdd1<_6> for _D { type Output = _3; type Carry = _1; }
impl HexAdd1<_6> for _E { type Output = _4; type Carry = _1; }
impl HexAdd1<_6> for _F { type Output = _5; type Carry = _1; }
impl HexAdd1<_7> for _0 { type Output = _7; type Carry = _0; }
impl HexAdd1<_7> for _1 { type Output = _8; type Carry = _0; }
impl HexAdd1<_7> for _2 { type Output = _9; type Carry = _0; }
impl HexAdd1<_7> for _3 { type Output = _A; type Carry = _0; }
impl HexAdd1<_7> for _4 { type Output = _B; type Carry = _0; }
impl HexAdd1<_7> for _5 { type Output = _C; type Carry = _0; }
impl HexAdd1<_7> for _6 { type Output = _D; type Carry = _0; }
impl HexAdd1<_7> for _7 { type Output = _E; type Carry = _0; }
impl HexAdd1<_7> for _8 { type Output = _F; type Carry = _0; }
impl HexAdd1<_7> for _9 { type Output = _0; type Carry = _1; }
impl HexAdd1<_7> for _A { type Output = _1; type Carry = _1; }
impl HexAdd1<_7> for _B { type Output = _2; type Carry = _1; }
impl HexAdd1<_7> for _C { type Output = _3; type Carry = _1; }
impl HexAdd1<_7> for _D { type Output = _4; type Carry = _1; }
impl HexAdd1<_7> for _E { type Output = _5; type Carry = _1; }
impl HexAdd1<_7> for _F { type Output = _6; type Carry = _1; }
impl HexAdd1<_8> for _0 { type Output = _8; type Carry = _0; }
impl HexAdd1<_8> for _1 { type Output = _9; type Carry = _0; }
impl HexAdd1<_8> for _2 { type Output = _A; type Carry = _0; }
impl HexAdd1<_8> for _3 { type Output = _B; type Carry = _0; }
impl HexAdd1<_8> for _4 { type Output = _C; type Carry = _0; }
impl HexAdd1<_8> for _5 { type Output = _D; type Carry = _0; }
impl HexAdd1<_8> for _6 { type Output = _E; type Carry = _0; }
impl HexAdd1<_8> for _7 { type Output = _F; type Carry = _0; }
impl HexAdd1<_8> for _8 { type Output = _0; type Carry = _1; }
impl HexAdd1<_8> for _9 { type Output = _1; type Carry = _1; }
impl HexAdd1<_8> for _A { type Output = _2; type Carry = _1; }
impl HexAdd1<_8> for _B { type Output = _3; type Carry = _1; }
impl HexAdd1<_8> for _C { type Output = _4; type Carry = _1; }
impl HexAdd1<_8> for _D { type Output = _5; type Carry = _1; }
impl HexAdd1<_8> for _E { type Output = _6; type Carry = _1; }
impl HexAdd1<_8> for _F { type Output = _7; type Carry = _1; }
impl HexAdd1<_9> for _0 { type Output = _9; type Carry = _0; }
impl HexAdd1<_9> for _1 { type Output = _A; type Carry = _0; }
impl HexAdd1<_9> for _2 { type Output = _B; type Carry = _0; }
impl HexAdd1<_9> for _3 { type Output = _C; type Carry = _0; }
impl HexAdd1<_9> for _4 { type Output = _D; type Carry = _0; }
impl HexAdd1<_9> for _5 { type Output = _E; type Carry = _0; }
impl HexAdd1<_9> for _6 { type Output = _F; type Carry = _0; }
impl HexAdd1<_9> for _7 { type Output = _0; type Carry = _1; }
impl HexAdd1<_9> for _8 { type Output = _1; type Carry = _1; }
impl HexAdd1<_9> for _9 { type Output = _2; type Carry = _1; }
impl HexAdd1<_9> for _A { type Output = _3; type Carry = _1; }
impl HexAdd1<_9> for _B { type Output = _4; type Carry = _1; }
impl HexAdd1<_9> for _C { type Output = _5; type Carry = _1; }
impl HexAdd1<_9> for _D { type Output = _6; type Carry = _1; }
impl HexAdd1<_9> for _E { type Output = _7; type Carry = _1; }
impl HexAdd1<_9> for _F { type Output = _8; type Carry = _1; }
impl HexAdd1<_A> for _0 { type Output = _A; type Carry = _0; }
impl HexAdd1<_A> for _1 { type Output = _B; type Carry = _0; }
impl HexAdd1<_A> for _2 { type Output = _C; type Carry = _0; }
impl HexAdd1<_A> for _3 { type Output = _D; type Carry = _0; }
impl HexAdd1<_A> for _4 { type Output = _E; type Carry = _0; }
impl HexAdd1<_A> for _5 { type Output = _F; type Carry = _0; }
impl HexAdd1<_A> for _6 { type Output = _0; type Carry = _1; }
impl HexAdd1<_A> for _7 { type Output = _1; type Carry = _1; }
impl HexAdd1<_A> for _8 { type Output = _2; type Carry = _1; }
impl HexAdd1<_A> for _9 { type Output = _3; type Carry = _1; }
impl HexAdd1<_A> for _A { type Output = _4; type Carry = _1; }
impl HexAdd1<_A> for _B { type Output = _5; type Carry = _1; }
impl HexAdd1<_A> for _C { type Output = _6; type Carry = _1; }
impl HexAdd1<_A> for _D { type Output = _7; type Carry = _1; }
impl HexAdd1<_A> for _E { type Output = _8; type Carry = _1; }
impl HexAdd1<_A> for _F { type Output = _9; type Carry = _1; }
impl HexAdd1<_B> for _0 { type Output = _B; type Carry = _0; }
impl HexAdd1<_B> for _1 { type Output = _C; type Carry = _0; }
impl HexAdd1<_B> for _2 { type Output = _D; type Carry = _0; }
impl HexAdd1<_B> for _3 { type Output = _E; type Carry = _0; }
impl HexAdd1<_B> for _4 { type Output = _F; type Carry = _0; }
impl HexAdd1<_B> for _5 { type Output = _0; type Carry = _1; }
impl HexAdd1<_B> for _6 { type Output = _1; type Carry = _1; }
impl HexAdd1<_B> for _7 { type Output = _2; type Carry = _1; }
impl HexAdd1<_B> for _8 { type Output = _3; type Carry = _1; }
impl HexAdd1<_B> for _9 { type Output = _4; type Carry = _1; }
impl HexAdd1<_B> for _A { type Output = _5; type Carry = _1; }
impl HexAdd1<_B> for _B { type Output = _6; type Carry = _1; }
impl HexAdd1<_B> for _C { type Output = _7; type Carry = _1; }
impl HexAdd1<_B> for _D { type Output = _8; type Carry = _1; }
impl HexAdd1<_B> for _E { type Output = _9; type Carry = _1; }
impl HexAdd1<_B> for _F { type Output = _A; type Carry = _1; }
impl HexAdd1<_C> for _0 { type Output = _C; type Carry = _0; }
impl HexAdd1<_C> for _1 { type Output = _D; type Carry = _0; }
impl HexAdd1<_C> for _2 { type Output = _E; type Carry = _0; }
impl HexAdd1<_C> for _3 { type Output = _F; type Carry = _0; }
impl HexAdd1<_C> for _4 { type Output = _0; type Carry = _1; }
impl HexAdd1<_C> for _5 { type Output = _1; type Carry = _1; }
impl HexAdd1<_C> for _6 { type Output = _2; type Carry = _1; }
impl HexAdd1<_C> for _7 { type Output = _3; type Carry = _1; }
impl HexAdd1<_C> for _8 { type Output = _4; type Carry = _1; }
impl HexAdd1<_C> for _9 { type Output = _5; type Carry = _1; }
impl HexAdd1<_C> for _A { type Output = _6; type Carry = _1; }
impl HexAdd1<_C> for _B { type Output = _7; type Carry = _1; }
impl HexAdd1<_C> for _C { type Output = _8; type Carry = _1; }
impl HexAdd1<_C> for _D { type Output = _9; type Carry = _1; }
impl HexAdd1<_C> for _E { type Output = _A; type Carry = _1; }
impl HexAdd1<_C> for _F { type Output = _B; type Carry = _1; }
impl HexAdd1<_D> for _0 { type Output = _D; type Carry = _0; }
impl HexAdd1<_D> for _1 { type Output = _E; type Carry = _0; }
impl HexAdd1<_D> for _2 { type Output = _F; type Carry = _0; }
impl HexAdd1<_D> for _3 { type Output = _0; type Carry = _1; }
impl HexAdd1<_D> for _4 { type Output = _1; type Carry = _1; }
impl HexAdd1<_D> for _5 { type Output = _2; type Carry = _1; }
impl HexAdd1<_D> for _6 { type Output = _3; type Carry = _1; }
impl HexAdd1<_D> for _7 { type Output = _4; type Carry = _1; }
impl HexAdd1<_D> for _8 { type Output = _5; type Carry = _1; }
impl HexAdd1<_D> for _9 { type Output = _6; type Carry = _1; }
impl HexAdd1<_D> for _A { type Output = _7; type Carry = _1; }
impl HexAdd1<_D> for _B { type Output = _8; type Carry = _1; }
impl HexAdd1<_D> for _C { type Output = _9; type Carry = _1; }
impl HexAdd1<_D> for _D { type Output = _A; type Carry = _1; }
impl HexAdd1<_D> for _E { type Output = _B; type Carry = _1; }
impl HexAdd1<_D> for _F { type Output = _C; type Carry = _1; }
impl HexAdd1<_E> for _0 { type Output = _E; type Carry = _0; }
impl HexAdd1<_E> for _1 { type Output = _F; type Carry = _0; }
impl HexAdd1<_E> for _2 { type Output = _0; type Carry = _1; }
impl HexAdd1<_E> for _3 { type Output = _1; type Carry = _1; }
impl HexAdd1<_E> for _4 { type Output = _2; type Carry = _1; }
impl HexAdd1<_E> for _5 { type Output = _3; type Carry = _1; }
impl HexAdd1<_E> for _6 { type Output = _4; type Carry = _1; }
impl HexAdd1<_E> for _7 { type Output = _5; type Carry = _1; }
impl HexAdd1<_E> for _8 { type Output = _6; type Carry = _1; }
impl HexAdd1<_E> for _9 { type Output = _7; type Carry = _1; }
impl HexAdd1<_E> for _A { type Output = _8; type Carry = _1; }
impl HexAdd1<_E> for _B { type Output = _9; type Carry = _1; }
impl HexAdd1<_E> for _C { type Output = _A; type Carry = _1; }
impl HexAdd1<_E> for _D { type Output = _B; type Carry = _1; }
impl HexAdd1<_E> for _E { type Output = _C; type Carry = _1; }
impl HexAdd1<_E> for _F { type Output = _D; type Carry = _1; }
impl HexAdd1<_F> for _0 { type Output = _F; type Carry = _0; }
impl HexAdd1<_F> for _1 { type Output = _0; type Carry = _1; }
impl HexAdd1<_F> for _2 { type Output = _1; type Carry = _1; }
impl HexAdd1<_F> for _3 { type Output = _2; type Carry = _1; }
impl HexAdd1<_F> for _4 { type Output = _3; type Carry = _1; }
impl HexAdd1<_F> for _5 { type Output = _4; type Carry = _1; }
impl HexAdd1<_F> for _6 { type Output = _5; type Carry = _1; }
impl HexAdd1<_F> for _7 { type Output = _6; type Carry = _1; }
impl HexAdd1<_F> for _8 { type Output = _7; type Carry = _1; }
impl HexAdd1<_F> for _9 { type Output = _8; type Carry = _1; }
impl HexAdd1<_F> for _A { type Output = _9; type Carry = _1; }
impl HexAdd1<_F> for _B { type Output = _A; type Carry = _1; }
impl HexAdd1<_F> for _C { type Output = _B; type Carry = _1; }
impl HexAdd1<_F> for _D { type Output = _C; type Carry = _1; }
impl HexAdd1<_F> for _E { type Output = _D; type Carry = _1; }
impl HexAdd1<_F> for _F { type Output = _E; type Carry = _1; }

/// This is an internal implementation of addition with carryover
pub trait HexAdd2<H: Hex, C: Hex> { type Output: Hex; type Carry: Hex; }

impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _0 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_0, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _1 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_1, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _2 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_2, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _3 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_3, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _4 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_4, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _5 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_5, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _6 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_6, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _7 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_7, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _8 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_8, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _9 where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_9, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _A where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_A, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _B where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_B, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _C where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_C, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _D where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_D, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _E where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_E, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}


impl<H, C, R1, C1, R2, C2, Cr, C_> HexAdd2<H, C> for _F where H: Hex, C: Hex, R1: Hex, R2: Hex, C1: Hex, C2: Hex, Cr: Hex, H: HexAdd1<_F, Output = R1, Carry = C1>, R1: HexAdd1<C, Output = R2, Carry = C2>, C1: HexAdd1<C2, Output = Cr, Carry = C_> {
    type Output = R2;
    type Carry = Cr;
}

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
