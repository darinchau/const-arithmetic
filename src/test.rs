//! If this compiles then all tests passed
use super::*;
use const_arith_macros::{parse_integer};
use std::marker::PhantomData;

// All the code used to generate test cases can be found in a.ipynb

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

#[test]
fn test_asserts() {
    let a0 = parse_integer!(1); assert_eq!(a0.number(), 1);
    let a1 = parse_integer!(7); assert_eq!(a1.number(), 7);
    let a2 = parse_integer!(9); assert_eq!(a2.number(), 9);
    let a3 = parse_integer!(15); assert_eq!(a3.number(), 15);
    let a4 = parse_integer!(26); assert_eq!(a4.number(), 26);
    let a5 = parse_integer!(36); assert_eq!(a5.number(), 36);
    let a6 = parse_integer!(47); assert_eq!(a6.number(), 47);
    let a7 = parse_integer!(52); assert_eq!(a7.number(), 52);
    let a8 = parse_integer!(61); assert_eq!(a8.number(), 61);
    let a9 = parse_integer!(67); assert_eq!(a9.number(), 67);
    let a10 = parse_integer!(73); assert_eq!(a10.number(), 73);
    let a11 = parse_integer!(121); assert_eq!(a11.number(), 121);
    let a12 = parse_integer!(172); assert_eq!(a12.number(), 172);
    let a13 = parse_integer!(174); assert_eq!(a13.number(), 174);
    let a14 = parse_integer!(752); assert_eq!(a14.number(), 752);
    let a15 = parse_integer!(1864); assert_eq!(a15.number(), 1864);
    let a16 = parse_integer!(1950); assert_eq!(a16.number(), 1950);
    let a17 = parse_integer!(2622); assert_eq!(a17.number(), 2622);
    let a18 = parse_integer!(3539); assert_eq!(a18.number(), 3539);
    let a19 = parse_integer!(9331); assert_eq!(a19.number(), 9331);
    let a20 = parse_integer!(9844); assert_eq!(a20.number(), 9844);
    let a21 = parse_integer!(10310); assert_eq!(a21.number(), 10310);
    let a22 = parse_integer!(14806); assert_eq!(a22.number(), 14806);
    let a23 = parse_integer!(71513); assert_eq!(a23.number(), 71513);
    let a24 = parse_integer!(98215); assert_eq!(a24.number(), 98215);
    let a25 = parse_integer!(110610); assert_eq!(a25.number(), 110610);
    let a26 = parse_integer!(213381); assert_eq!(a26.number(), 213381);
    let a27 = parse_integer!(222389); assert_eq!(a27.number(), 222389);
    let a28 = parse_integer!(259704); assert_eq!(a28.number(), 259704);
    let a29 = parse_integer!(283357); assert_eq!(a29.number(), 283357);
    let a30 = parse_integer!(370407); assert_eq!(a30.number(), 370407);
    let a31 = parse_integer!(467781); assert_eq!(a31.number(), 467781);
    let a32 = parse_integer!(6815395); assert_eq!(a32.number(), 6815395);
    let a33 = parse_integer!(8429107); assert_eq!(a33.number(), 8429107);
    let a34 = parse_integer!(12458533); assert_eq!(a34.number(), 12458533);
    let a35 = parse_integer!(14860314); assert_eq!(a35.number(), 14860314);
    let a36 = parse_integer!(26632764); assert_eq!(a36.number(), 26632764);
    let a37 = parse_integer!(53047285); assert_eq!(a37.number(), 53047285);
    let a38 = parse_integer!(59565971); assert_eq!(a38.number(), 59565971);
    let a39 = parse_integer!(88225129); assert_eq!(a39.number(), 88225129);
    let a40 = parse_integer!(88957215); assert_eq!(a40.number(), 88957215);
    let a41 = parse_integer!(244378648); assert_eq!(a41.number(), 244378648);
    let a42 = parse_integer!(257871831); assert_eq!(a42.number(), 257871831);
    let a43 = parse_integer!(339107381); assert_eq!(a43.number(), 339107381);
    let a44 = parse_integer!(1140984541); assert_eq!(a44.number(), 1140984541);
    let a45 = parse_integer!(1694336315); assert_eq!(a45.number(), 1694336315);
    let a46 = parse_integer!(1716949082); assert_eq!(a46.number(), 1716949082);
    let a47 = parse_integer!(2087948069); assert_eq!(a47.number(), 2087948069);
    let a48 = parse_integer!(2674236400); assert_eq!(a48.number(), 2674236400);
    let a49 = parse_integer!(2683652142); assert_eq!(a49.number(), 2683652142);
}


fn add_equal<P, R, S>(_a0: &P, _a1: &R, _a2: &S) where
P: IsInteger,
R: IsInteger,
S: IsInteger,
P: Add<R, Output = S>
// Another way of implementing
// P: IsInteger,
// R: IsInteger,
// S: IsInteger,
// P: Add<R, Output = PR>
// PR: TypedAssertEq<S>
{}

#[test]
fn additions() {
    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(1); let a0_2 = parse_integer!(2); add_equal(&a0_0, &a0_1, &a0_2);
    let a1_0 = parse_integer!(2); let a1_1 = parse_integer!(38); let a1_2 = parse_integer!(40); add_equal(&a1_0, &a1_1, &a1_2);
    let a2_0 = parse_integer!(5); let a2_1 = parse_integer!(38); let a2_2 = parse_integer!(43); add_equal(&a2_0, &a2_1, &a2_2);
    let a3_0 = parse_integer!(52); let a3_1 = parse_integer!(9); let a3_2 = parse_integer!(61); add_equal(&a3_0, &a3_1, &a3_2);
    let a4_0 = parse_integer!(28); let a4_1 = parse_integer!(161); let a4_2 = parse_integer!(189); add_equal(&a4_0, &a4_1, &a4_2);
    let a5_0 = parse_integer!(37); let a5_1 = parse_integer!(598); let a5_2 = parse_integer!(635); add_equal(&a5_0, &a5_1, &a5_2);
    let a6_0 = parse_integer!(1314); let a6_1 = parse_integer!(2); let a6_2 = parse_integer!(1316); add_equal(&a6_0, &a6_1, &a6_2);
    let a7_0 = parse_integer!(1950); let a7_1 = parse_integer!(1); let a7_2 = parse_integer!(1951); add_equal(&a7_0, &a7_1, &a7_2);
    let a8_0 = parse_integer!(3539); let a8_1 = parse_integer!(73); let a8_2 = parse_integer!(3612); add_equal(&a8_0, &a8_1, &a8_2);
    let a9_0 = parse_integer!(10310); let a9_1 = parse_integer!(1); let a9_2 = parse_integer!(10311); add_equal(&a9_0, &a9_1, &a9_2);
    let a10_0 = parse_integer!(15); let a10_1 = parse_integer!(14806); let a10_2 = parse_integer!(14821); add_equal(&a10_0, &a10_1, &a10_2);
    let a11_0 = parse_integer!(60060); let a11_1 = parse_integer!(60); let a11_2 = parse_integer!(60120); add_equal(&a11_0, &a11_1, &a11_2);
    let a12_0 = parse_integer!(71513); let a12_1 = parse_integer!(26); let a12_2 = parse_integer!(71539); add_equal(&a12_0, &a12_1, &a12_2);
    let a13_0 = parse_integer!(8); let a13_1 = parse_integer!(73494); let a13_2 = parse_integer!(73502); add_equal(&a13_0, &a13_1, &a13_2);
    let a14_0 = parse_integer!(174); let a14_1 = parse_integer!(110610); let a14_2 = parse_integer!(110784); add_equal(&a14_0, &a14_1, &a14_2);
    let a15_0 = parse_integer!(156133); let a15_1 = parse_integer!(32); let a15_2 = parse_integer!(156165); add_equal(&a15_0, &a15_1, &a15_2);
    let a16_0 = parse_integer!(213381); let a16_1 = parse_integer!(2622); let a16_2 = parse_integer!(216003); add_equal(&a16_0, &a16_1, &a16_2);
    let a17_0 = parse_integer!(149300); let a17_1 = parse_integer!(101254); let a17_2 = parse_integer!(250554); add_equal(&a17_0, &a17_1, &a17_2);
    let a18_0 = parse_integer!(259704); let a18_1 = parse_integer!(47); let a18_2 = parse_integer!(259751); add_equal(&a18_0, &a18_1, &a18_2);
    let a19_0 = parse_integer!(283357); let a19_1 = parse_integer!(36); let a19_2 = parse_integer!(283393); add_equal(&a19_0, &a19_1, &a19_2);
    let a20_0 = parse_integer!(61); let a20_1 = parse_integer!(467781); let a20_2 = parse_integer!(467842); add_equal(&a20_0, &a20_1, &a20_2);
    let a21_0 = parse_integer!(1374); let a21_1 = parse_integer!(753292); let a21_2 = parse_integer!(754666); add_equal(&a21_0, &a21_1, &a21_2);
    let a22_0 = parse_integer!(3); let a22_1 = parse_integer!(1074568); let a22_2 = parse_integer!(1074571); add_equal(&a22_0, &a22_1, &a22_2);
    let a23_0 = parse_integer!(3487); let a23_1 = parse_integer!(3451034); let a23_2 = parse_integer!(3454521); add_equal(&a23_0, &a23_1, &a23_2);
    let a24_0 = parse_integer!(5360660); let a24_1 = parse_integer!(229799); let a24_2 = parse_integer!(5590459); add_equal(&a24_0, &a24_1, &a24_2);
    let a25_0 = parse_integer!(7170654); let a25_1 = parse_integer!(5); let a25_2 = parse_integer!(7170659); add_equal(&a25_0, &a25_1, &a25_2);
    let a26_0 = parse_integer!(8429107); let a26_1 = parse_integer!(67); let a26_2 = parse_integer!(8429174); add_equal(&a26_0, &a26_1, &a26_2);
    let a27_0 = parse_integer!(370407); let a27_1 = parse_integer!(12458533); let a27_2 = parse_integer!(12828940); add_equal(&a27_0, &a27_1, &a27_2);
    let a28_0 = parse_integer!(21536501); let a28_1 = parse_integer!(42785); let a28_2 = parse_integer!(21579286); add_equal(&a28_0, &a28_1, &a28_2);
    let a29_0 = parse_integer!(87); let a29_1 = parse_integer!(25489570); let a29_2 = parse_integer!(25489657); add_equal(&a29_0, &a29_1, &a29_2);
    let a30_0 = parse_integer!(8171); let a30_1 = parse_integer!(41153307); let a30_2 = parse_integer!(41161478); add_equal(&a30_0, &a30_1, &a30_2);
    let a31_0 = parse_integer!(711264); let a31_1 = parse_integer!(49154391); let a31_2 = parse_integer!(49865655); add_equal(&a31_0, &a31_1, &a31_2);
    let a32_0 = parse_integer!(53047285); let a32_1 = parse_integer!(222389); let a32_2 = parse_integer!(53269674); add_equal(&a32_0, &a32_1, &a32_2);
    let a33_0 = parse_integer!(59565971); let a33_1 = parse_integer!(172); let a33_2 = parse_integer!(59566143); add_equal(&a33_0, &a33_1, &a33_2);
    let a34_0 = parse_integer!(39696924); let a34_1 = parse_integer!(27577502); let a34_2 = parse_integer!(67274426); add_equal(&a34_0, &a34_1, &a34_2);
    let a35_0 = parse_integer!(88957215); let a35_1 = parse_integer!(752); let a35_2 = parse_integer!(88957967); add_equal(&a35_0, &a35_1, &a35_2);
    let a36_0 = parse_integer!(6815395); let a36_1 = parse_integer!(88225129); let a36_2 = parse_integer!(95040524); add_equal(&a36_0, &a36_1, &a36_2);
    let a37_0 = parse_integer!(257871831); let a37_1 = parse_integer!(121); let a37_2 = parse_integer!(257871952); add_equal(&a37_0, &a37_1, &a37_2);
    let a38_0 = parse_integer!(244378648); let a38_1 = parse_integer!(26632764); let a38_2 = parse_integer!(271011412); add_equal(&a38_0, &a38_1, &a38_2);
    let a39_0 = parse_integer!(9844); let a39_1 = parse_integer!(339107381); let a39_2 = parse_integer!(339117225); add_equal(&a39_0, &a39_1, &a39_2);
    let a40_0 = parse_integer!(1); let a40_1 = parse_integer!(1140984541); let a40_2 = parse_integer!(1140984542); add_equal(&a40_0, &a40_1, &a40_2);
    let a41_0 = parse_integer!(11319); let a41_1 = parse_integer!(1498617499); let a41_2 = parse_integer!(1498628818); add_equal(&a41_0, &a41_1, &a41_2);
    let a42_0 = parse_integer!(1575728180); let a42_1 = parse_integer!(10333); let a42_2 = parse_integer!(1575738513); add_equal(&a42_0, &a42_1, &a42_2);
    let a43_0 = parse_integer!(1694336315); let a43_1 = parse_integer!(7); let a43_2 = parse_integer!(1694336322); add_equal(&a43_0, &a43_1, &a43_2);
    let a44_0 = parse_integer!(98215); let a44_1 = parse_integer!(1716949082); let a44_2 = parse_integer!(1717047297); add_equal(&a44_0, &a44_1, &a44_2);
    let a45_0 = parse_integer!(2078683947); let a45_1 = parse_integer!(166); let a45_2 = parse_integer!(2078684113); add_equal(&a45_0, &a45_1, &a45_2);
    let a46_0 = parse_integer!(1864); let a46_1 = parse_integer!(2087948069); let a46_2 = parse_integer!(2087949933); add_equal(&a46_0, &a46_1, &a46_2);
    let a47_0 = parse_integer!(2674236400); let a47_1 = parse_integer!(9331); let a47_2 = parse_integer!(2674245731); add_equal(&a47_0, &a47_1, &a47_2);
    let a48_0 = parse_integer!(2683652142); let a48_1 = parse_integer!(14860314); let a48_2 = parse_integer!(2698512456); add_equal(&a48_0, &a48_1, &a48_2);
    let a49_0 = parse_integer!(2964958777); let a49_1 = parse_integer!(103895386); let a49_2 = parse_integer!(3068854163); add_equal(&a49_0, &a49_1, &a49_2);
    
}

fn sub_equal<P, R, S>(_a0: &P, _a1: &R, _a2: &S) where
P: IsInteger,
R: IsInteger,
S: IsInteger,
R: Sub<P, Output = S>
{}

#[test]
fn subtraction() {
    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(1); let a0_2 = parse_integer!(0); sub_equal(&a0_0, &a0_1, &a0_2);
    let a1_0 = parse_integer!(5); let a1_1 = parse_integer!(38); let a1_2 = parse_integer!(33); sub_equal(&a1_0, &a1_1, &a1_2);
    let a2_0 = parse_integer!(2); let a2_1 = parse_integer!(38); let a2_2 = parse_integer!(36); sub_equal(&a2_0, &a2_1, &a2_2);
    let a3_0 = parse_integer!(9); let a3_1 = parse_integer!(52); let a3_2 = parse_integer!(43); sub_equal(&a3_0, &a3_1, &a3_2);
    let a4_0 = parse_integer!(28); let a4_1 = parse_integer!(161); let a4_2 = parse_integer!(133); sub_equal(&a4_0, &a4_1, &a4_2);
    let a5_0 = parse_integer!(37); let a5_1 = parse_integer!(598); let a5_2 = parse_integer!(561); sub_equal(&a5_0, &a5_1, &a5_2);
    let a6_0 = parse_integer!(2); let a6_1 = parse_integer!(1314); let a6_2 = parse_integer!(1312); sub_equal(&a6_0, &a6_1, &a6_2);
    let a7_0 = parse_integer!(1); let a7_1 = parse_integer!(1950); let a7_2 = parse_integer!(1949); sub_equal(&a7_0, &a7_1, &a7_2);
    let a8_0 = parse_integer!(73); let a8_1 = parse_integer!(3539); let a8_2 = parse_integer!(3466); sub_equal(&a8_0, &a8_1, &a8_2);
    let a9_0 = parse_integer!(1); let a9_1 = parse_integer!(10310); let a9_2 = parse_integer!(10309); sub_equal(&a9_0, &a9_1, &a9_2);
    let a10_0 = parse_integer!(15); let a10_1 = parse_integer!(14806); let a10_2 = parse_integer!(14791); sub_equal(&a10_0, &a10_1, &a10_2);
    let a11_0 = parse_integer!(101254); let a11_1 = parse_integer!(149300); let a11_2 = parse_integer!(48046); sub_equal(&a11_0, &a11_1, &a11_2);
    let a12_0 = parse_integer!(60); let a12_1 = parse_integer!(60060); let a12_2 = parse_integer!(60000); sub_equal(&a12_0, &a12_1, &a12_2);
    let a13_0 = parse_integer!(26); let a13_1 = parse_integer!(71513); let a13_2 = parse_integer!(71487); sub_equal(&a13_0, &a13_1, &a13_2);
    let a14_0 = parse_integer!(8); let a14_1 = parse_integer!(73494); let a14_2 = parse_integer!(73486); sub_equal(&a14_0, &a14_1, &a14_2);
    let a15_0 = parse_integer!(174); let a15_1 = parse_integer!(110610); let a15_2 = parse_integer!(110436); sub_equal(&a15_0, &a15_1, &a15_2);
    let a16_0 = parse_integer!(32); let a16_1 = parse_integer!(156133); let a16_2 = parse_integer!(156101); sub_equal(&a16_0, &a16_1, &a16_2);
    let a17_0 = parse_integer!(2622); let a17_1 = parse_integer!(213381); let a17_2 = parse_integer!(210759); sub_equal(&a17_0, &a17_1, &a17_2);
    let a18_0 = parse_integer!(47); let a18_1 = parse_integer!(259704); let a18_2 = parse_integer!(259657); sub_equal(&a18_0, &a18_1, &a18_2);
    let a19_0 = parse_integer!(36); let a19_1 = parse_integer!(283357); let a19_2 = parse_integer!(283321); sub_equal(&a19_0, &a19_1, &a19_2);
    let a20_0 = parse_integer!(61); let a20_1 = parse_integer!(467781); let a20_2 = parse_integer!(467720); sub_equal(&a20_0, &a20_1, &a20_2);
    let a21_0 = parse_integer!(1374); let a21_1 = parse_integer!(753292); let a21_2 = parse_integer!(751918); sub_equal(&a21_0, &a21_1, &a21_2);
    let a22_0 = parse_integer!(3); let a22_1 = parse_integer!(1074568); let a22_2 = parse_integer!(1074565); sub_equal(&a22_0, &a22_1, &a22_2);
    let a23_0 = parse_integer!(3487); let a23_1 = parse_integer!(3451034); let a23_2 = parse_integer!(3447547); sub_equal(&a23_0, &a23_1, &a23_2);
    let a24_0 = parse_integer!(229799); let a24_1 = parse_integer!(5360660); let a24_2 = parse_integer!(5130861); sub_equal(&a24_0, &a24_1, &a24_2);
    let a25_0 = parse_integer!(5); let a25_1 = parse_integer!(7170654); let a25_2 = parse_integer!(7170649); sub_equal(&a25_0, &a25_1, &a25_2);
    let a26_0 = parse_integer!(67); let a26_1 = parse_integer!(8429107); let a26_2 = parse_integer!(8429040); sub_equal(&a26_0, &a26_1, &a26_2);
    let a27_0 = parse_integer!(370407); let a27_1 = parse_integer!(12458533); let a27_2 = parse_integer!(12088126); sub_equal(&a27_0, &a27_1, &a27_2);
    let a28_0 = parse_integer!(27577502); let a28_1 = parse_integer!(39696924); let a28_2 = parse_integer!(12119422); sub_equal(&a28_0, &a28_1, &a28_2);
    let a29_0 = parse_integer!(42785); let a29_1 = parse_integer!(21536501); let a29_2 = parse_integer!(21493716); sub_equal(&a29_0, &a29_1, &a29_2);
    let a30_0 = parse_integer!(87); let a30_1 = parse_integer!(25489570); let a30_2 = parse_integer!(25489483); sub_equal(&a30_0, &a30_1, &a30_2);
    let a31_0 = parse_integer!(8171); let a31_1 = parse_integer!(41153307); let a31_2 = parse_integer!(41145136); sub_equal(&a31_0, &a31_1, &a31_2);
    let a32_0 = parse_integer!(711264); let a32_1 = parse_integer!(49154391); let a32_2 = parse_integer!(48443127); sub_equal(&a32_0, &a32_1, &a32_2);
    let a33_0 = parse_integer!(222389); let a33_1 = parse_integer!(53047285); let a33_2 = parse_integer!(52824896); sub_equal(&a33_0, &a33_1, &a33_2);
    let a34_0 = parse_integer!(172); let a34_1 = parse_integer!(59565971); let a34_2 = parse_integer!(59565799); sub_equal(&a34_0, &a34_1, &a34_2);
    let a35_0 = parse_integer!(6815395); let a35_1 = parse_integer!(88225129); let a35_2 = parse_integer!(81409734); sub_equal(&a35_0, &a35_1, &a35_2);
    let a36_0 = parse_integer!(752); let a36_1 = parse_integer!(88957215); let a36_2 = parse_integer!(88956463); sub_equal(&a36_0, &a36_1, &a36_2);
    let a37_0 = parse_integer!(26632764); let a37_1 = parse_integer!(244378648); let a37_2 = parse_integer!(217745884); sub_equal(&a37_0, &a37_1, &a37_2);
    let a38_0 = parse_integer!(121); let a38_1 = parse_integer!(257871831); let a38_2 = parse_integer!(257871710); sub_equal(&a38_0, &a38_1, &a38_2);
    let a39_0 = parse_integer!(9844); let a39_1 = parse_integer!(339107381); let a39_2 = parse_integer!(339097537); sub_equal(&a39_0, &a39_1, &a39_2);
    let a40_0 = parse_integer!(1); let a40_1 = parse_integer!(1140984541); let a40_2 = parse_integer!(1140984540); sub_equal(&a40_0, &a40_1, &a40_2);
    let a41_0 = parse_integer!(11319); let a41_1 = parse_integer!(1498617499); let a41_2 = parse_integer!(1498606180); sub_equal(&a41_0, &a41_1, &a41_2);
    let a42_0 = parse_integer!(7); let a42_1 = parse_integer!(1694336315); let a42_2 = parse_integer!(1694336308); sub_equal(&a42_0, &a42_1, &a42_2);
    let a43_0 = parse_integer!(98215); let a43_1 = parse_integer!(1716949082); let a43_2 = parse_integer!(1716850867); sub_equal(&a43_0, &a43_1, &a43_2);
    let a44_0 = parse_integer!(166); let a44_1 = parse_integer!(2078683947); let a44_2 = parse_integer!(2078683781); sub_equal(&a44_0, &a44_1, &a44_2);
    let a45_0 = parse_integer!(1864); let a45_1 = parse_integer!(2087948069); let a45_2 = parse_integer!(2087946205); sub_equal(&a45_0, &a45_1, &a45_2);
    let a46_0 = parse_integer!(14860314); let a46_1 = parse_integer!(2683652142); let a46_2 = parse_integer!(2668791828); sub_equal(&a46_0, &a46_1, &a46_2);
    let a47_0 = parse_integer!(9331); let a47_1 = parse_integer!(2674236400); let a47_2 = parse_integer!(2674227069); sub_equal(&a47_0, &a47_1, &a47_2);
    let a48_0 = parse_integer!(103895386); let a48_1 = parse_integer!(2964958777); let a48_2 = parse_integer!(2861063391); sub_equal(&a48_0, &a48_1, &a48_2);
    let a49_0 = parse_integer!(737585385); let a49_1 = parse_integer!(3810507542); let a49_2 = parse_integer!(3072922157); sub_equal(&a49_0, &a49_1, &a49_2);
}

fn mul_equal<P, Q, R, S>(_p: &P, _q: &Q, _r: &R, _s: &S) where
P: IsInteger,
Q: IsInteger,
R: IsInteger,
S: IsInteger,
P: Mul<Q, Output = S, Overflow = R>
{}

#[test]
fn multiplication() {
    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(1); let a0_2 = parse_integer!(0); let a0_3 = parse_integer!(1); mul_equal(&a0_0,&a0_1, &a0_2, &a0_3);
    let a1_0 = parse_integer!(2); let a1_1 = parse_integer!(38); let a1_2 = parse_integer!(0); let a1_3 = parse_integer!(76); mul_equal(&a1_0,&a1_1, &a1_2, &a1_3);
    let a2_0 = parse_integer!(5); let a2_1 = parse_integer!(38); let a2_2 = parse_integer!(0); let a2_3 = parse_integer!(190); mul_equal(&a2_0,&a2_1, &a2_2, &a2_3);
    let a3_0 = parse_integer!(9); let a3_1 = parse_integer!(52); let a3_2 = parse_integer!(0); let a3_3 = parse_integer!(468); mul_equal(&a3_0,&a3_1, &a3_2, &a3_3);
    let a4_0 = parse_integer!(1); let a4_1 = parse_integer!(1950); let a4_2 = parse_integer!(0); let a4_3 = parse_integer!(1950); mul_equal(&a4_0,&a4_1, &a4_2, &a4_3);
    let a5_0 = parse_integer!(2); let a5_1 = parse_integer!(1314); let a5_2 = parse_integer!(0); let a5_3 = parse_integer!(2628); mul_equal(&a5_0,&a5_1, &a5_2, &a5_3);
    let a6_0 = parse_integer!(28); let a6_1 = parse_integer!(161); let a6_2 = parse_integer!(0); let a6_3 = parse_integer!(4508); mul_equal(&a6_0,&a6_1, &a6_2, &a6_3);
    let a7_0 = parse_integer!(1); let a7_1 = parse_integer!(10310); let a7_2 = parse_integer!(0); let a7_3 = parse_integer!(10310); mul_equal(&a7_0,&a7_1, &a7_2, &a7_3);
    let a8_0 = parse_integer!(37); let a8_1 = parse_integer!(598); let a8_2 = parse_integer!(0); let a8_3 = parse_integer!(22126); mul_equal(&a8_0,&a8_1, &a8_2, &a8_3);
    let a9_0 = parse_integer!(15); let a9_1 = parse_integer!(14806); let a9_2 = parse_integer!(0); let a9_3 = parse_integer!(222090); mul_equal(&a9_0,&a9_1, &a9_2, &a9_3);
    let a10_0 = parse_integer!(73); let a10_1 = parse_integer!(3539); let a10_2 = parse_integer!(0); let a10_3 = parse_integer!(258347); mul_equal(&a10_0,&a10_1, &a10_2, &a10_3);
    let a11_0 = parse_integer!(8); let a11_1 = parse_integer!(73494); let a11_2 = parse_integer!(0); let a11_3 = parse_integer!(587952); mul_equal(&a11_0,&a11_1, &a11_2, &a11_3);
    let a12_0 = parse_integer!(26); let a12_1 = parse_integer!(71513); let a12_2 = parse_integer!(0); let a12_3 = parse_integer!(1859338); mul_equal(&a12_0,&a12_1, &a12_2, &a12_3);
    let a13_0 = parse_integer!(3); let a13_1 = parse_integer!(1074568); let a13_2 = parse_integer!(0); let a13_3 = parse_integer!(3223704); mul_equal(&a13_0,&a13_1, &a13_2, &a13_3);
    let a14_0 = parse_integer!(60); let a14_1 = parse_integer!(60060); let a14_2 = parse_integer!(0); let a14_3 = parse_integer!(3603600); mul_equal(&a14_0,&a14_1, &a14_2, &a14_3);
    let a15_0 = parse_integer!(32); let a15_1 = parse_integer!(156133); let a15_2 = parse_integer!(0); let a15_3 = parse_integer!(4996256); mul_equal(&a15_0,&a15_1, &a15_2, &a15_3);
    let a16_0 = parse_integer!(36); let a16_1 = parse_integer!(283357); let a16_2 = parse_integer!(0); let a16_3 = parse_integer!(10200852); mul_equal(&a16_0,&a16_1, &a16_2, &a16_3);
    let a17_0 = parse_integer!(47); let a17_1 = parse_integer!(259704); let a17_2 = parse_integer!(0); let a17_3 = parse_integer!(12206088); mul_equal(&a17_0,&a17_1, &a17_2, &a17_3);
    let a18_0 = parse_integer!(174); let a18_1 = parse_integer!(110610); let a18_2 = parse_integer!(0); let a18_3 = parse_integer!(19246140); mul_equal(&a18_0,&a18_1, &a18_2, &a18_3);
    let a19_0 = parse_integer!(61); let a19_1 = parse_integer!(467781); let a19_2 = parse_integer!(0); let a19_3 = parse_integer!(28534641); mul_equal(&a19_0,&a19_1, &a19_2, &a19_3);
    let a20_0 = parse_integer!(5); let a20_1 = parse_integer!(7170654); let a20_2 = parse_integer!(0); let a20_3 = parse_integer!(35853270); mul_equal(&a20_0,&a20_1, &a20_2, &a20_3);
    let a21_0 = parse_integer!(2622); let a21_1 = parse_integer!(213381); let a21_2 = parse_integer!(0); let a21_3 = parse_integer!(559484982); mul_equal(&a21_0,&a21_1, &a21_2, &a21_3);
    let a22_0 = parse_integer!(67); let a22_1 = parse_integer!(8429107); let a22_2 = parse_integer!(0); let a22_3 = parse_integer!(564750169); mul_equal(&a22_0,&a22_1, &a22_2, &a22_3);
    let a23_0 = parse_integer!(1374); let a23_1 = parse_integer!(753292); let a23_2 = parse_integer!(0); let a23_3 = parse_integer!(1035023208); mul_equal(&a23_0,&a23_1, &a23_2, &a23_3);
    let a24_0 = parse_integer!(1); let a24_1 = parse_integer!(1140984541); let a24_2 = parse_integer!(0); let a24_3 = parse_integer!(1140984541); mul_equal(&a24_0,&a24_1, &a24_2, &a24_3);
    let a25_0 = parse_integer!(87); let a25_1 = parse_integer!(25489570); let a25_2 = parse_integer!(0); let a25_3 = parse_integer!(2217592590); mul_equal(&a25_0,&a25_1, &a25_2, &a25_3);
    let a26_0 = parse_integer!(172); let a26_1 = parse_integer!(59565971); let a26_2 = parse_integer!(2); let a26_3 = parse_integer!(1655412420); mul_equal(&a26_0,&a26_1, &a26_2, &a26_3);
    let a27_0 = parse_integer!(7); let a27_1 = parse_integer!(1694336315); let a27_2 = parse_integer!(2); let a27_3 = parse_integer!(3270419613); mul_equal(&a27_0,&a27_1, &a27_2, &a27_3);
    let a28_0 = parse_integer!(3487); let a28_1 = parse_integer!(3451034); let a28_2 = parse_integer!(2); let a28_3 = parse_integer!(3443820966); mul_equal(&a28_0,&a28_1, &a28_2, &a28_3);
    let a29_0 = parse_integer!(101254); let a29_1 = parse_integer!(149300); let a29_2 = parse_integer!(3); let a29_3 = parse_integer!(2232320312); mul_equal(&a29_0,&a29_1, &a29_2, &a29_3);
    let a30_0 = parse_integer!(121); let a30_1 = parse_integer!(257871831); let a30_2 = parse_integer!(7); let a30_3 = parse_integer!(1137720479); mul_equal(&a30_0,&a30_1, &a30_2, &a30_3);
    let a31_0 = parse_integer!(752); let a31_1 = parse_integer!(88957215); let a31_2 = parse_integer!(15); let a31_3 = parse_integer!(2471316240); mul_equal(&a31_0,&a31_1, &a31_2, &a31_3);
    let a32_0 = parse_integer!(8171); let a32_1 = parse_integer!(41153307); let a32_2 = parse_integer!(78); let a32_3 = parse_integer!(1256222409); mul_equal(&a32_0,&a32_1, &a32_2, &a32_3);
    let a33_0 = parse_integer!(166); let a33_1 = parse_integer!(2078683947); let a33_2 = parse_integer!(80); let a33_3 = parse_integer!(1464151522); mul_equal(&a33_0,&a33_1, &a33_2, &a33_3);
    let a34_0 = parse_integer!(42785); let a34_1 = parse_integer!(21536501); let a34_2 = parse_integer!(214); let a34_3 = parse_integer!(2316193941); mul_equal(&a34_0,&a34_1, &a34_2, &a34_3);
    let a35_0 = parse_integer!(229799); let a35_1 = parse_integer!(5360660); let a35_2 = parse_integer!(286); let a35_3 = parse_integer!(3513660684); mul_equal(&a35_0,&a35_1, &a35_2, &a35_3);
    let a36_0 = parse_integer!(9844); let a36_1 = parse_integer!(339107381); let a36_2 = parse_integer!(777); let a36_3 = parse_integer!(983469572); mul_equal(&a36_0,&a36_1, &a36_2, &a36_3);
    let a37_0 = parse_integer!(1864); let a37_1 = parse_integer!(2087948069); let a37_2 = parse_integer!(906); let a37_3 = parse_integer!(694830440); mul_equal(&a37_0,&a37_1, &a37_2, &a37_3);
    let a38_0 = parse_integer!(370407); let a38_1 = parse_integer!(12458533); let a38_2 = parse_integer!(1074); let a38_3 = parse_integer!(1932957027); mul_equal(&a38_0,&a38_1, &a38_2, &a38_3);
    let a39_0 = parse_integer!(222389); let a39_1 = parse_integer!(53047285); let a39_2 = parse_integer!(2746); let a39_3 = parse_integer!(3152469049); mul_equal(&a39_0,&a39_1, &a39_2, &a39_3);
    let a40_0 = parse_integer!(11319); let a40_1 = parse_integer!(1498617499); let a40_2 = parse_integer!(3949); let a40_3 = parse_integer!(2025619277); mul_equal(&a40_0,&a40_1, &a40_2, &a40_3);
    let a41_0 = parse_integer!(9331); let a41_1 = parse_integer!(2674236400); let a41_2 = parse_integer!(5809); let a41_3 = parse_integer!(3834825936); mul_equal(&a41_0,&a41_1, &a41_2, &a41_3);
    let a42_0 = parse_integer!(711264); let a42_1 = parse_integer!(49154391); let a42_2 = parse_integer!(8140); let a42_3 = parse_integer!(714970784); mul_equal(&a42_0,&a42_1, &a42_2, &a42_3);
    let a43_0 = parse_integer!(98215); let a43_1 = parse_integer!(1716949082); let a43_2 = parse_integer!(39262); let a43_3 = parse_integer!(1148113078); mul_equal(&a43_0,&a43_1, &a43_2, &a43_3);
    let a44_0 = parse_integer!(6815395); let a44_1 = parse_integer!(88225129); let a44_2 = parse_integer!(139998); let a44_3 = parse_integer!(2271555547); mul_equal(&a44_0,&a44_1, &a44_2, &a44_3);
    let a45_0 = parse_integer!(27577502); let a45_1 = parse_integer!(39696924); let a45_2 = parse_integer!(254889); let a45_3 = parse_integer!(2081893704); mul_equal(&a45_0,&a45_1, &a45_2, &a45_3);
    let a46_0 = parse_integer!(26632764); let a46_1 = parse_integer!(244378648); let a46_2 = parse_integer!(1515373); let a46_3 = parse_integer!(1382581664); mul_equal(&a46_0,&a46_1, &a46_2, &a46_3);
    let a47_0 = parse_integer!(14860314); let a47_1 = parse_integer!(2683652142); let a47_2 = parse_integer!(9285265); let a47_3 = parse_integer!(3987199148); mul_equal(&a47_0,&a47_1, &a47_2, &a47_3);
    let a48_0 = parse_integer!(103895386); let a48_1 = parse_integer!(2964958777); let a48_2 = parse_integer!(71722440); let a48_3 = parse_integer!(2421180682); mul_equal(&a48_0,&a48_1, &a48_2, &a48_3);
    let a49_0 = parse_integer!(737585385); let a49_1 = parse_integer!(3810507542); let a49_2 = parse_integer!(654387910); let a49_3 = parse_integer!(63682310); mul_equal(&a49_0,&a49_1, &a49_2, &a49_3);
    
}