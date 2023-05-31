//! If this compiles then all tests passed
use super::*;

fn div_equal<H, K, Q, R>(_a: H, _b: K, _c: Q, _d: R) where H: IsInteger, K: IsInteger, Q: IsInteger, R: IsInteger, H: Div<K, Output = Q, Remainder = R> {}
fn mul_equal<H, K, R, S>(_p: H, _q: K, _r: R, _s: S) where H: IsInteger, K: IsInteger, R: IsInteger, S: IsInteger, H: Mul<K, Output = S, Overflow = R> {}
fn less_than<H, K, R>(_p: H, _q: K) where H: IsInteger, K: IsInteger, R: Binary, H: TypedLessThan<K, Output = R>, R: AssertTrue {}
fn add_equal<H, K, S>(_a0: H, _a1: K, _a2: S) where H: IsInteger, K: IsInteger, S: IsInteger, H: Add<K, Output = S> {}
fn sub_equal<H, K, D>(_a0: H, _a1: K, _a2: D) where H: IsInteger, K: IsInteger, D: IsInteger, H: Sub<K, Output = D> {}

#[test]
fn test_set_1() {
    let h = parse_integer!(0); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(0); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
}

#[test]
fn test_set_2() {
    let h = parse_integer!(0); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(1); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_3() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(2); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_4() {
    let h = parse_integer!(0); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(3); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(3); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_5() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(4); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_6() {
    let h = parse_integer!(0); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(5); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_7() {
    let h = parse_integer!(0); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(8); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(8); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_8() {
    let h = parse_integer!(0); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(32); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(32); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_9() {
    let h = parse_integer!(0); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(95); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(95); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_10() {
    let h = parse_integer!(0); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(96); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(96); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_11() {
    let h = parse_integer!(0); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(97); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_12() {
    let h = parse_integer!(0); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(128); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(128); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_13() {
    let h = parse_integer!(0); 
    let k = parse_integer!(240); 
    let hk_sum = parse_integer!(240); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(240); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_14() {
    let h = parse_integer!(0); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(324); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(324); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_15() {
    let h = parse_integer!(0); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(330); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(330); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_16() {
    let h = parse_integer!(0); 
    let k = parse_integer!(1365); 
    let hk_sum = parse_integer!(1365); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1365); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_17() {
    let h = parse_integer!(0); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862761); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862761); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_18() {
    let h = parse_integer!(0); 
    let k = parse_integer!(33643156); 
    let hk_sum = parse_integer!(33643156); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(33643156); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_19() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483647); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483647); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_20() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483648); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483648); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_21() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4138295020); 
    let hk_sum = parse_integer!(4138295020); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4138295020); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_22() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4294967294); 
    let hk_sum = parse_integer!(4294967294); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4294967294); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_23() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4294967295); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4294967295); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    less_than(h, k);
}

#[test]
fn test_set_24() {
    let h = parse_integer!(1); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(1); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_25() {
    let h = parse_integer!(1); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(2); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_26() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(3); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_27() {
    let h = parse_integer!(1); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(4); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(3); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_28() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(3); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_29() {
    let h = parse_integer!(1); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(6); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(5); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_30() {
    let h = parse_integer!(1); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(9); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(7); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(8); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_31() {
    let h = parse_integer!(1); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(33); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(31); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(32); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_32() {
    let h = parse_integer!(1); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(96); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(94); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(95); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(95); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_33() {
    let h = parse_integer!(1); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(95); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(96); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(96); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_34() {
    let h = parse_integer!(1); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(96); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(97); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(97); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_35() {
    let h = parse_integer!(1); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(129); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(127); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(128); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(128); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_36() {
    let h = parse_integer!(1); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(325); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(323); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(324); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(324); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_37() {
    let h = parse_integer!(1); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(331); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(329); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(330); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(330); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_38() {
    let h = parse_integer!(1); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862762); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862760); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(862761); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(862761); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_39() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483648); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483646); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483647); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_40() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483649); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483647); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483648); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_41() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4294967294); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4294967293); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4294967294); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_42() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967294); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967295); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4294967295); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_43() {
    let h = parse_integer!(2); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(2); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_44() {
    let h = parse_integer!(2); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(3); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_45() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(4); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_46() {
    let h = parse_integer!(2); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(6); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_47() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(6); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(8); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_48() {
    let h = parse_integer!(2); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(7); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(3); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(10); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_49() {
    let h = parse_integer!(2); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(10); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(6); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(16); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_50() {
    let h = parse_integer!(2); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(34); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(30); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(64); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_51() {
    let h = parse_integer!(2); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(93); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(190); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(47); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_52() {
    let h = parse_integer!(2); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(94); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(192); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(48); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_53() {
    let h = parse_integer!(2); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(95); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(194); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(48); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_54() {
    let h = parse_integer!(2); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(130); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(126); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(256); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(64); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_55() {
    let h = parse_integer!(2); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(326); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(322); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(648); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(162); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_56() {
    let h = parse_integer!(2); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(332); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(328); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(660); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(165); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_57() {
    let h = parse_integer!(2); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862763); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862759); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1725522); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(431380); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_58() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483649); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483645); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_59() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483650); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483646); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741824); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_60() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967292); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_61() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967293); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_62() {
    let h = parse_integer!(3); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(3); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(3); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_63() {
    let h = parse_integer!(3); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(4); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(3); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_64() {
    let h = parse_integer!(3); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(6); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_65() {
    let h = parse_integer!(3); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(6); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_66() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(7); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(12); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_67() {
    let h = parse_integer!(3); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(8); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(15); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_68() {
    let h = parse_integer!(3); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(11); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(5); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(24); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_69() {
    let h = parse_integer!(3); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(35); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(29); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(96); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_70() {
    let h = parse_integer!(3); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(92); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(285); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(31); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_71() {
    let h = parse_integer!(3); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(93); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(288); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_72() {
    let h = parse_integer!(3); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(94); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(291); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_73() {
    let h = parse_integer!(3); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(131); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(125); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(384); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(42); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_74() {
    let h = parse_integer!(3); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(327); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(321); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(972); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(108); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_75() {
    let h = parse_integer!(3); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(333); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(327); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(990); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(110); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_76() {
    let h = parse_integer!(3); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862764); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862758); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2588283); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(287587); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_77() {
    let h = parse_integer!(3); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483650); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483644); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483645); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(715827882); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_78() {
    let h = parse_integer!(3); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483651); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483645); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(715827882); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_79() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967291); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967290); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1431655764); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_80() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967292); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967293); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1431655765); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_81() {
    let h = parse_integer!(4); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(4); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_82() {
    let h = parse_integer!(4); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(3); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_83() {
    let h = parse_integer!(4); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(6); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(8); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_84() {
    let h = parse_integer!(4); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(7); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(12); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_85() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(8); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(16); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_86() {
    let h = parse_integer!(4); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(9); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(20); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_87() {
    let h = parse_integer!(4); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(12); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(32); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_88() {
    let h = parse_integer!(4); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(36); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(28); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(128); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_89() {
    let h = parse_integer!(4); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(91); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(380); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(23); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_90() {
    let h = parse_integer!(4); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(92); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(384); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(24); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_91() {
    let h = parse_integer!(4); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(101); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(93); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(388); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(24); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_92() {
    let h = parse_integer!(4); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(132); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(124); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(512); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_93() {
    let h = parse_integer!(4); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(328); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(320); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1296); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(81); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_94() {
    let h = parse_integer!(4); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(334); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(326); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1320); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(82); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_95() {
    let h = parse_integer!(4); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862765); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862757); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(3451044); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(215690); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_96() {
    let h = parse_integer!(4); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483651); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483643); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_97() {
    let h = parse_integer!(4); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483652); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483644); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870912); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_98() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967290); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_99() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967291); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_100() {
    let h = parse_integer!(5); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(5); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(5); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_101() {
    let h = parse_integer!(5); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(6); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(5); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(5); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_102() {
    let h = parse_integer!(5); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(7); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(3); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(10); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_103() {
    let h = parse_integer!(5); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(8); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(15); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_104() {
    let h = parse_integer!(5); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(9); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(20); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_105() {
    let h = parse_integer!(5); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(10); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(25); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_106() {
    let h = parse_integer!(5); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(13); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(3); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(40); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_107() {
    let h = parse_integer!(5); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(37); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(27); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(160); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_108() {
    let h = parse_integer!(5); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(90); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(475); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_109() {
    let h = parse_integer!(5); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(101); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(91); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(480); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_110() {
    let h = parse_integer!(5); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(102); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(92); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(485); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_111() {
    let h = parse_integer!(5); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(133); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(123); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(640); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(25); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_112() {
    let h = parse_integer!(5); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(329); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(319); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1620); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(64); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_113() {
    let h = parse_integer!(5); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(335); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(325); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1650); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(66); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_114() {
    let h = parse_integer!(5); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862766); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862756); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4313805); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(172552); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_115() {
    let h = parse_integer!(5); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483652); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483642); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483643); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(429496729); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_116() {
    let h = parse_integer!(5); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483653); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483643); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(429496729); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_117() {
    let h = parse_integer!(5); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967289); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967286); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(858993458); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_118() {
    let h = parse_integer!(5); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967290); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967291); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(858993459); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_119() {
    let h = parse_integer!(7); 
    let k = parse_integer!(421); 
    let hk_sum = parse_integer!(428); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(414); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2947); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(60); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_120() {
    let h = parse_integer!(8); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(8); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(8); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_121() {
    let h = parse_integer!(8); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(9); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(7); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(8); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(8); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_122() {
    let h = parse_integer!(8); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(10); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(6); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(16); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_123() {
    let h = parse_integer!(8); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(11); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(5); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(24); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_124() {
    let h = parse_integer!(8); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(12); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(32); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_125() {
    let h = parse_integer!(8); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(13); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(3); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(40); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_126() {
    let h = parse_integer!(8); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(16); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(64); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_127() {
    let h = parse_integer!(8); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(40); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(24); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(256); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_128() {
    let h = parse_integer!(8); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(103); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(87); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(760); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(11); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_129() {
    let h = parse_integer!(8); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(104); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(88); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(768); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_130() {
    let h = parse_integer!(8); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(105); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(89); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(776); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_131() {
    let h = parse_integer!(8); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(136); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(120); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1024); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_132() {
    let h = parse_integer!(8); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(332); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(316); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2592); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(40); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_133() {
    let h = parse_integer!(8); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(338); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(322); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2640); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(41); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_134() {
    let h = parse_integer!(8); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862769); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862753); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(6902088); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(107845); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_135() {
    let h = parse_integer!(8); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483655); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483639); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(268435455); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_136() {
    let h = parse_integer!(8); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483656); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483640); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(268435456); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_137() {
    let h = parse_integer!(8); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967286); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967280); 
    let hk_over = parse_integer!(7); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(6); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_138() {
    let h = parse_integer!(8); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967287); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(7); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_139() {
    let h = parse_integer!(9); 
    let k = parse_integer!(328); 
    let hk_sum = parse_integer!(337); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(319); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2952); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(9); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(36); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_140() {
    let h = parse_integer!(14); 
    let k = parse_integer!(708827196); 
    let hk_sum = parse_integer!(708827210); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(708827182); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1333646152); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(14); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(50630514); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_141() {
    let h = parse_integer!(15); 
    let k = parse_integer!(29266833); 
    let hk_sum = parse_integer!(29266848); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(29266818); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(439002495); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(15); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1951122); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_142() {
    let h = parse_integer!(24); 
    let k = parse_integer!(415); 
    let hk_sum = parse_integer!(439); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(391); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9960); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(24); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(17); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_143() {
    let h = parse_integer!(32); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(32); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(32); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_144() {
    let h = parse_integer!(32); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(33); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(31); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(32); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(32); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_145() {
    let h = parse_integer!(32); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(34); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(30); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(64); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(16); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_146() {
    let h = parse_integer!(32); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(35); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(29); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(96); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(10); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_147() {
    let h = parse_integer!(32); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(36); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(28); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(128); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(8); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_148() {
    let h = parse_integer!(32); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(37); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(27); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(160); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_149() {
    let h = parse_integer!(32); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(40); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(24); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(256); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_150() {
    let h = parse_integer!(32); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(64); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1024); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_151() {
    let h = parse_integer!(32); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(127); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(63); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(3040); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_152() {
    let h = parse_integer!(32); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(128); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(64); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(3072); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_153() {
    let h = parse_integer!(32); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(129); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(65); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(3104); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_154() {
    let h = parse_integer!(32); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(160); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(96); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4096); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_155() {
    let h = parse_integer!(32); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(356); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(292); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(10368); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_156() {
    let h = parse_integer!(32); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(362); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(298); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(10560); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(10); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_157() {
    let h = parse_integer!(32); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862793); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862729); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(27608352); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(26961); 
    let kh_rem = parse_integer!(9); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_158() {
    let h = parse_integer!(32); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483679); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483615); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967264); 
    let hk_over = parse_integer!(15); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(67108863); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_159() {
    let h = parse_integer!(32); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483680); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483616); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(16); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(67108864); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_160() {
    let h = parse_integer!(32); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967262); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967232); 
    let hk_over = parse_integer!(31); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(134217727); 
    let kh_rem = parse_integer!(30); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_161() {
    let h = parse_integer!(32); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967263); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967264); 
    let hk_over = parse_integer!(31); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(134217727); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_162() {
    let h = parse_integer!(58); 
    let k = parse_integer!(2683766); 
    let hk_sum = parse_integer!(2683824); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2683708); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(155658428); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(58); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(46271); 
    let kh_rem = parse_integer!(48); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_163() {
    let h = parse_integer!(95); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(95); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(95); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_164() {
    let h = parse_integer!(95); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(96); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(94); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(95); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(95); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_165() {
    let h = parse_integer!(95); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(93); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(190); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(47); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_166() {
    let h = parse_integer!(95); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(92); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(285); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(31); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_167() {
    let h = parse_integer!(95); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(91); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(380); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(23); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_168() {
    let h = parse_integer!(95); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(90); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(475); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(19); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_169() {
    let h = parse_integer!(95); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(103); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(87); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(760); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(11); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_170() {
    let h = parse_integer!(95); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(127); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(63); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(3040); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(31); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_171() {
    let h = parse_integer!(95); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(190); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9025); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_172() {
    let h = parse_integer!(95); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(191); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9120); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_173() {
    let h = parse_integer!(95); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(192); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9215); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_174() {
    let h = parse_integer!(95); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(223); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(33); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(12160); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_175() {
    let h = parse_integer!(95); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(419); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(229); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(30780); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(39); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_176() {
    let h = parse_integer!(95); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(425); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(235); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(31350); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(45); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_177() {
    let h = parse_integer!(95); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862856); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862666); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(81962295); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9081); 
    let kh_rem = parse_integer!(66); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_178() {
    let h = parse_integer!(95); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483742); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483552); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483553); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22605091); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_179() {
    let h = parse_integer!(95); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483743); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483553); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22605091); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_180() {
    let h = parse_integer!(95); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967199); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967106); 
    let hk_over = parse_integer!(94); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(45210182); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_181() {
    let h = parse_integer!(95); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967200); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967201); 
    let hk_over = parse_integer!(94); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(45210182); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_182() {
    let h = parse_integer!(96); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(96); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(96); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_183() {
    let h = parse_integer!(96); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(95); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(96); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(96); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_184() {
    let h = parse_integer!(96); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(94); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(192); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(48); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_185() {
    let h = parse_integer!(96); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(93); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(288); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(32); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_186() {
    let h = parse_integer!(96); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(92); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(384); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(24); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_187() {
    let h = parse_integer!(96); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(101); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(91); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(480); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(19); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_188() {
    let h = parse_integer!(96); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(104); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(88); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(768); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(12); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_189() {
    let h = parse_integer!(96); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(128); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(64); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(3072); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_190() {
    let h = parse_integer!(96); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(191); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(9120); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_191() {
    let h = parse_integer!(96); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(192); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9216); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_192() {
    let h = parse_integer!(96); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(193); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9312); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_193() {
    let h = parse_integer!(96); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(224); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(32); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(12288); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_194() {
    let h = parse_integer!(96); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(420); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(228); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(31104); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(36); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_195() {
    let h = parse_integer!(96); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(426); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(234); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(31680); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(42); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_196() {
    let h = parse_integer!(96); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862857); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862665); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(82825056); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8987); 
    let kh_rem = parse_integer!(9); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_197() {
    let h = parse_integer!(96); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483743); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483551); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967200); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22369621); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_198() {
    let h = parse_integer!(96); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483744); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483552); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22369621); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_199() {
    let h = parse_integer!(96); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967198); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967104); 
    let hk_over = parse_integer!(95); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44739242); 
    let kh_rem = parse_integer!(62); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_200() {
    let h = parse_integer!(96); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967199); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967200); 
    let hk_over = parse_integer!(95); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44739242); 
    let kh_rem = parse_integer!(63); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_201() {
    let h = parse_integer!(97); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(97); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(97); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_202() {
    let h = parse_integer!(97); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(98); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(96); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(97); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(97); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_203() {
    let h = parse_integer!(97); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(99); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(95); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(194); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(48); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_204() {
    let h = parse_integer!(97); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(100); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(94); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(291); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(32); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_205() {
    let h = parse_integer!(97); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(101); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(93); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(388); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(24); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_206() {
    let h = parse_integer!(97); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(102); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(92); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(485); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(19); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_207() {
    let h = parse_integer!(97); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(105); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(89); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(776); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(12); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_208() {
    let h = parse_integer!(97); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(129); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(65); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(3104); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_209() {
    let h = parse_integer!(97); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(192); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(9215); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_210() {
    let h = parse_integer!(97); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(193); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(9312); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_211() {
    let h = parse_integer!(97); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(194); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(9409); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_212() {
    let h = parse_integer!(97); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(225); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(31); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(12416); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_213() {
    let h = parse_integer!(97); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(421); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(227); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(31428); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_214() {
    let h = parse_integer!(97); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(427); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(233); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(32010); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(39); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_215() {
    let h = parse_integer!(97); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862858); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862664); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(83687817); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8894); 
    let kh_rem = parse_integer!(43); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_216() {
    let h = parse_integer!(97); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483744); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483550); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483551); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22139006); 
    let kh_rem = parse_integer!(65); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_217() {
    let h = parse_integer!(97); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483745); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483551); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22139006); 
    let kh_rem = parse_integer!(66); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_218() {
    let h = parse_integer!(97); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967197); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967102); 
    let hk_over = parse_integer!(96); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44278013); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_219() {
    let h = parse_integer!(97); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967198); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967199); 
    let hk_over = parse_integer!(96); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44278013); 
    let kh_rem = parse_integer!(34); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_220() {
    let h = parse_integer!(128); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(128); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(128); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_221() {
    let h = parse_integer!(128); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(129); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(127); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(128); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(128); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_222() {
    let h = parse_integer!(128); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(130); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(126); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(256); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(64); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_223() {
    let h = parse_integer!(128); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(131); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(125); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(384); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(42); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_224() {
    let h = parse_integer!(128); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(132); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(124); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(512); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(32); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_225() {
    let h = parse_integer!(128); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(133); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(123); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(640); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(25); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_226() {
    let h = parse_integer!(128); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(136); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(120); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1024); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(16); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_227() {
    let h = parse_integer!(128); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(160); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(96); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4096); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_228() {
    let h = parse_integer!(128); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(223); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(33); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(12160); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(33); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_229() {
    let h = parse_integer!(128); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(224); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(32); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(12288); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_230() {
    let h = parse_integer!(128); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(225); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(31); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(12416); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(31); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_231() {
    let h = parse_integer!(128); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(256); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(16384); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_232() {
    let h = parse_integer!(128); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(452); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(196); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(41472); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(68); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_233() {
    let h = parse_integer!(128); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(458); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(202); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(42240); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(74); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_234() {
    let h = parse_integer!(128); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(862889); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862633); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(110433408); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6740); 
    let kh_rem = parse_integer!(41); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_235() {
    let h = parse_integer!(128); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483775); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483519); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967168); 
    let hk_over = parse_integer!(63); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16777215); 
    let kh_rem = parse_integer!(127); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_236() {
    let h = parse_integer!(128); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483776); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483520); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(64); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16777216); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_237() {
    let h = parse_integer!(128); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294967166); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967040); 
    let hk_over = parse_integer!(127); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(33554431); 
    let kh_rem = parse_integer!(126); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_238() {
    let h = parse_integer!(128); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294967167); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294967168); 
    let hk_over = parse_integer!(127); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(33554431); 
    let kh_rem = parse_integer!(127); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_239() {
    let h = parse_integer!(240); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(240); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(240); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_240() {
    let h = parse_integer!(324); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(324); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(324); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_241() {
    let h = parse_integer!(324); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(325); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(323); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(324); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(324); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_242() {
    let h = parse_integer!(324); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(326); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(322); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(648); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(162); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_243() {
    let h = parse_integer!(324); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(327); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(321); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(972); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(108); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_244() {
    let h = parse_integer!(324); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(328); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(320); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1296); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(81); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_245() {
    let h = parse_integer!(324); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(329); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(319); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1620); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(64); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_246() {
    let h = parse_integer!(324); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(332); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(316); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2592); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(40); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_247() {
    let h = parse_integer!(324); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(356); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(292); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(10368); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(10); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_248() {
    let h = parse_integer!(324); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(419); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(229); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(30780); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(39); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_249() {
    let h = parse_integer!(324); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(420); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(228); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(31104); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(36); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_250() {
    let h = parse_integer!(324); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(421); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(227); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(31428); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(33); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_251() {
    let h = parse_integer!(324); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(452); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(196); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(41472); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(68); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_252() {
    let h = parse_integer!(324); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(648); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(104976); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_253() {
    let h = parse_integer!(324); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(654); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(6); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(106920); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(6); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_254() {
    let h = parse_integer!(324); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(863085); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862437); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(279534564); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2662); 
    let kh_rem = parse_integer!(273); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_255() {
    let h = parse_integer!(324); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483971); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483323); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966972); 
    let hk_over = parse_integer!(161); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6628035); 
    let kh_rem = parse_integer!(307); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_256() {
    let h = parse_integer!(324); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483972); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483324); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(162); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6628035); 
    let kh_rem = parse_integer!(308); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_257() {
    let h = parse_integer!(324); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294966970); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966648); 
    let hk_over = parse_integer!(323); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13256071); 
    let kh_rem = parse_integer!(290); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_258() {
    let h = parse_integer!(324); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294966971); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966972); 
    let hk_over = parse_integer!(323); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13256071); 
    let kh_rem = parse_integer!(291); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_259() {
    let h = parse_integer!(328); 
    let k = parse_integer!(9); 
    let hk_sum = parse_integer!(337); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(319); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2952); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(36); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(9); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_260() {
    let h = parse_integer!(330); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(330); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(330); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_261() {
    let h = parse_integer!(330); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(331); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(329); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(330); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(330); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_262() {
    let h = parse_integer!(330); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(332); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(328); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(660); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(165); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_263() {
    let h = parse_integer!(330); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(333); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(327); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(990); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(110); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_264() {
    let h = parse_integer!(330); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(334); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(326); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1320); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(82); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_265() {
    let h = parse_integer!(330); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(335); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(325); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1650); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(66); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_266() {
    let h = parse_integer!(330); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(338); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(322); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2640); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(41); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_267() {
    let h = parse_integer!(330); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(362); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(298); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(10560); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(10); 
    let hk_rem = parse_integer!(10); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_268() {
    let h = parse_integer!(330); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(425); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(235); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(31350); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(45); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_269() {
    let h = parse_integer!(330); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(426); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(234); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(31680); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(42); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_270() {
    let h = parse_integer!(330); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(427); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(233); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(32010); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(3); 
    let hk_rem = parse_integer!(39); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_271() {
    let h = parse_integer!(330); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(458); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(202); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(42240); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(74); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_272() {
    let h = parse_integer!(330); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(654); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(6); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(106920); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(6); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_273() {
    let h = parse_integer!(330); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(660); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(108900); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_274() {
    let h = parse_integer!(330); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(863091); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(862431); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(284711130); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2614); 
    let kh_rem = parse_integer!(141); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_275() {
    let h = parse_integer!(330); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2147483977); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483317); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966966); 
    let hk_over = parse_integer!(164); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6507526); 
    let kh_rem = parse_integer!(67); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_276() {
    let h = parse_integer!(330); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2147483978); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2147483318); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(165); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6507526); 
    let kh_rem = parse_integer!(68); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_277() {
    let h = parse_integer!(330); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294966964); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966636); 
    let hk_over = parse_integer!(329); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13015052); 
    let kh_rem = parse_integer!(134); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_278() {
    let h = parse_integer!(330); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294966965); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294966966); 
    let hk_over = parse_integer!(329); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13015052); 
    let kh_rem = parse_integer!(135); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_279() {
    let h = parse_integer!(415); 
    let k = parse_integer!(24); 
    let hk_sum = parse_integer!(439); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(391); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(9960); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(17); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(24); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_280() {
    let h = parse_integer!(421); 
    let k = parse_integer!(7); 
    let hk_sum = parse_integer!(428); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(414); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2947); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(60); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_281() {
    let h = parse_integer!(1365); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(1365); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1365); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_282() {
    let h = parse_integer!(1552); 
    let k = parse_integer!(14323); 
    let hk_sum = parse_integer!(15875); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(12771); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(22229296); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1552); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9); 
    let kh_rem = parse_integer!(355); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_283() {
    let h = parse_integer!(14323); 
    let k = parse_integer!(1552); 
    let hk_sum = parse_integer!(15875); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(12771); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(22229296); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(9); 
    let hk_rem = parse_integer!(355); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1552); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_284() {
    let h = parse_integer!(94362); 
    let k = parse_integer!(1157197446); 
    let hk_sum = parse_integer!(1157291808); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1157103084); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(216865948); 
    let hk_over = parse_integer!(25424); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(94362); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12263); 
    let kh_rem = parse_integer!(36240); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_285() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(862761); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862761); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_286() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(862762); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862760); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(862761); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(862761); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_287() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(862763); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862759); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1725522); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(431380); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_288() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(862764); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862758); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2588283); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(287587); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_289() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(862765); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862757); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(3451044); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(215690); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_290() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(862766); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862756); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4313805); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(172552); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_291() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(862769); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862753); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(6902088); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(107845); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_292() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(862793); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862729); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(27608352); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(26961); 
    let hk_rem = parse_integer!(9); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_293() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(862856); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862666); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(81962295); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(9081); 
    let hk_rem = parse_integer!(66); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_294() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(862857); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862665); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(82825056); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(8987); 
    let hk_rem = parse_integer!(9); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_295() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(862858); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862664); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(83687817); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(8894); 
    let hk_rem = parse_integer!(43); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_296() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(862889); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862633); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(110433408); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6740); 
    let hk_rem = parse_integer!(41); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_297() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(863085); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862437); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(279534564); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2662); 
    let hk_rem = parse_integer!(273); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_298() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(863091); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(862431); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(284711130); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2614); 
    let hk_rem = parse_integer!(141); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(330); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_299() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(1725522); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1327200913); 
    let hk_over = parse_integer!(173); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_300() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(2148346408); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2146620886); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2146620887); 
    let hk_over = parse_integer!(431380); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2489); 
    let kh_rem = parse_integer!(71518); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_301() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(2148346409); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2146620887); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(431380); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2489); 
    let kh_rem = parse_integer!(71519); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_302() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(4294104533); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4293241774); 
    let hk_over = parse_integer!(862760); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4978); 
    let kh_rem = parse_integer!(143036); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_303() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(4294104534); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4294104535); 
    let hk_over = parse_integer!(862760); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4978); 
    let kh_rem = parse_integer!(143037); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_304() {
    let h = parse_integer!(2103208); 
    let k = parse_integer!(2473114); 
    let hk_sum = parse_integer!(4576322); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(369906); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(267754256); 
    let hk_over = parse_integer!(1211); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2103208); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(369906); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_305() {
    let h = parse_integer!(2473114); 
    let k = parse_integer!(2103208); 
    let hk_sum = parse_integer!(4576322); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(369906); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(267754256); 
    let hk_over = parse_integer!(1211); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(369906); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2103208); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_306() {
    let h = parse_integer!(2683766); 
    let k = parse_integer!(58); 
    let hk_sum = parse_integer!(2683824); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2683708); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(155658428); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(46271); 
    let hk_rem = parse_integer!(48); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(58); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_307() {
    let h = parse_integer!(3373669); 
    let k = parse_integer!(8042894); 
    let hk_sum = parse_integer!(11416563); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(4669225); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2753749254); 
    let hk_over = parse_integer!(6317); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3373669); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1295556); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_308() {
    let h = parse_integer!(8042894); 
    let k = parse_integer!(3373669); 
    let hk_sum = parse_integer!(11416563); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4669225); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2753749254); 
    let hk_over = parse_integer!(6317); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(1295556); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3373669); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_309() {
    let h = parse_integer!(29266833); 
    let k = parse_integer!(15); 
    let hk_sum = parse_integer!(29266848); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(29266818); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(439002495); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1951122); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(15); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_310() {
    let h = parse_integer!(33643156); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(33643156); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(33643156); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_311() {
    let h = parse_integer!(46753796); 
    let k = parse_integer!(2293448598); 
    let hk_sum = parse_integer!(2340202394); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(2246694802); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(224915032); 
    let hk_over = parse_integer!(24965831); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(46753796); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(49); 
    let kh_rem = parse_integer!(2512594); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_312() {
    let h = parse_integer!(708827196); 
    let k = parse_integer!(14); 
    let hk_sum = parse_integer!(708827210); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(708827182); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(1333646152); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(50630514); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(14); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_313() {
    let h = parse_integer!(1157197446); 
    let k = parse_integer!(94362); 
    let hk_sum = parse_integer!(1157291808); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1157103084); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(216865948); 
    let hk_over = parse_integer!(25424); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(12263); 
    let hk_rem = parse_integer!(36240); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(94362); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_314() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(2147483647); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483647); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_315() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(2147483648); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483646); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483647); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2147483647); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_316() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(2147483649); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483645); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1073741823); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_317() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(2147483650); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483644); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483645); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(715827882); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_318() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(2147483651); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483643); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(536870911); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_319() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(2147483652); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483642); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483643); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(429496729); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_320() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(2147483655); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483639); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(268435455); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_321() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(2147483679); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483615); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967264); 
    let hk_over = parse_integer!(15); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(67108863); 
    let hk_rem = parse_integer!(31); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_322() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(2147483742); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483552); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483553); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22605091); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_323() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(2147483743); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483551); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967200); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22369621); 
    let hk_rem = parse_integer!(31); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_324() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(2147483744); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483550); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483551); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22139006); 
    let hk_rem = parse_integer!(65); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_325() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(2147483775); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483519); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967168); 
    let hk_over = parse_integer!(63); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(16777215); 
    let hk_rem = parse_integer!(127); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_326() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(2147483971); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483323); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966972); 
    let hk_over = parse_integer!(161); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6628035); 
    let hk_rem = parse_integer!(307); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_327() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(2147483977); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483317); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966966); 
    let hk_over = parse_integer!(164); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6507526); 
    let hk_rem = parse_integer!(67); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(330); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_328() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(2148346408); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2146620886); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2146620887); 
    let hk_over = parse_integer!(431380); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2489); 
    let hk_rem = parse_integer!(71518); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(862761); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_329() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(4294967294); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1); 
    let hk_over = parse_integer!(1073741823); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_330() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(2147483648); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(1073741823); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_331() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(2147483647); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(2147483646); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_332() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(2147483648); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483649); 
    let hk_over = parse_integer!(2147483646); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_333() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(2147483648); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483648); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_334() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(2147483649); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483647); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2147483648); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_335() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(2); 
    let hk_sum = parse_integer!(2147483650); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483646); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1073741824); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_336() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(3); 
    let hk_sum = parse_integer!(2147483651); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483645); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(715827882); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_337() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(4); 
    let hk_sum = parse_integer!(2147483652); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483644); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(536870912); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_338() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(5); 
    let hk_sum = parse_integer!(2147483653); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483643); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(429496729); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_339() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(8); 
    let hk_sum = parse_integer!(2147483656); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483640); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(268435456); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_340() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(32); 
    let hk_sum = parse_integer!(2147483680); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483616); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(16); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(67108864); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_341() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(95); 
    let hk_sum = parse_integer!(2147483743); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483553); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(47); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22605091); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_342() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(96); 
    let hk_sum = parse_integer!(2147483744); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483552); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22369621); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_343() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(97); 
    let hk_sum = parse_integer!(2147483745); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483551); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(48); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(22139006); 
    let hk_rem = parse_integer!(66); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_344() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(128); 
    let hk_sum = parse_integer!(2147483776); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483520); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(64); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(16777216); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_345() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(324); 
    let hk_sum = parse_integer!(2147483972); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483324); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(162); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6628035); 
    let hk_rem = parse_integer!(308); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_346() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(330); 
    let hk_sum = parse_integer!(2147483978); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2147483318); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(165); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(6507526); 
    let hk_rem = parse_integer!(68); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(330); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_347() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(862761); 
    let hk_sum = parse_integer!(2148346409); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2146620887); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(431380); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2489); 
    let hk_rem = parse_integer!(71519); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(862761); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_348() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(2147483647); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(1073741823); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2147483647); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_349() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(2147483648); 
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(1073741824); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_350() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(4294967294); 
    let kh_diff = parse_integer!(2147483646); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(2147483647); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483648); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2147483646); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_351() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(2147483647); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(2147483647); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483648); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2147483647); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_352() {
    let h = parse_integer!(2293448598); 
    let k = parse_integer!(46753796); 
    let hk_sum = parse_integer!(2340202394); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(2246694802); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(224915032); 
    let hk_over = parse_integer!(24965831); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(49); 
    let hk_rem = parse_integer!(2512594); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(46753796); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_353() {
    let h = parse_integer!(4138295020); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(4138295020); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4138295020); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_354() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(4294967294); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4294967294); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_355() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(1); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4294967293); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4294967294); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_356() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(2); 
    let hk_diff = parse_integer!(4294967292); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2147483647); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_357() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(3); 
    let hk_diff = parse_integer!(4294967291); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967290); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1431655764); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_358() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(4); 
    let hk_diff = parse_integer!(4294967290); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1073741823); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_359() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(5); 
    let hk_diff = parse_integer!(4294967289); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967286); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(858993458); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_360() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(8); 
    let hk_diff = parse_integer!(4294967286); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967280); 
    let hk_over = parse_integer!(7); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(536870911); 
    let hk_rem = parse_integer!(6); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_361() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(32); 
    let hk_diff = parse_integer!(4294967262); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967232); 
    let hk_over = parse_integer!(31); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(134217727); 
    let hk_rem = parse_integer!(30); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_362() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(95); 
    let hk_diff = parse_integer!(4294967199); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967106); 
    let hk_over = parse_integer!(94); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(45210182); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_363() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(96); 
    let hk_diff = parse_integer!(4294967198); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967104); 
    let hk_over = parse_integer!(95); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(44739242); 
    let hk_rem = parse_integer!(62); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_364() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(97); 
    let hk_diff = parse_integer!(4294967197); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967102); 
    let hk_over = parse_integer!(96); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(44278013); 
    let hk_rem = parse_integer!(33); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_365() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(128); 
    let hk_diff = parse_integer!(4294967166); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967040); 
    let hk_over = parse_integer!(127); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(33554431); 
    let hk_rem = parse_integer!(126); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_366() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(324); 
    let hk_diff = parse_integer!(4294966970); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966648); 
    let hk_over = parse_integer!(323); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(13256071); 
    let hk_rem = parse_integer!(290); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_367() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(330); 
    let hk_diff = parse_integer!(4294966964); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966636); 
    let hk_over = parse_integer!(329); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(13015052); 
    let hk_rem = parse_integer!(134); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(330); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_368() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(862761); 
    let hk_diff = parse_integer!(4294104533); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4293241774); 
    let hk_over = parse_integer!(862760); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4978); 
    let hk_rem = parse_integer!(143036); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(862761); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_369() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(2147483647); 
    let hk_diff = parse_integer!(2147483647); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(2147483646); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2147483647); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_370() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(2147483648); 
    let hk_diff = parse_integer!(2147483646); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(2147483647); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(2147483646); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2147483648); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_371() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(4294967294); 
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(4); 
    let hk_over = parse_integer!(4294967292); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_372() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(4294967295); 
    let kh_diff = parse_integer!(1); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(4294967293); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4294967294); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(h, k);
}

#[test]
fn test_set_373() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(0); 
    let hk_sum = parse_integer!(4294967295); 
    add_equal(h, k, hk_sum);
    let hk_diff = parse_integer!(4294967295); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(0); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_374() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(1); 
    let hk_diff = parse_integer!(4294967294); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967295); 
    let hk_over = parse_integer!(0); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4294967295); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_375() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(2); 
    let hk_diff = parse_integer!(4294967293); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967294); 
    let hk_over = parse_integer!(1); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2147483647); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_376() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(3); 
    let hk_diff = parse_integer!(4294967292); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967293); 
    let hk_over = parse_integer!(2); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1431655765); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_377() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(4); 
    let hk_diff = parse_integer!(4294967291); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967292); 
    let hk_over = parse_integer!(3); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1073741823); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_378() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(5); 
    let hk_diff = parse_integer!(4294967290); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967291); 
    let hk_over = parse_integer!(4); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(858993459); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_379() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(8); 
    let hk_diff = parse_integer!(4294967287); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967288); 
    let hk_over = parse_integer!(7); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(536870911); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(8); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_380() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(32); 
    let hk_diff = parse_integer!(4294967263); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967264); 
    let hk_over = parse_integer!(31); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(134217727); 
    let hk_rem = parse_integer!(31); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_381() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(95); 
    let hk_diff = parse_integer!(4294967200); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967201); 
    let hk_over = parse_integer!(94); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(45210182); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(95); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_382() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(96); 
    let hk_diff = parse_integer!(4294967199); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967200); 
    let hk_over = parse_integer!(95); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(44739242); 
    let hk_rem = parse_integer!(63); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(96); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_383() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(97); 
    let hk_diff = parse_integer!(4294967198); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967199); 
    let hk_over = parse_integer!(96); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(44278013); 
    let hk_rem = parse_integer!(34); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(97); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_384() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(128); 
    let hk_diff = parse_integer!(4294967167); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294967168); 
    let hk_over = parse_integer!(127); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(33554431); 
    let hk_rem = parse_integer!(127); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(128); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_385() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(324); 
    let hk_diff = parse_integer!(4294966971); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966972); 
    let hk_over = parse_integer!(323); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(13256071); 
    let hk_rem = parse_integer!(291); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(324); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_386() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(330); 
    let hk_diff = parse_integer!(4294966965); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294966966); 
    let hk_over = parse_integer!(329); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(13015052); 
    let hk_rem = parse_integer!(135); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(330); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_387() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(862761); 
    let hk_diff = parse_integer!(4294104534); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(4294104535); 
    let hk_over = parse_integer!(862760); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(4978); 
    let hk_rem = parse_integer!(143037); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(862761); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_388() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(2147483647); 
    let hk_diff = parse_integer!(2147483648); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483649); 
    let hk_over = parse_integer!(2147483646); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(2); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2147483647); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_389() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(2147483648); 
    let hk_diff = parse_integer!(2147483647); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2147483648); 
    let hk_over = parse_integer!(2147483647); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(2147483648); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_390() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(4294967294); 
    let hk_diff = parse_integer!(1); 
    sub_equal(h, k, hk_diff);
    let hk_prod = parse_integer!(2); 
    let hk_over = parse_integer!(4294967293); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(0); 
    let kh_rem = parse_integer!(4294967294); 
    div_equal(k, h, kh_quot, kh_rem);
    less_than(k, h);
}

#[test]
fn test_set_391() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(4294967295); 
    let hk_diff = parse_integer!(0); 
    sub_equal(h, k, hk_diff);
    let kh_diff = parse_integer!(0); 
    sub_equal(k, h, kh_diff);
    let hk_prod = parse_integer!(1); 
    let hk_over = parse_integer!(4294967294); 
    mul_equal(h, k, hk_over, hk_prod);
    mul_equal(k, h, hk_over, hk_prod);
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}
