extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
use quote::quote;

// Debug :)
fn print_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// Generate a compile error to output struct name
#[proc_macro]
pub fn parse_integer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let input_value = input.base10_digits();
    let mut num = match input_value.parse::<u32>() {
        Ok(value) => value,
        Err(_) => panic!("Input is not of correct type: {}", input_value),
    };

    // panic!("Parsed value: {}, type: {}", num, print_type_of(&num));
    let mut hex_digits: Vec<String> = Vec::new();

    while num > 0 {
        let hex_digit = (num & 0xF).to_string();
        hex_digits.push(format!("_{}", hex_digit));
        num >>= 4;
    }

    while hex_digits.len() < 8 {
        hex_digits.push(String::from("_0"));
    }

    let result = hex_digits.join(", ");

    let type_name = format!("Integer<{}>", result);

    panic!("{}", type_name);
}