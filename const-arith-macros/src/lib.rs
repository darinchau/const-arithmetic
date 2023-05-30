extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
use quote::quote;

// Debug :)
fn print_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn int_to_hex_codes(input: LitInt) -> Vec<String> {
    let input_value = input.base10_digits();
    let parsed_value = match input_value.parse::<u32>() {
        Ok(value) => value,
        Err(_) => panic!("Input is not of correct type: {}", input_value),
    };

    // panic!("Parsed value: {}, type: {}", num, print_type_of(&num));
    let mut hex_digits: Vec<String> = Vec::new();
    let mut num = parsed_value;

    while num > 0 {
        let hex_digit = num & 0xF;
        hex_digits.push(format!("_{:X}", hex_digit));
        num >>= 4;
    }

    while hex_digits.len() < 8 {
        hex_digits.push(String::from("_0"));
    }

    return hex_digits;
}

/// Parses a literal integer into its typed counterpart. Panics inline if the input is not a u32
#[proc_macro]
pub fn parse_integer(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as LitInt);
    let hex_digits = int_to_hex_codes(input);
    let joined = hex_digits.join(", ");

    // let result = format!("Integer::<{}> {{ _m0: PhantomData, _m1: PhantomData, _m2: PhantomData, _m3: PhantomData, _m4: PhantomData, _m5: PhantomData, _m6: PhantomData, _m7: PhantomData }}", hex_digits.join(", "));
    let result = format!("TypedInteger::<{}>::new()", joined);

    // panic!("{}", result);
    result.parse().unwrap()
}

/// Assert the typed integer equals the number you provided
#[proc_macro]
pub fn typed_assert_eq_inner(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as LitInt);
    // Use an iterator so that strings does not have to be copied
    let mut hex_digits = int_to_hex_codes(input).into_iter();
    let s0 = hex_digits.next().unwrap();
    let s1 = hex_digits.next().unwrap();
    let s2 = hex_digits.next().unwrap();
    let s3 = hex_digits.next().unwrap();
    let s4 = hex_digits.next().unwrap();
    let s5 = hex_digits.next().unwrap();
    let s6 = hex_digits.next().unwrap();
    let s7 = hex_digits.next().unwrap();
    let result = format!(r"
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
        S0: HexAssertEqual<{}>,
        S1: HexAssertEqual<{}>,
        S2: HexAssertEqual<{}>,
        S3: HexAssertEqual<{}>,
        S4: HexAssertEqual<{}>,
        S5: HexAssertEqual<{}>,
        S6: HexAssertEqual<{}>,
        S7: HexAssertEqual<{}>,
    {{}}", s0, s1, s2, s3, s4, s5, s6, s7);
    result.parse().unwrap()
}
