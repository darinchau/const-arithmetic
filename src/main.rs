use const_arithmetic::*;

fn div_equal<H, K, Q, R>(a: H, b: K, c: Q, d: R) where
H: IsInteger,
K: IsInteger,
Q: IsInteger,
R: IsInteger,
H: Div<K, Output = Q, Remainder = R> {

}

fn mul_equal<P, Q, R, S>(_p: P, _q: Q, _r: R, _s: S) where
P: IsInteger,
Q: IsInteger,
R: IsInteger,
S: IsInteger,
P: Mul<Q, Output = S, Overflow = R>
{}

fn less_than<P, Q, R>(_p: P, _q: Q) where 
P: IsInteger,
Q: IsInteger,
R: Binary,
P: TypedLessThan<Q, Output = R>,
R: AssertFalse {}

fn typed_equal<P>(_p: P) where
P: TypedEqual<TypedInteger<_0, _0, _0, _0, _0, _0, _0, _0>> {}

fn main() {
    let h40 = parse_integer!(97); 
    let k40 = parse_integer!(32); 
    let q40 = parse_integer!(3); 
    let r40 = parse_integer!(1); 
    div_equal(h40, k40, q40, r40);

    // let p0 = parse_integer!(97);
    // let q0 = parse_integer!(2147483648);
    // let s0 = parse_integer!(2147483648);
    // let r0 = parse_integer!(48);
    // mul_equal(p0, q0, r0, s0);
    // mul_equal(q0, p0, r0, s0);

    // less_than(s0, p0);
}