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
    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(5); let a0_2 = parse_integer!(4); sub_equal(&a0_0, &a0_1, &a0_2);
    let a1_0 = parse_integer!(1); let a1_1 = parse_integer!(7); let a1_2 = parse_integer!(6); sub_equal(&a1_0, &a1_1, &a1_2);
    let a2_0 = parse_integer!(3); let a2_1 = parse_integer!(11); let a2_2 = parse_integer!(8); sub_equal(&a2_0, &a2_1, &a2_2);
    let a3_0 = parse_integer!(67); let a3_1 = parse_integer!(132); let a3_2 = parse_integer!(65); sub_equal(&a3_0, &a3_1, &a3_2);
    let a4_0 = parse_integer!(1); let a4_1 = parse_integer!(108); let a4_2 = parse_integer!(107); sub_equal(&a4_0, &a4_1, &a4_2);
    let a5_0 = parse_integer!(3); let a5_1 = parse_integer!(315); let a5_2 = parse_integer!(312); sub_equal(&a5_0, &a5_1, &a5_2);
    let a6_0 = parse_integer!(12); let a6_1 = parse_integer!(432); let a6_2 = parse_integer!(420); sub_equal(&a6_0, &a6_1, &a6_2);
    let a7_0 = parse_integer!(24); let a7_1 = parse_integer!(661); let a7_2 = parse_integer!(637); sub_equal(&a7_0, &a7_1, &a7_2);
    let a8_0 = parse_integer!(24); let a8_1 = parse_integer!(2959); let a8_2 = parse_integer!(2935); sub_equal(&a8_0, &a8_1, &a8_2);
    let a9_0 = parse_integer!(29); let a9_1 = parse_integer!(3527); let a9_2 = parse_integer!(3498); sub_equal(&a9_0, &a9_1, &a9_2);
    let a10_0 = parse_integer!(13); let a10_1 = parse_integer!(3911); let a10_2 = parse_integer!(3898); sub_equal(&a10_0, &a10_1, &a10_2);
    let a11_0 = parse_integer!(1); let a11_1 = parse_integer!(16536); let a11_2 = parse_integer!(16535); sub_equal(&a11_0, &a11_1, &a11_2);
    let a12_0 = parse_integer!(3); let a12_1 = parse_integer!(70896); let a12_2 = parse_integer!(70893); sub_equal(&a12_0, &a12_1, &a12_2);
    let a13_0 = parse_integer!(5473); let a13_1 = parse_integer!(113302); let a13_2 = parse_integer!(107829); sub_equal(&a13_0, &a13_1, &a13_2);
    let a14_0 = parse_integer!(259); let a14_1 = parse_integer!(140438); let a14_2 = parse_integer!(140179); sub_equal(&a14_0, &a14_1, &a14_2);
    let a15_0 = parse_integer!(1); let a15_1 = parse_integer!(275639); let a15_2 = parse_integer!(275638); sub_equal(&a15_0, &a15_1, &a15_2);
    let a16_0 = parse_integer!(128); let a16_1 = parse_integer!(390565); let a16_2 = parse_integer!(390437); sub_equal(&a16_0, &a16_1, &a16_2);
    let a17_0 = parse_integer!(2); let a17_1 = parse_integer!(504213); let a17_2 = parse_integer!(504211); sub_equal(&a17_0, &a17_1, &a17_2);
    let a18_0 = parse_integer!(5); let a18_1 = parse_integer!(632729); let a18_2 = parse_integer!(632724); sub_equal(&a18_0, &a18_1, &a18_2);
    let a19_0 = parse_integer!(595144); let a19_1 = parse_integer!(1408719); let a19_2 = parse_integer!(813575); sub_equal(&a19_0, &a19_1, &a19_2);
    let a20_0 = parse_integer!(55); let a20_1 = parse_integer!(1978473); let a20_2 = parse_integer!(1978418); sub_equal(&a20_0, &a20_1, &a20_2);
    let a21_0 = parse_integer!(4779); let a21_1 = parse_integer!(2083626); let a21_2 = parse_integer!(2078847); sub_equal(&a21_0, &a21_1, &a21_2);
    let a22_0 = parse_integer!(20724); let a22_1 = parse_integer!(3678339); let a22_2 = parse_integer!(3657615); sub_equal(&a22_0, &a22_1, &a22_2);
    let a23_0 = parse_integer!(63); let a23_1 = parse_integer!(6945452); let a23_2 = parse_integer!(6945389); sub_equal(&a23_0, &a23_1, &a23_2);
    let a24_0 = parse_integer!(3037); let a24_1 = parse_integer!(11559212); let a24_2 = parse_integer!(11556175); sub_equal(&a24_0, &a24_1, &a24_2);
    let a25_0 = parse_integer!(1); let a25_1 = parse_integer!(20084527); let a25_2 = parse_integer!(20084526); sub_equal(&a25_0, &a25_1, &a25_2);
    let a26_0 = parse_integer!(3); let a26_1 = parse_integer!(30638746); let a26_2 = parse_integer!(30638743); sub_equal(&a26_0, &a26_1, &a26_2);
    let a27_0 = parse_integer!(24487); let a27_1 = parse_integer!(31374601); let a27_2 = parse_integer!(31350114); sub_equal(&a27_0, &a27_1, &a27_2);
    let a28_0 = parse_integer!(13); let a28_1 = parse_integer!(51403327); let a28_2 = parse_integer!(51403314); sub_equal(&a28_0, &a28_1, &a28_2);
    let a29_0 = parse_integer!(60263142); let a29_1 = parse_integer!(201176371); let a29_2 = parse_integer!(140913229); sub_equal(&a29_0, &a29_1, &a29_2);
    let a30_0 = parse_integer!(3456469); let a30_1 = parse_integer!(152261864); let a30_2 = parse_integer!(148805395); sub_equal(&a30_0, &a30_1, &a30_2);
    let a31_0 = parse_integer!(5); let a31_1 = parse_integer!(150746089); let a31_2 = parse_integer!(150746084); sub_equal(&a31_0, &a31_1, &a31_2);
    let a32_0 = parse_integer!(239); let a32_1 = parse_integer!(152522343); let a32_2 = parse_integer!(152522104); sub_equal(&a32_0, &a32_1, &a32_2);
    let a33_0 = parse_integer!(4808); let a33_1 = parse_integer!(216351142); let a33_2 = parse_integer!(216346334); sub_equal(&a33_0, &a33_1, &a33_2);
    let a34_0 = parse_integer!(1); let a34_1 = parse_integer!(234719976); let a34_2 = parse_integer!(234719975); sub_equal(&a34_0, &a34_1, &a34_2);
    let a35_0 = parse_integer!(9791); let a35_1 = parse_integer!(277956566); let a35_2 = parse_integer!(277946775); sub_equal(&a35_0, &a35_1, &a35_2);
    let a36_0 = parse_integer!(40811555); let a36_1 = parse_integer!(423594305); let a36_2 = parse_integer!(382782750); sub_equal(&a36_0, &a36_1, &a36_2);
    let a37_0 = parse_integer!(3); let a37_1 = parse_integer!(411773309); let a37_2 = parse_integer!(411773306); sub_equal(&a37_0, &a37_1, &a37_2);
    let a38_0 = parse_integer!(5223); let a38_1 = parse_integer!(531378778); let a38_2 = parse_integer!(531373555); sub_equal(&a38_0, &a38_1, &a38_2);
    let a39_0 = parse_integer!(63); let a39_1 = parse_integer!(542286800); let a39_2 = parse_integer!(542286737); sub_equal(&a39_0, &a39_1, &a39_2);
    let a40_0 = parse_integer!(1); let a40_1 = parse_integer!(656709319); let a40_2 = parse_integer!(656709318); sub_equal(&a40_0, &a40_1, &a40_2);
    let a41_0 = parse_integer!(7860); let a41_1 = parse_integer!(705821025); let a41_2 = parse_integer!(705813165); sub_equal(&a41_0, &a41_1, &a41_2);
    let a42_0 = parse_integer!(405); let a42_1 = parse_integer!(967238233); let a42_2 = parse_integer!(967237828); sub_equal(&a42_0, &a42_1, &a42_2);
    let a43_0 = parse_integer!(104385211); let a43_1 = parse_integer!(1310650010); let a43_2 = parse_integer!(1206264799); sub_equal(&a43_0, &a43_1, &a43_2);
    let a44_0 = parse_integer!(5670906); let a44_1 = parse_integer!(1624039747); let a44_2 = parse_integer!(1618368841); sub_equal(&a44_0, &a44_1, &a44_2);
    let a45_0 = parse_integer!(230175); let a45_1 = parse_integer!(1695734517); let a45_2 = parse_integer!(1695504342); sub_equal(&a45_0, &a45_1, &a45_2);
    let a46_0 = parse_integer!(103233); let a46_1 = parse_integer!(1849087582); let a46_2 = parse_integer!(1848984349); sub_equal(&a46_0, &a46_1, &a46_2);
    let a47_0 = parse_integer!(2631505); let a47_1 = parse_integer!(2004715957); let a47_2 = parse_integer!(2002084452); sub_equal(&a47_0, &a47_1, &a47_2);
    let a48_0 = parse_integer!(2866); let a48_1 = parse_integer!(2051155627); let a48_2 = parse_integer!(2051152761); sub_equal(&a48_0, &a48_1, &a48_2);
    let a49_0 = parse_integer!(40); let a49_1 = parse_integer!(2088052255); let a49_2 = parse_integer!(2088052215); sub_equal(&a49_0, &a49_1, &a49_2);
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
    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(38); let a0_2 = parse_integer!(0); let a0_3 = parse_integer!(38); mul_equal(&a0_0,&a0_1, &a0_2, &a0_3);
    let a1_0 = parse_integer!(5); let a1_1 = parse_integer!(77); let a1_2 = parse_integer!(0); let a1_3 = parse_integer!(385); mul_equal(&a1_0,&a1_1, &a1_2, &a1_3);
    let a2_0 = parse_integer!(12); let a2_1 = parse_integer!(51); let a2_2 = parse_integer!(0); let a2_3 = parse_integer!(612); mul_equal(&a2_0,&a2_1, &a2_2, &a2_3);
    let a3_0 = parse_integer!(7); let a3_1 = parse_integer!(1724); let a3_2 = parse_integer!(0); let a3_3 = parse_integer!(12068); mul_equal(&a3_0,&a3_1, &a3_2, &a3_3);
    let a4_0 = parse_integer!(93); let a4_1 = parse_integer!(404); let a4_2 = parse_integer!(0); let a4_3 = parse_integer!(37572); mul_equal(&a4_0,&a4_1, &a4_2, &a4_3);
    let a5_0 = parse_integer!(1); let a5_1 = parse_integer!(47904); let a5_2 = parse_integer!(0); let a5_3 = parse_integer!(47904); mul_equal(&a5_0,&a5_1, &a5_2, &a5_3);
    let a6_0 = parse_integer!(9); let a6_1 = parse_integer!(8212); let a6_2 = parse_integer!(0); let a6_3 = parse_integer!(73908); mul_equal(&a6_0,&a6_1, &a6_2, &a6_3);
    let a7_0 = parse_integer!(122); let a7_1 = parse_integer!(670); let a7_2 = parse_integer!(0); let a7_3 = parse_integer!(81740); mul_equal(&a7_0,&a7_1, &a7_2, &a7_3);
    let a8_0 = parse_integer!(180); let a8_1 = parse_integer!(946); let a8_2 = parse_integer!(0); let a8_3 = parse_integer!(170280); mul_equal(&a8_0,&a8_1, &a8_2, &a8_3);
    let a9_0 = parse_integer!(17); let a9_1 = parse_integer!(13036); let a9_2 = parse_integer!(0); let a9_3 = parse_integer!(221612); mul_equal(&a9_0,&a9_1, &a9_2, &a9_3);
    let a10_0 = parse_integer!(1); let a10_1 = parse_integer!(1034901); let a10_2 = parse_integer!(0); let a10_3 = parse_integer!(1034901); mul_equal(&a10_0,&a10_1, &a10_2, &a10_3);
    let a11_0 = parse_integer!(47); let a11_1 = parse_integer!(25400); let a11_2 = parse_integer!(0); let a11_3 = parse_integer!(1193800); mul_equal(&a11_0,&a11_1, &a11_2, &a11_3);
    let a12_0 = parse_integer!(10); let a12_1 = parse_integer!(264889); let a12_2 = parse_integer!(0); let a12_3 = parse_integer!(2648890); mul_equal(&a12_0,&a12_1, &a12_2, &a12_3);
    let a13_0 = parse_integer!(434); let a13_1 = parse_integer!(16368); let a13_2 = parse_integer!(0); let a13_3 = parse_integer!(7103712); mul_equal(&a13_0,&a13_1, &a13_2, &a13_3);
    let a14_0 = parse_integer!(2); let a14_1 = parse_integer!(5450924); let a14_2 = parse_integer!(0); let a14_3 = parse_integer!(10901848); mul_equal(&a14_0,&a14_1, &a14_2, &a14_3);
    let a15_0 = parse_integer!(1150); let a15_1 = parse_integer!(13956); let a15_2 = parse_integer!(0); let a15_3 = parse_integer!(16049400); mul_equal(&a15_0,&a15_1, &a15_2, &a15_3);
    let a16_0 = parse_integer!(431); let a16_1 = parse_integer!(40669); let a16_2 = parse_integer!(0); let a16_3 = parse_integer!(17528339); mul_equal(&a16_0,&a16_1, &a16_2, &a16_3);
    let a17_0 = parse_integer!(3332); let a17_1 = parse_integer!(5782); let a17_2 = parse_integer!(0); let a17_3 = parse_integer!(19265624); mul_equal(&a17_0,&a17_1, &a17_2, &a17_3);
    let a18_0 = parse_integer!(3374); let a18_1 = parse_integer!(8247); let a18_2 = parse_integer!(0); let a18_3 = parse_integer!(27825378); mul_equal(&a18_0,&a18_1, &a18_2, &a18_3);
    let a19_0 = parse_integer!(234); let a19_1 = parse_integer!(338680); let a19_2 = parse_integer!(0); let a19_3 = parse_integer!(79251120); mul_equal(&a19_0,&a19_1, &a19_2, &a19_3);
    let a20_0 = parse_integer!(18); let a20_1 = parse_integer!(5431843); let a20_2 = parse_integer!(0); let a20_3 = parse_integer!(97773174); mul_equal(&a20_0,&a20_1, &a20_2, &a20_3);
    let a21_0 = parse_integer!(3942); let a21_1 = parse_integer!(27584); let a21_2 = parse_integer!(0); let a21_3 = parse_integer!(108736128); mul_equal(&a21_0,&a21_1, &a21_2, &a21_3);
    let a22_0 = parse_integer!(5); let a22_1 = parse_integer!(23979462); let a22_2 = parse_integer!(0); let a22_3 = parse_integer!(119897310); mul_equal(&a22_0,&a22_1, &a22_2, &a22_3);
    let a23_0 = parse_integer!(3152); let a23_1 = parse_integer!(38287); let a23_2 = parse_integer!(0); let a23_3 = parse_integer!(120680624); mul_equal(&a23_0,&a23_1, &a23_2, &a23_3);
    let a24_0 = parse_integer!(23); let a24_1 = parse_integer!(8797564); let a24_2 = parse_integer!(0); let a24_3 = parse_integer!(202343972); mul_equal(&a24_0,&a24_1, &a24_2, &a24_3);
    let a25_0 = parse_integer!(48); let a25_1 = parse_integer!(5331543); let a25_2 = parse_integer!(0); let a25_3 = parse_integer!(255914064); mul_equal(&a25_0,&a25_1, &a25_2, &a25_3);
    let a26_0 = parse_integer!(538); let a26_1 = parse_integer!(555791); let a26_2 = parse_integer!(0); let a26_3 = parse_integer!(299015558); mul_equal(&a26_0,&a26_1, &a26_2, &a26_3);
    let a27_0 = parse_integer!(1132); let a27_1 = parse_integer!(472250); let a27_2 = parse_integer!(0); let a27_3 = parse_integer!(534587000); mul_equal(&a27_0,&a27_1, &a27_2, &a27_3);
    let a28_0 = parse_integer!(179); let a28_1 = parse_integer!(6533552); let a28_2 = parse_integer!(0); let a28_3 = parse_integer!(1169505808); mul_equal(&a28_0,&a28_1, &a28_2, &a28_3);
    let a29_0 = parse_integer!(5); let a29_1 = parse_integer!(413088807); let a29_2 = parse_integer!(0); let a29_3 = parse_integer!(2065444035); mul_equal(&a29_0,&a29_1, &a29_2, &a29_3);
    let a30_0 = parse_integer!(3); let a30_1 = parse_integer!(1473608391); let a30_2 = parse_integer!(1); let a30_3 = parse_integer!(125857877); mul_equal(&a30_0,&a30_1, &a30_2, &a30_3);
    let a31_0 = parse_integer!(34446); let a31_1 = parse_integer!(168952); let a31_2 = parse_integer!(1); let a31_3 = parse_integer!(1524753296); mul_equal(&a31_0,&a31_1, &a31_2, &a31_3);
    let a32_0 = parse_integer!(49); let a32_1 = parse_integer!(155695436); let a32_2 = parse_integer!(1); let a32_3 = parse_integer!(3334109068); mul_equal(&a32_0,&a32_1, &a32_2, &a32_3);
    let a33_0 = parse_integer!(239); let a33_1 = parse_integer!(86637124); let a33_2 = parse_integer!(4); let a33_3 = parse_integer!(3526403452); mul_equal(&a33_0,&a33_1, &a33_2, &a33_3);
    let a34_0 = parse_integer!(844); let a34_1 = parse_integer!(24630936); let a34_2 = parse_integer!(4); let a34_3 = parse_integer!(3608640800); mul_equal(&a34_0,&a34_1, &a34_2, &a34_3);
    let a35_0 = parse_integer!(7); let a35_1 = parse_integer!(3435465236); let a35_2 = parse_integer!(5); let a35_3 = parse_integer!(2573420172); mul_equal(&a35_0,&a35_1, &a35_2, &a35_3);
    let a36_0 = parse_integer!(546); let a36_1 = parse_integer!(59537422); let a36_2 = parse_integer!(7); let a36_3 = parse_integer!(2442661340); mul_equal(&a36_0,&a36_1, &a36_2, &a36_3);
    let a37_0 = parse_integer!(3647); let a37_1 = parse_integer!(13942606); let a37_2 = parse_integer!(11); let a37_3 = parse_integer!(3604043826); mul_equal(&a37_0,&a37_1, &a37_2, &a37_3);
    let a38_0 = parse_integer!(66); let a38_1 = parse_integer!(3220170934); let a38_2 = parse_integer!(49); let a38_3 = parse_integer!(2077884140); mul_equal(&a38_0,&a38_1, &a38_2, &a38_3);
    let a39_0 = parse_integer!(2936); let a39_1 = parse_integer!(82571183); let a39_2 = parse_integer!(56); let a39_3 = parse_integer!(1910824712); mul_equal(&a39_0,&a39_1, &a39_2, &a39_3);
    let a40_0 = parse_integer!(2582); let a40_1 = parse_integer!(460540139); let a40_2 = parse_integer!(276); let a40_3 = parse_integer!(3703665202); mul_equal(&a40_0,&a40_1, &a40_2, &a40_3);
    let a41_0 = parse_integer!(1115059); let a41_1 = parse_integer!(1951561); let a41_2 = parse_integer!(506); let a41_3 = parse_integer!(2852205323); mul_equal(&a41_0,&a41_1, &a41_2, &a41_3);
    let a42_0 = parse_integer!(25954); let a42_1 = parse_integer!(138375816); let a42_2 = parse_integer!(836); let a42_3 = parse_integer!(813269008); mul_equal(&a42_0,&a42_1, &a42_2, &a42_3);
    let a43_0 = parse_integer!(6875); let a43_1 = parse_integer!(929581407); let a43_2 = parse_integer!(1487); let a43_3 = parse_integer!(4255803973); mul_equal(&a43_0,&a43_1, &a43_2, &a43_3);
    let a44_0 = parse_integer!(3711); let a44_1 = parse_integer!(1815486734); let a44_2 = parse_integer!(1568); let a44_3 = parse_integer!(2762549746); mul_equal(&a44_0,&a44_1, &a44_2, &a44_3);
    let a45_0 = parse_integer!(8016); let a45_1 = parse_integer!(1201190634); let a45_2 = parse_integer!(2241); let a45_3 = parse_integer!(3722411808); mul_equal(&a45_0,&a45_1, &a45_2, &a45_3);
    let a46_0 = parse_integer!(345190); let a46_1 = parse_integer!(2067623621); let a46_2 = parse_integer!(166176); let a46_3 = parse_integer!(2512352894); mul_equal(&a46_0,&a46_1, &a46_2, &a46_3);
    let a47_0 = parse_integer!(7597147); let a47_1 = parse_integer!(98095753); let a47_2 = parse_integer!(173516); let a47_3 = parse_integer!(2310283955); mul_equal(&a47_0,&a47_1, &a47_2, &a47_3);
    let a48_0 = parse_integer!(4488514); let a48_1 = parse_integer!(976220767); let a48_2 = parse_integer!(1020212); let a48_3 = parse_integer!(3404783486); mul_equal(&a48_0,&a48_1, &a48_2, &a48_3);
    let a49_0 = parse_integer!(30428250); let a49_1 = parse_integer!(2337528154); let a49_2 = parse_integer!(16560519); let a49_3 = parse_integer!(3542163876); mul_equal(&a49_0,&a49_1, &a49_2, &a49_3);    
}


fn is_less<P, Q, R>(a: &P, b: &Q, res: &R) where
P: IsInteger,
Q: IsInteger,
R: Binary,
P: TypedLessThan<Q, Output = R>,
{ }

#[test]
fn comparison() {
    let True: _1 = _1;
    let False: _0 = _0;

    let a0_0 = parse_integer!(1); let a0_1 = parse_integer!(3); is_less(&a0_0, &a0_1, &True); is_less(&a0_1, &a0_0, &False);
    let a1_0 = parse_integer!(1); let a1_1 = parse_integer!(1406); is_less(&a1_0, &a1_1, &True); is_less(&a1_1, &a1_0, &False);
    let a2_0 = parse_integer!(1); let a2_1 = parse_integer!(211920427); is_less(&a2_0, &a2_1, &True); is_less(&a2_1, &a2_0, &False);
    let a3_0 = parse_integer!(3); let a3_1 = parse_integer!(591295195); is_less(&a3_0, &a3_1, &True); is_less(&a3_1, &a3_0, &False);
    let a4_0 = parse_integer!(4); let a4_1 = parse_integer!(98); is_less(&a4_0, &a4_1, &True); is_less(&a4_1, &a4_0, &False);
    let a5_0 = parse_integer!(4); let a5_1 = parse_integer!(15483789); is_less(&a5_0, &a5_1, &True); is_less(&a5_1, &a5_0, &False);
    let a6_0 = parse_integer!(5); let a6_1 = parse_integer!(31); is_less(&a6_0, &a6_1, &True); is_less(&a6_1, &a6_0, &False);
    let a7_0 = parse_integer!(7); let a7_1 = parse_integer!(4675); is_less(&a7_0, &a7_1, &True); is_less(&a7_1, &a7_0, &False);
    let a8_0 = parse_integer!(7); let a8_1 = parse_integer!(18070); is_less(&a8_0, &a8_1, &True); is_less(&a8_1, &a8_0, &False);
    let a9_0 = parse_integer!(12); let a9_1 = parse_integer!(15562); is_less(&a9_0, &a9_1, &True); is_less(&a9_1, &a9_0, &False);
    let a10_0 = parse_integer!(15); let a10_1 = parse_integer!(127892); is_less(&a10_0, &a10_1, &True); is_less(&a10_1, &a10_0, &False);
    let a11_0 = parse_integer!(20); let a11_1 = parse_integer!(51753); is_less(&a11_0, &a11_1, &True); is_less(&a11_1, &a11_0, &False);
    let a12_0 = parse_integer!(22); let a12_1 = parse_integer!(287677); is_less(&a12_0, &a12_1, &True); is_less(&a12_1, &a12_0, &False);
    let a13_0 = parse_integer!(23); let a13_1 = parse_integer!(303); is_less(&a13_0, &a13_1, &True); is_less(&a13_1, &a13_0, &False);
    let a14_0 = parse_integer!(35); let a14_1 = parse_integer!(865); is_less(&a14_0, &a14_1, &True); is_less(&a14_1, &a14_0, &False);
    let a15_0 = parse_integer!(52); let a15_1 = parse_integer!(2643212859); is_less(&a15_0, &a15_1, &True); is_less(&a15_1, &a15_0, &False);
    let a16_0 = parse_integer!(53); let a16_1 = parse_integer!(473781825); is_less(&a16_0, &a16_1, &True); is_less(&a16_1, &a16_0, &False);
    let a17_0 = parse_integer!(65); let a17_1 = parse_integer!(7383120); is_less(&a17_0, &a17_1, &True); is_less(&a17_1, &a17_0, &False);
    let a18_0 = parse_integer!(71); let a18_1 = parse_integer!(1480043888); is_less(&a18_0, &a18_1, &True); is_less(&a18_1, &a18_0, &False);
    let a19_0 = parse_integer!(84); let a19_1 = parse_integer!(39075); is_less(&a19_0, &a19_1, &True); is_less(&a19_1, &a19_0, &False);
    let a20_0 = parse_integer!(91); let a20_1 = parse_integer!(173666); is_less(&a20_0, &a20_1, &True); is_less(&a20_1, &a20_0, &False);
    let a21_0 = parse_integer!(96); let a21_1 = parse_integer!(893227); is_less(&a21_0, &a21_1, &True); is_less(&a21_1, &a21_0, &False);
    let a22_0 = parse_integer!(120); let a22_1 = parse_integer!(120); is_less(&a22_0, &a22_1, &False); is_less(&a22_1, &a22_0, &False);
    let a23_0 = parse_integer!(121); let a23_1 = parse_integer!(3725); is_less(&a23_0, &a23_1, &True); is_less(&a23_1, &a23_0, &False);
    let a24_0 = parse_integer!(231); let a24_1 = parse_integer!(317328); is_less(&a24_0, &a24_1, &True); is_less(&a24_1, &a24_0, &False);
    let a25_0 = parse_integer!(569); let a25_1 = parse_integer!(39057649); is_less(&a25_0, &a25_1, &True); is_less(&a25_1, &a25_0, &False);
    let a26_0 = parse_integer!(1071); let a26_1 = parse_integer!(1071); is_less(&a26_0, &a26_1, &False); is_less(&a26_1, &a26_0, &False);
    let a27_0 = parse_integer!(1088); let a27_1 = parse_integer!(19411); is_less(&a27_0, &a27_1, &True); is_less(&a27_1, &a27_0, &False);
    let a28_0 = parse_integer!(1150); let a28_1 = parse_integer!(8650227); is_less(&a28_0, &a28_1, &True); is_less(&a28_1, &a28_0, &False);
    let a29_0 = parse_integer!(1463); let a29_1 = parse_integer!(2534287941); is_less(&a29_0, &a29_1, &True); is_less(&a29_1, &a29_0, &False);
    let a30_0 = parse_integer!(1757); let a30_1 = parse_integer!(2793338388); is_less(&a30_0, &a30_1, &True); is_less(&a30_1, &a30_0, &False);
    let a31_0 = parse_integer!(2533); let a31_1 = parse_integer!(1137107959); is_less(&a31_0, &a31_1, &True); is_less(&a31_1, &a31_0, &False);
    let a32_0 = parse_integer!(3455); let a32_1 = parse_integer!(1014372384); is_less(&a32_0, &a32_1, &True); is_less(&a32_1, &a32_0, &False);
    let a33_0 = parse_integer!(3744); let a33_1 = parse_integer!(7817); is_less(&a33_0, &a33_1, &True); is_less(&a33_1, &a33_0, &False);
    let a34_0 = parse_integer!(11487); let a34_1 = parse_integer!(785056903); is_less(&a34_0, &a34_1, &True); is_less(&a34_1, &a34_0, &False);
    let a35_0 = parse_integer!(13665); let a35_1 = parse_integer!(1192568655); is_less(&a35_0, &a35_1, &True); is_less(&a35_1, &a35_0, &False);
    let a36_0 = parse_integer!(19119); let a36_1 = parse_integer!(775424); is_less(&a36_0, &a36_1, &True); is_less(&a36_1, &a36_0, &False);
    let a37_0 = parse_integer!(19370); let a37_1 = parse_integer!(36830); is_less(&a37_0, &a37_1, &True); is_less(&a37_1, &a37_0, &False);
    let a38_0 = parse_integer!(26553); let a38_1 = parse_integer!(48042); is_less(&a38_0, &a38_1, &True); is_less(&a38_1, &a38_0, &False);
    let a39_0 = parse_integer!(34869); let a39_1 = parse_integer!(169899487); is_less(&a39_0, &a39_1, &True); is_less(&a39_1, &a39_0, &False);
    let a40_0 = parse_integer!(48895); let a40_1 = parse_integer!(56221); is_less(&a40_0, &a40_1, &True); is_less(&a40_1, &a40_0, &False);
    let a41_0 = parse_integer!(54364); let a41_1 = parse_integer!(311110); is_less(&a41_0, &a41_1, &True); is_less(&a41_1, &a41_0, &False);
    let a42_0 = parse_integer!(57254); let a42_1 = parse_integer!(33144403); is_less(&a42_0, &a42_1, &True); is_less(&a42_1, &a42_0, &False);
    let a43_0 = parse_integer!(114591); let a43_1 = parse_integer!(246316); is_less(&a43_0, &a43_1, &True); is_less(&a43_1, &a43_0, &False);
    let a44_0 = parse_integer!(132250); let a44_1 = parse_integer!(96641092); is_less(&a44_0, &a44_1, &True); is_less(&a44_1, &a44_0, &False);
    let a45_0 = parse_integer!(677155); let a45_1 = parse_integer!(20189056); is_less(&a45_0, &a45_1, &True); is_less(&a45_1, &a45_0, &False);
    let a46_0 = parse_integer!(1712277); let a46_1 = parse_integer!(1095362660); is_less(&a46_0, &a46_1, &True); is_less(&a46_1, &a46_0, &False);
    let a47_0 = parse_integer!(3971257); let a47_1 = parse_integer!(225217508); is_less(&a47_0, &a47_1, &True); is_less(&a47_1, &a47_0, &False);
    let a48_0 = parse_integer!(13481790); let a48_1 = parse_integer!(96516460); is_less(&a48_0, &a48_1, &True); is_less(&a48_1, &a48_0, &False);
    let a49_0 = parse_integer!(124054232); let a49_1 = parse_integer!(199559616); is_less(&a49_0, &a49_1, &True); is_less(&a49_1, &a49_0, &False);    
}