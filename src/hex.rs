mod equal;
mod add;

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
