//! If this compiles then all tests passed
use super::*;
use const_arith_macros::{parse_integer};
use std::marker::PhantomData;

#[test]
fn test_assert_3() {
    // let a = parse_integer!(3);
    let a = parse_integer!(3);

    fn asserting<T, S0, S1, S2, S3, S4, S5, S6, S7>(_: &T) where
        S0: Hex,
        S1: Hex,
        S2: Hex,
        S3: Hex,
        S4: Hex,
        S5: Hex,
        S6: Hex,
        S7: Hex,
        T: IsInteger<Hex0 = S0, Hex1 = S1, Hex2 = S2, Hex3 = S3, Hex4 = S4, Hex5 = S5, Hex6 = S6, Hex7 = S7>,
        S0: HexAssertEqual<_3>,
        S1: HexAssertEqual<_0>,
        S2: HexAssertEqual<_0>,
        S3: HexAssertEqual<_0>,
        S4: HexAssertEqual<_0>,
        S5: HexAssertEqual<_0>,
        S6: HexAssertEqual<_0>,
        S7: HexAssertEqual<_0>,
    {}

    asserting(&a);
    typed_assert_eq!(a, 3);
}

// Test cases generated using this python snippet
// import random
// random.seed(12345)
// a = set()
// while len(a) < 100:
//     r = random.random() * 32
//     r = int(2 ** r)
//     if r in a:
//         continue
//     a.add(r)
// for i, x in enumerate(sorted(a)):
//     print(f"let a{i} = parse_integer!({x});\ntyped_assert_eq!(a{i}, {x});")
#[test]
fn test_asserts() {
    let a0 = parse_integer!(1); typed_assert_eq!(a0, 1);
    let a1 = parse_integer!(2); typed_assert_eq!(a1, 2);
    let a2 = parse_integer!(3); typed_assert_eq!(a2, 3);
    let a3 = parse_integer!(5); typed_assert_eq!(a3, 5);
    let a4 = parse_integer!(7); typed_assert_eq!(a4, 7);
    let a5 = parse_integer!(8); typed_assert_eq!(a5, 8);
    let a6 = parse_integer!(9); typed_assert_eq!(a6, 9);
    let a7 = parse_integer!(15); typed_assert_eq!(a7, 15);
    let a8 = parse_integer!(26); typed_assert_eq!(a8, 26);
    let a9 = parse_integer!(28); typed_assert_eq!(a9, 28);
    let a10 = parse_integer!(32); typed_assert_eq!(a10, 32);
    let a11 = parse_integer!(36); typed_assert_eq!(a11, 36);
    let a12 = parse_integer!(37); typed_assert_eq!(a12, 37);
    let a13 = parse_integer!(38); typed_assert_eq!(a13, 38);
    let a14 = parse_integer!(47); typed_assert_eq!(a14, 47);
    let a15 = parse_integer!(51); typed_assert_eq!(a15, 51);
    let a16 = parse_integer!(52); typed_assert_eq!(a16, 52);
    let a17 = parse_integer!(60); typed_assert_eq!(a17, 60);
    let a18 = parse_integer!(61); typed_assert_eq!(a18, 61);
    let a19 = parse_integer!(67); typed_assert_eq!(a19, 67);
    let a20 = parse_integer!(73); typed_assert_eq!(a20, 73);
    let a21 = parse_integer!(87); typed_assert_eq!(a21, 87);
    let a22 = parse_integer!(121); typed_assert_eq!(a22, 121);
    let a23 = parse_integer!(161); typed_assert_eq!(a23, 161);
    let a24 = parse_integer!(166); typed_assert_eq!(a24, 166);
    let a25 = parse_integer!(172); typed_assert_eq!(a25, 172);
    let a26 = parse_integer!(174); typed_assert_eq!(a26, 174);
    let a27 = parse_integer!(220); typed_assert_eq!(a27, 220);
    let a28 = parse_integer!(598); typed_assert_eq!(a28, 598);
    let a29 = parse_integer!(752); typed_assert_eq!(a29, 752);
    let a30 = parse_integer!(1314); typed_assert_eq!(a30, 1314);
    let a31 = parse_integer!(1374); typed_assert_eq!(a31, 1374);
    let a32 = parse_integer!(1525); typed_assert_eq!(a32, 1525);
    let a33 = parse_integer!(1864); typed_assert_eq!(a33, 1864);
    let a34 = parse_integer!(1950); typed_assert_eq!(a34, 1950);
    let a35 = parse_integer!(2622); typed_assert_eq!(a35, 2622);
    let a36 = parse_integer!(3487); typed_assert_eq!(a36, 3487);
    let a37 = parse_integer!(3539); typed_assert_eq!(a37, 3539);
    let a38 = parse_integer!(6351); typed_assert_eq!(a38, 6351);
    let a39 = parse_integer!(8171); typed_assert_eq!(a39, 8171);
    let a40 = parse_integer!(9331); typed_assert_eq!(a40, 9331);
    let a41 = parse_integer!(9844); typed_assert_eq!(a41, 9844);
    let a42 = parse_integer!(10310); typed_assert_eq!(a42, 10310);
    let a43 = parse_integer!(10333); typed_assert_eq!(a43, 10333);
    let a44 = parse_integer!(11319); typed_assert_eq!(a44, 11319);
    let a45 = parse_integer!(14806); typed_assert_eq!(a45, 14806);
    let a46 = parse_integer!(42785); typed_assert_eq!(a46, 42785);
    let a47 = parse_integer!(60060); typed_assert_eq!(a47, 60060);
    let a48 = parse_integer!(71513); typed_assert_eq!(a48, 71513);
    let a49 = parse_integer!(73494); typed_assert_eq!(a49, 73494);
    let a50 = parse_integer!(98215); typed_assert_eq!(a50, 98215);
    let a51 = parse_integer!(101254); typed_assert_eq!(a51, 101254);
    let a52 = parse_integer!(110610); typed_assert_eq!(a52, 110610);
    let a53 = parse_integer!(149300); typed_assert_eq!(a53, 149300);
    let a54 = parse_integer!(156133); typed_assert_eq!(a54, 156133);
    let a55 = parse_integer!(213381); typed_assert_eq!(a55, 213381);
    let a56 = parse_integer!(222389); typed_assert_eq!(a56, 222389);
    let a57 = parse_integer!(229799); typed_assert_eq!(a57, 229799);
    let a58 = parse_integer!(259704); typed_assert_eq!(a58, 259704);
    let a59 = parse_integer!(283357); typed_assert_eq!(a59, 283357);
    let a60 = parse_integer!(370407); typed_assert_eq!(a60, 370407);
    let a61 = parse_integer!(467781); typed_assert_eq!(a61, 467781);
    let a62 = parse_integer!(711264); typed_assert_eq!(a62, 711264);
    let a63 = parse_integer!(753292); typed_assert_eq!(a63, 753292);
    let a64 = parse_integer!(1074568); typed_assert_eq!(a64, 1074568);
    let a65 = parse_integer!(3451034); typed_assert_eq!(a65, 3451034);
    let a66 = parse_integer!(5360660); typed_assert_eq!(a66, 5360660);
    let a67 = parse_integer!(6815395); typed_assert_eq!(a67, 6815395);
    let a68 = parse_integer!(7170654); typed_assert_eq!(a68, 7170654);
    let a69 = parse_integer!(8429107); typed_assert_eq!(a69, 8429107);
    let a70 = parse_integer!(12458533); typed_assert_eq!(a70, 12458533);
    let a71 = parse_integer!(14860314); typed_assert_eq!(a71, 14860314);
    let a72 = parse_integer!(21536501); typed_assert_eq!(a72, 21536501);
    let a73 = parse_integer!(25489570); typed_assert_eq!(a73, 25489570);
    let a74 = parse_integer!(26632764); typed_assert_eq!(a74, 26632764);
    let a75 = parse_integer!(27577502); typed_assert_eq!(a75, 27577502);
    let a76 = parse_integer!(39696924); typed_assert_eq!(a76, 39696924);
    let a77 = parse_integer!(41153307); typed_assert_eq!(a77, 41153307);
    let a78 = parse_integer!(49154391); typed_assert_eq!(a78, 49154391);
    let a79 = parse_integer!(53047285); typed_assert_eq!(a79, 53047285);
    let a80 = parse_integer!(59565971); typed_assert_eq!(a80, 59565971);
    let a81 = parse_integer!(88225129); typed_assert_eq!(a81, 88225129);
    let a82 = parse_integer!(88957215); typed_assert_eq!(a82, 88957215);
    let a83 = parse_integer!(103895386); typed_assert_eq!(a83, 103895386);
    let a84 = parse_integer!(244378648); typed_assert_eq!(a84, 244378648);
    let a85 = parse_integer!(257871831); typed_assert_eq!(a85, 257871831);
    let a86 = parse_integer!(289933678); typed_assert_eq!(a86, 289933678);
    let a87 = parse_integer!(339107381); typed_assert_eq!(a87, 339107381);
    let a88 = parse_integer!(737585385); typed_assert_eq!(a88, 737585385);
    let a89 = parse_integer!(1140984541); typed_assert_eq!(a89, 1140984541);
    let a90 = parse_integer!(1498617499); typed_assert_eq!(a90, 1498617499);
    let a91 = parse_integer!(1575728180); typed_assert_eq!(a91, 1575728180);
    let a92 = parse_integer!(1694336315); typed_assert_eq!(a92, 1694336315);
    let a93 = parse_integer!(1716949082); typed_assert_eq!(a93, 1716949082);
    let a94 = parse_integer!(2078683947); typed_assert_eq!(a94, 2078683947);
    let a95 = parse_integer!(2087948069); typed_assert_eq!(a95, 2087948069);
    let a96 = parse_integer!(2674236400); typed_assert_eq!(a96, 2674236400);
    let a97 = parse_integer!(2683652142); typed_assert_eq!(a97, 2683652142);
    let a98 = parse_integer!(2964958777); typed_assert_eq!(a98, 2964958777);
    let a99 = parse_integer!(3810507542); typed_assert_eq!(a99, 3810507542);
}