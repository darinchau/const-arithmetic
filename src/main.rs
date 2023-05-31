use const_arithmetic::*;

fn div_equal<H, K, Q, R>(a: H, b: K, c: Q, d: R) where
H: IsInteger,
K: IsInteger,
Q: IsInteger,
R: IsInteger,
H: Div<K, Output = Q, Remainder = R> {

}

fn main() {
    let h40 = parse_integer!(862761); 
    let k40 = parse_integer!(512); 
    let q40 = parse_integer!(1685); 
    let r40 = parse_integer!(41); 
    div_equal(h40,k40, q40, r40);
}