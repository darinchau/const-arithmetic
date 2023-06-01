//! If this compiles then all tests passed

// This is generated using the following python code
// # Test cases
// import random
// random.seed(420)

// # Some big ones we wanted to test
// nos = (0, 1, 2, 3, 4, 5, 8, 32, 95, 96, 97, 128, 324, 330, 862761, 2147483647, 2147483648, 4294967294, 4294967295)
// sets_to_test = set()

// for r in nos:
//     for s in nos:
//         if (r, s) in sets_to_test:
//             continue
//         if (s, r) in sets_to_test:
//             continue
//         if r > s:
//             sets_to_test.add((s, r))
//         else:
//             sets_to_test.add((r, s))
// # Generate some random ones
// i = 0
// while i < 110:
//     r = int(2 ** (random.random() * 32)) - 1
//     s = int(2 ** (random.random() * 32)) - 1
//     if r > s:
//         r, s = s, r
//     if (r, s) in sets_to_test:
//         continue
//     sets_to_test.add((r, s))
//     i += 1

// for i, (h, k) in enumerate(sorted(sets_to_test)):
//     print("#[test]")
//     print(f"fn test_set_{i+1}() {{")
//     print(f"    let h = parse_integer!({h}); ")
//     print(f"    let k = parse_integer!({k}); ")

//     if h + k < 2**32:
//         print(f"    let hk_sum = parse_integer!({h + k}); ")
//         print(f"    add_equal(h, k, hk_sum);")
    
//     if h >= k:
//         print(f"    let hk_diff = parse_integer!({h - k}); ")
//         print(f"    sub_equal(h, k, hk_diff);")
//     if k >= h:
//         print(f"    let kh_diff = parse_integer!({k - h}); ")
//         print(f"    sub_equal(k, h, kh_diff);")
    
//     print(f"    let hk_prod = parse_integer!({(h*k)%2**32}); ")
//     print(f"    let hk_over = parse_integer!({(h*k)//2**32}); ")
//     print(f"    mul_equal(h, k, hk_over, hk_prod);")
//     print(f"    mul_equal(k, h, hk_over, hk_prod);")

//     if k > 0:
//         print(f"    let hk_quot = parse_integer!({h//k}); ")
//         print(f"    let hk_rem = parse_integer!({h%k}); ")
//         print(f"    div_equal(h, k, hk_quot, hk_rem);")

//     if h > 0:
//         print(f"    let kh_quot = parse_integer!({k//h}); ")
//         print(f"    let kh_rem = parse_integer!({k%h}); ")
//         print(f"    div_equal(k, h, kh_quot, kh_rem);")

//     if h < k:
//         print(f"    less_than(h, k);")
//     if k < h:
//         print(f"    less_than(k, h);")

//     print("}")
//     print()
use super::*;

fn div_equal<H, K, Q, R>(_a: H, _b: K, _c: Q, _d: R) where H: IsInteger, K: IsInteger, Q: IsInteger, R: IsInteger, H: TypedDiv<K, Output = Q, Remainder = R> {}
fn mul_equal<H, K, R, S>(_p: H, _q: K, _r: R, _s: S) where H: IsInteger, K: IsInteger, R: IsInteger, S: IsInteger, H: TypedMul<K, Output = S, Overflow = R> {}
fn less_than<H, K, R>(_p: H, _q: K) where H: IsInteger, K: IsInteger, R: Binary, H: TypedLessThan<K, Output = R>, R: AssertTrue {}
fn add_equal<H, K, S>(_a0: H, _a1: K, _a2: S) where H: IsInteger, K: IsInteger, S: IsInteger, H: TypedAdd<K, Output = S> {}
fn sub_equal<H, K, D>(_a0: H, _a1: K, _a2: D) where H: IsInteger, K: IsInteger, D: IsInteger, H: TypedSub<K, Output = D> {}

#[test]
fn test_set_1() {
    let h = parse_integer!(0); 
    let k = parse_integer!(0); 
}

#[test]
fn test_set_2() {
    let h = parse_integer!(0); 
    let k = parse_integer!(1); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0);
    div_equal(h, k, hk_quot, hk_rem);
} 

#[test]
fn test_set_3() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_4() {
    let h = parse_integer!(0); 
    let k = parse_integer!(3); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_5() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_6() {
    let h = parse_integer!(0); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_7() {
    let h = parse_integer!(0); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_8() {
    let h = parse_integer!(0); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_9() {
    let h = parse_integer!(0); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_10() {
    let h = parse_integer!(0); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_11() {
    let h = parse_integer!(0); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_12() {
    let h = parse_integer!(0); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_13() {
    let h = parse_integer!(0); 
    let k = parse_integer!(240); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_14() {
    let h = parse_integer!(0); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_15() {
    let h = parse_integer!(0); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_16() {
    let h = parse_integer!(0); 
    let k = parse_integer!(353); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_17() {
    let h = parse_integer!(0); 
    let k = parse_integer!(1365); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_18() {
    let h = parse_integer!(0); 
    let k = parse_integer!(11279); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_19() {
    let h = parse_integer!(0); 
    let k = parse_integer!(32251); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_20() {
    let h = parse_integer!(0); 
    let k = parse_integer!(240863); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_21() {
    let h = parse_integer!(0); 
    let k = parse_integer!(490105); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_22() {
    let h = parse_integer!(0); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_23() {
    let h = parse_integer!(0); 
    let k = parse_integer!(33643156); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_24() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_25() {
    let h = parse_integer!(0); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_26() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4138295020); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_27() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_28() {
    let h = parse_integer!(0); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
}

#[test]
fn test_set_29() {
    let h = parse_integer!(1); 
    let k = parse_integer!(1); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_30() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_31() {
    let h = parse_integer!(1); 
    let k = parse_integer!(3); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_32() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_33() {
    let h = parse_integer!(1); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_34() {
    let h = parse_integer!(1); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_35() {
    let h = parse_integer!(1); 
    let k = parse_integer!(15); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(15); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_36() {
    let h = parse_integer!(1); 
    let k = parse_integer!(26); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(26); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}



#[test]
fn test_set_38() {
    let h = parse_integer!(1); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(95); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_39() {
    let h = parse_integer!(1); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(96); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_40() {
    let h = parse_integer!(1); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(97); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_41() {
    let h = parse_integer!(1); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(128); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_42() {
    let h = parse_integer!(1); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(324); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_43() {
    let h = parse_integer!(1); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(330); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_44() {
    let h = parse_integer!(1); 
    let k = parse_integer!(1158); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1158); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_45() {
    let h = parse_integer!(1); 
    let k = parse_integer!(6714); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6714); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_46() {
    let h = parse_integer!(1); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(862761); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_47() {
    let h = parse_integer!(1); 
    let k = parse_integer!(3486687); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3486687); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_48() {
    let h = parse_integer!(1); 
    let k = parse_integer!(29119201); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(29119201); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_49() {
    let h = parse_integer!(1); 
    let k = parse_integer!(1289219388); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1289219388); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_50() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_51() {
    let h = parse_integer!(1); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483648); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_52() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4294967294); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_53() {
    let h = parse_integer!(1); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4294967295); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_54() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_55() {
    let h = parse_integer!(2); 
    let k = parse_integer!(3); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_56() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_57() {
    let h = parse_integer!(2); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_58() {
    let h = parse_integer!(2); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_59() {
    let h = parse_integer!(2); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_60() {
    let h = parse_integer!(2); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(47); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_61() {
    let h = parse_integer!(2); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(48); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_62() {
    let h = parse_integer!(2); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(48); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_63() {
    let h = parse_integer!(2); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(64); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_64() {
    let h = parse_integer!(2); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(162); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_65() {
    let h = parse_integer!(2); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(165); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_66() {
    let h = parse_integer!(2); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(431380); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_67() {
    let h = parse_integer!(2); 
    let k = parse_integer!(290135313); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(145067656); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_68() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_69() {
    let h = parse_integer!(2); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741824); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_70() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_71() {
    let h = parse_integer!(2); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2147483647); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_72() {
    let h = parse_integer!(3); 
    let k = parse_integer!(3); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_73() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_74() {
    let h = parse_integer!(3); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_75() {
    let h = parse_integer!(3); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_76() {
    let h = parse_integer!(3); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_77() {
    let h = parse_integer!(3); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(31); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_78() {
    let h = parse_integer!(3); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_79() {
    let h = parse_integer!(3); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_80() {
    let h = parse_integer!(3); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(42); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_81() {
    let h = parse_integer!(3); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(108); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_82() {
    let h = parse_integer!(3); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(110); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_83() {
    let h = parse_integer!(3); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(287587); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_84() {
    let h = parse_integer!(3); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(715827882); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_85() {
    let h = parse_integer!(3); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(715827882); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_86() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1431655764); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_87() {
    let h = parse_integer!(3); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1431655765); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_88() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_89() {
    let h = parse_integer!(4); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_90() {
    let h = parse_integer!(4); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_91() {
    let h = parse_integer!(4); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_92() {
    let h = parse_integer!(4); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(23); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_93() {
    let h = parse_integer!(4); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(24); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_94() {
    let h = parse_integer!(4); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(24); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_95() {
    let h = parse_integer!(4); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(32); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_96() {
    let h = parse_integer!(4); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(81); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_97() {
    let h = parse_integer!(4); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(82); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_98() {
    let h = parse_integer!(4); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(215690); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_99() {
    let h = parse_integer!(4); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_100() {
    let h = parse_integer!(4); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870912); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_101() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_102() {
    let h = parse_integer!(4); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1073741823); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_103() {
    let h = parse_integer!(5); 
    let k = parse_integer!(5); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_104() {
    let h = parse_integer!(5); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_105() {
    let h = parse_integer!(5); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_106() {
    let h = parse_integer!(5); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_107() {
    let h = parse_integer!(5); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_108() {
    let h = parse_integer!(5); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_109() {
    let h = parse_integer!(5); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(25); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_110() {
    let h = parse_integer!(5); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(64); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_111() {
    let h = parse_integer!(5); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(66); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_112() {
    let h = parse_integer!(5); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(172552); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_113() {
    let h = parse_integer!(5); 
    let k = parse_integer!(1363729); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(272745); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_114() {
    let h = parse_integer!(5); 
    let k = parse_integer!(11973737); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2394747); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_115() {
    let h = parse_integer!(5); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(429496729); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_116() {
    let h = parse_integer!(5); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(429496729); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_117() {
    let h = parse_integer!(5); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(858993458); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_118() {
    let h = parse_integer!(5); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(858993459); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_119() {
    let h = parse_integer!(7); 
    let k = parse_integer!(421); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(60); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_120() {
    let h = parse_integer!(7); 
    let k = parse_integer!(2292); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(327); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_121() {
    let h = parse_integer!(8); 
    let k = parse_integer!(8); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_122() {
    let h = parse_integer!(8); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_123() {
    let h = parse_integer!(8); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(11); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_124() {
    let h = parse_integer!(8); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_125() {
    let h = parse_integer!(8); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_126() {
    let h = parse_integer!(8); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_127() {
    let h = parse_integer!(8); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(40); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_128() {
    let h = parse_integer!(8); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(41); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_129() {
    let h = parse_integer!(8); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(107845); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_130() {
    let h = parse_integer!(8); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(268435455); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_131() {
    let h = parse_integer!(8); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(268435456); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_132() {
    let h = parse_integer!(8); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(6); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_133() {
    let h = parse_integer!(8); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(536870911); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_134() {
    let h = parse_integer!(9); 
    let k = parse_integer!(328); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(9); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(36); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_135() {
    let h = parse_integer!(9); 
    let k = parse_integer!(3605393); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(9); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(400599); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_136() {
    let h = parse_integer!(10); 
    let k = parse_integer!(33702467); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(10); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3370246); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_137() {
    let h = parse_integer!(12); 
    let k = parse_integer!(5330981); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(12); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(444248); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_138() {
    let h = parse_integer!(13); 
    let k = parse_integer!(1383919464); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(13); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(106455343); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_139() {
    let h = parse_integer!(14); 
    let k = parse_integer!(708827196); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(14); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(50630514); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_140() {
    let h = parse_integer!(15); 
    let k = parse_integer!(29266833); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(15); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1951122); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_141() {
    let h = parse_integer!(16); 
    let k = parse_integer!(3378987); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(16); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(211186); 
    let kh_rem = parse_integer!(11); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_142() {
    let h = parse_integer!(18); 
    let k = parse_integer!(2664805); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(18); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(148044); 
    let kh_rem = parse_integer!(13); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_143() {
    let h = parse_integer!(19); 
    let k = parse_integer!(773); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(19); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(40); 
    let kh_rem = parse_integer!(13); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_144() {
    let h = parse_integer!(21); 
    let k = parse_integer!(6337600); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(21); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(301790); 
    let kh_rem = parse_integer!(10); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_145() {
    let h = parse_integer!(22); 
    let k = parse_integer!(85381211); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(22); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3880964); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_146() {
    let h = parse_integer!(24); 
    let k = parse_integer!(415); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(24); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(17); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_147() {
    let h = parse_integer!(26); 
    let k = parse_integer!(130); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(26); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_148() {
    let h = parse_integer!(28); 
    let k = parse_integer!(123381211); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(28); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4406471); 
    let kh_rem = parse_integer!(23); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_149() {
    let h = parse_integer!(32); 
    let k = parse_integer!(32); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_150() {
    let h = parse_integer!(32); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_151() {
    let h = parse_integer!(32); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_152() {
    let h = parse_integer!(32); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_153() {
    let h = parse_integer!(32); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_154() {
    let h = parse_integer!(32); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_155() {
    let h = parse_integer!(32); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10); 
    let kh_rem = parse_integer!(10); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_156() {
    let h = parse_integer!(32); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(26961); 
    let kh_rem = parse_integer!(9); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_157() {
    let h = parse_integer!(32); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(67108863); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_158() {
    let h = parse_integer!(32); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(67108864); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_159() {
    let h = parse_integer!(32); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(134217727); 
    let kh_rem = parse_integer!(30); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_160() {
    let h = parse_integer!(32); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(32); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(134217727); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_161() {
    let h = parse_integer!(40); 
    let k = parse_integer!(79403854); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(40); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1985096); 
    let kh_rem = parse_integer!(14); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_162() {
    let h = parse_integer!(40); 
    let k = parse_integer!(151555509); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(40); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3788887); 
    let kh_rem = parse_integer!(29); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_163() {
    let h = parse_integer!(52); 
    let k = parse_integer!(1325690); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(52); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(25494); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_164() {
    let h = parse_integer!(55); 
    let k = parse_integer!(30062); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(55); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(546); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_165() {
    let h = parse_integer!(58); 
    let k = parse_integer!(2683766); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(58); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(46271); 
    let kh_rem = parse_integer!(48); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_166() {
    let h = parse_integer!(63); 
    let k = parse_integer!(40227673); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(63); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(638534); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_167() {
    let h = parse_integer!(69); 
    let k = parse_integer!(1409815); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(69); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(20432); 
    let kh_rem = parse_integer!(7); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_168() {
    let h = parse_integer!(74); 
    let k = parse_integer!(3801819); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(74); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(51375); 
    let kh_rem = parse_integer!(69); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_169() {
    let h = parse_integer!(87); 
    let k = parse_integer!(28969631); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(87); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(332984); 
    let kh_rem = parse_integer!(23); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_170() {
    let h = parse_integer!(95); 
    let k = parse_integer!(95); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_171() {
    let h = parse_integer!(95); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_172() {
    let h = parse_integer!(95); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_173() {
    let h = parse_integer!(95); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_174() {
    let h = parse_integer!(95); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(39); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_175() {
    let h = parse_integer!(95); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(45); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_176() {
    let h = parse_integer!(95); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9081); 
    let kh_rem = parse_integer!(66); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_177() {
    let h = parse_integer!(95); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22605091); 
    let kh_rem = parse_integer!(2); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_178() {
    let h = parse_integer!(95); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22605091); 
    let kh_rem = parse_integer!(3); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_179() {
    let h = parse_integer!(95); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(45210182); 
    let kh_rem = parse_integer!(4); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_180() {
    let h = parse_integer!(95); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(95); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(45210182); 
    let kh_rem = parse_integer!(5); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_181() {
    let h = parse_integer!(96); 
    let k = parse_integer!(96); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_182() {
    let h = parse_integer!(96); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_183() {
    let h = parse_integer!(96); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_184() {
    let h = parse_integer!(96); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(36); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_185() {
    let h = parse_integer!(96); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(42); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_186() {
    let h = parse_integer!(96); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8987); 
    let kh_rem = parse_integer!(9); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_187() {
    let h = parse_integer!(96); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22369621); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_188() {
    let h = parse_integer!(96); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22369621); 
    let kh_rem = parse_integer!(32); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_189() {
    let h = parse_integer!(96); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44739242); 
    let kh_rem = parse_integer!(62); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_190() {
    let h = parse_integer!(96); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(96); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44739242); 
    let kh_rem = parse_integer!(63); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_191() {
    let h = parse_integer!(97); 
    let k = parse_integer!(97); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_192() {
    let h = parse_integer!(97); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(31); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_193() {
    let h = parse_integer!(97); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_194() {
    let h = parse_integer!(97); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(39); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_195() {
    let h = parse_integer!(97); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(8894); 
    let kh_rem = parse_integer!(43); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_196() {
    let h = parse_integer!(97); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22139006); 
    let kh_rem = parse_integer!(65); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_197() {
    let h = parse_integer!(97); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22139006); 
    let kh_rem = parse_integer!(66); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_198() {
    let h = parse_integer!(97); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44278013); 
    let kh_rem = parse_integer!(33); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_199() {
    let h = parse_integer!(97); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(97); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(44278013); 
    let kh_rem = parse_integer!(34); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_200() {
    let h = parse_integer!(118); 
    let k = parse_integer!(2501672); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(118); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(21200); 
    let kh_rem = parse_integer!(72); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_201() {
    let h = parse_integer!(128); 
    let k = parse_integer!(128); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_202() {
    let h = parse_integer!(128); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(68); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_203() {
    let h = parse_integer!(128); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(74); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_204() {
    let h = parse_integer!(128); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6740); 
    let kh_rem = parse_integer!(41); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_205() {
    let h = parse_integer!(128); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16777215); 
    let kh_rem = parse_integer!(127); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_206() {
    let h = parse_integer!(128); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16777216); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_207() {
    let h = parse_integer!(128); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(33554431); 
    let kh_rem = parse_integer!(126); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_208() {
    let h = parse_integer!(128); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(128); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(33554431); 
    let kh_rem = parse_integer!(127); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_209() {
    let h = parse_integer!(169); 
    let k = parse_integer!(15216588); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(169); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(90038); 
    let kh_rem = parse_integer!(166); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_210() {
    let h = parse_integer!(175); 
    let k = parse_integer!(89946081); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(175); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(513977); 
    let kh_rem = parse_integer!(106); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_211() {
    let h = parse_integer!(187); 
    let k = parse_integer!(864384); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(187); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4622); 
    let kh_rem = parse_integer!(70); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_212() {
    let h = parse_integer!(257); 
    let k = parse_integer!(5779029); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(257); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(22486); 
    let kh_rem = parse_integer!(127); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_213() {
    let h = parse_integer!(292); 
    let k = parse_integer!(482); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(292); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(190); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_214() {
    let h = parse_integer!(314); 
    let k = parse_integer!(81195); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(314); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(258); 
    let kh_rem = parse_integer!(183); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_215() {
    let h = parse_integer!(324); 
    let k = parse_integer!(324); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_216() {
    let h = parse_integer!(324); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(6); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_217() {
    let h = parse_integer!(324); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2662); 
    let kh_rem = parse_integer!(273); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_218() {
    let h = parse_integer!(324); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6628035); 
    let kh_rem = parse_integer!(307); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_219() {
    let h = parse_integer!(324); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6628035); 
    let kh_rem = parse_integer!(308); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_220() {
    let h = parse_integer!(324); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13256071); 
    let kh_rem = parse_integer!(290); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_221() {
    let h = parse_integer!(324); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(324); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13256071); 
    let kh_rem = parse_integer!(291); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_222() {
    let h = parse_integer!(330); 
    let k = parse_integer!(330); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_223() {
    let h = parse_integer!(330); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2614); 
    let kh_rem = parse_integer!(141); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_224() {
    let h = parse_integer!(330); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6507526); 
    let kh_rem = parse_integer!(67); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_225() {
    let h = parse_integer!(330); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6507526); 
    let kh_rem = parse_integer!(68); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_226() {
    let h = parse_integer!(330); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13015052); 
    let kh_rem = parse_integer!(134); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_227() {
    let h = parse_integer!(330); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(330); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(13015052); 
    let kh_rem = parse_integer!(135); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_228() {
    let h = parse_integer!(409); 
    let k = parse_integer!(15406); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(409); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(37); 
    let kh_rem = parse_integer!(273); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_229() {
    let h = parse_integer!(628); 
    let k = parse_integer!(1042); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(628); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(414); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_230() {
    let h = parse_integer!(767); 
    let k = parse_integer!(121677); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(767); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(158); 
    let kh_rem = parse_integer!(491); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_231() {
    let h = parse_integer!(1552); 
    let k = parse_integer!(14323); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1552); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9); 
    let kh_rem = parse_integer!(355); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_232() {
    let h = parse_integer!(1846); 
    let k = parse_integer!(2576995); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1846); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1395); 
    let kh_rem = parse_integer!(1825); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_233() {
    let h = parse_integer!(1889); 
    let k = parse_integer!(793387963); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1889); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(420004); 
    let kh_rem = parse_integer!(407); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_234() {
    let h = parse_integer!(1977); 
    let k = parse_integer!(37155916); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1977); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(18794); 
    let kh_rem = parse_integer!(178); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_235() {
    let h = parse_integer!(2182); 
    let k = parse_integer!(4221); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2182); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2039); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_236() {
    let h = parse_integer!(2288); 
    let k = parse_integer!(229002062); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2288); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(100088); 
    let kh_rem = parse_integer!(718); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_237() {
    let h = parse_integer!(2378); 
    let k = parse_integer!(8371617); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2378); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3520); 
    let kh_rem = parse_integer!(1057); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_238() {
    let h = parse_integer!(2549); 
    let k = parse_integer!(2645187); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2549); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1037); 
    let kh_rem = parse_integer!(1874); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_239() {
    let h = parse_integer!(2691); 
    let k = parse_integer!(165997848); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2691); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(61686); 
    let kh_rem = parse_integer!(822); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_240() {
    let h = parse_integer!(3064); 
    let k = parse_integer!(166734); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3064); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(54); 
    let kh_rem = parse_integer!(1278); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_241() {
    let h = parse_integer!(7265); 
    let k = parse_integer!(121116); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7265); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(16); 
    let kh_rem = parse_integer!(4876); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_242() {
    let h = parse_integer!(7953); 
    let k = parse_integer!(77616); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7953); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9); 
    let kh_rem = parse_integer!(6039); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_243() {
    let h = parse_integer!(8645); 
    let k = parse_integer!(248902814); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(8645); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(28791); 
    let kh_rem = parse_integer!(4619); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_244() {
    let h = parse_integer!(9428); 
    let k = parse_integer!(1314016411); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(9428); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(139373); 
    let kh_rem = parse_integer!(7767); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_245() {
    let h = parse_integer!(12125); 
    let k = parse_integer!(1604020914); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(12125); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(132290); 
    let kh_rem = parse_integer!(4664); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_246() {
    let h = parse_integer!(18068); 
    let k = parse_integer!(38177); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(18068); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(2041); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_247() {
    let h = parse_integer!(19689); 
    let k = parse_integer!(93177687); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(19689); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4732); 
    let kh_rem = parse_integer!(9339); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_248() {
    let h = parse_integer!(22613); 
    let k = parse_integer!(204921); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(22613); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9); 
    let kh_rem = parse_integer!(1404); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_249() {
    let h = parse_integer!(28159); 
    let k = parse_integer!(211764677); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(28159); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(7520); 
    let kh_rem = parse_integer!(8997); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_250() {
    let h = parse_integer!(42413); 
    let k = parse_integer!(211519); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(42413); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(41867); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_251() {
    let h = parse_integer!(47136); 
    let k = parse_integer!(238405); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(47136); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(2725); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_252() {
    let h = parse_integer!(47380); 
    let k = parse_integer!(2551374); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(47380); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(53); 
    let kh_rem = parse_integer!(40234); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_253() {
    let h = parse_integer!(59759); 
    let k = parse_integer!(200309); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(59759); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(21032); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_254() {
    let h = parse_integer!(73705); 
    let k = parse_integer!(759273579); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(73705); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(10301); 
    let kh_rem = parse_integer!(38374); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_255() {
    let h = parse_integer!(75914); 
    let k = parse_integer!(390025); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(75914); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(10455); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_256() {
    let h = parse_integer!(77056); 
    let k = parse_integer!(66625070); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(77056); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(864); 
    let kh_rem = parse_integer!(48686); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_257() {
    let h = parse_integer!(94362); 
    let k = parse_integer!(1157197446); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(94362); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(12263); 
    let kh_rem = parse_integer!(36240); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_258() {
    let h = parse_integer!(109482); 
    let k = parse_integer!(2759476); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(109482); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(25); 
    let kh_rem = parse_integer!(22426); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_259() {
    let h = parse_integer!(138561); 
    let k = parse_integer!(156234); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(138561); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(17673); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_260() {
    let h = parse_integer!(186716); 
    let k = parse_integer!(1778790); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(186716); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(9); 
    let kh_rem = parse_integer!(98346); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_261() {
    let h = parse_integer!(283840); 
    let k = parse_integer!(4135319934); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(283840); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(14569); 
    let kh_rem = parse_integer!(54974); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_262() {
    let h = parse_integer!(288805); 
    let k = parse_integer!(142722883); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(288805); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(494); 
    let kh_rem = parse_integer!(53213); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_263() {
    let h = parse_integer!(365512); 
    let k = parse_integer!(1435613291); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(365512); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3927); 
    let kh_rem = parse_integer!(247667); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_264() {
    let h = parse_integer!(374172); 
    let k = parse_integer!(2372776); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(374172); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(6); 
    let kh_rem = parse_integer!(127744); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_265() {
    let h = parse_integer!(622227); 
    let k = parse_integer!(17595891); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(622227); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(28); 
    let kh_rem = parse_integer!(173535); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_266() {
    let h = parse_integer!(822875); 
    let k = parse_integer!(16373909); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(822875); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(19); 
    let kh_rem = parse_integer!(739284); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_267() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(862761); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_268() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2489); 
    let kh_rem = parse_integer!(71518); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_269() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2489); 
    let kh_rem = parse_integer!(71519); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_270() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4978); 
    let kh_rem = parse_integer!(143036); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_271() {
    let h = parse_integer!(862761); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(862761); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4978); 
    let kh_rem = parse_integer!(143037); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_272() {
    let h = parse_integer!(987416); 
    let k = parse_integer!(1939660231); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(987416); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1964); 
    let kh_rem = parse_integer!(375207); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_273() {
    let h = parse_integer!(2103208); 
    let k = parse_integer!(2473114); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2103208); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(369906); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_274() {
    let h = parse_integer!(2377578); 
    let k = parse_integer!(2730333); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2377578); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(352755); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_275() {
    let h = parse_integer!(3373669); 
    let k = parse_integer!(8042894); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(3373669); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1295556); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_276() {
    let h = parse_integer!(5219449); 
    let k = parse_integer!(76649334); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(5219449); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(14); 
    let kh_rem = parse_integer!(3577048); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_277() {
    let h = parse_integer!(6983517); 
    let k = parse_integer!(4043828663); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(6983517); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(579); 
    let kh_rem = parse_integer!(372320); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_278() {
    let h = parse_integer!(7167838); 
    let k = parse_integer!(663533168); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(7167838); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(92); 
    let kh_rem = parse_integer!(4092072); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_279() {
    let h = parse_integer!(11727382); 
    let k = parse_integer!(22582041); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(11727382); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(10854659); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_280() {
    let h = parse_integer!(12061543); 
    let k = parse_integer!(38602954); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(12061543); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(3); 
    let kh_rem = parse_integer!(2418325); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_281() {
    let h = parse_integer!(13465092); 
    let k = parse_integer!(55316862); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(13465092); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(1456494); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_282() {
    let h = parse_integer!(38975436); 
    let k = parse_integer!(605142839); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(38975436); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(15); 
    let kh_rem = parse_integer!(20511299); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_283() {
    let h = parse_integer!(46753796); 
    let k = parse_integer!(2293448598); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(46753796); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(49); 
    let kh_rem = parse_integer!(2512594); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_284() {
    let h = parse_integer!(54384493); 
    let k = parse_integer!(110837908); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(54384493); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(2068922); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_285() {
    let h = parse_integer!(241989764); 
    let k = parse_integer!(1257447480); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(241989764); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(5); 
    let kh_rem = parse_integer!(47498660); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_286() {
    let h = parse_integer!(328484356); 
    let k = parse_integer!(2559291778); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(328484356); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(7); 
    let kh_rem = parse_integer!(259901286); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_287() {
    let h = parse_integer!(484713468); 
    let k = parse_integer!(2191430882); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(484713468); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(4); 
    let kh_rem = parse_integer!(252577010); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_288() {
    let h = parse_integer!(811595509); 
    let k = parse_integer!(1671571376); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(811595509); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(48380358); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_289() {
    let h = parse_integer!(1388195780); 
    let k = parse_integer!(2149940875); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1388195780); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(761745095); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_290() {
    let h = parse_integer!(1709737571); 
    let k = parse_integer!(3984332775); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(1709737571); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(564857633); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_291() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(2147483647); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_292() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_293() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_294() {
    let h = parse_integer!(2147483647); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483647); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(2); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_295() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(2147483648); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_296() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483648); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2147483646); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_297() {
    let h = parse_integer!(2147483648); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(2147483648); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(2147483647); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_298() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(4294967294); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_299() {
    let h = parse_integer!(4294967294); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(0); 
    let hk_rem = parse_integer!(4294967294); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(1); 
    div_equal(k, h, kh_quot, kh_rem);
}

#[test]
fn test_set_300() {
    let h = parse_integer!(4294967295); 
    let k = parse_integer!(4294967295); 
    let hk_quot = parse_integer!(1); 
    let hk_rem = parse_integer!(0); 
    div_equal(h, k, hk_quot, hk_rem);
    let kh_quot = parse_integer!(1); 
    let kh_rem = parse_integer!(0); 
    div_equal(k, h, kh_quot, kh_rem);
}
