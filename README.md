# const_arithmetic

[![Crates.io](https://img.shields.io/crates/v/const_arithmetic.svg)](https://crates.io/crates/const_arithmetic)
[![Documentation](https://docs.rs/const_arithmetic/badge.svg)](https://docs.rs/const_arithmetic)

`const_arithmetic` is a crate dedicated to exploiting Rust's powerful compiler and type generic system to perform integer arithmetics at compile time. This works on stable to circumvent (sort of) `#[generic_const_exprs]`

## Usage

Add the `const_arithmetic` crate as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
const_arithmetic = "1.0.3"
```

Import the traits and types from the `const_arithmetic` crate:

```rust
use const_arithmetic::*;
```

## Example:

Let's say we want to verify `6 x 4 = 24`. Here is the code snippet that does nothing if the above statement is true and fails to compile if the above statement is false.

```rust
use const_arithmetic::*;
let a = parse_integer!(6);
let b = parse_integer!(4);
let c = parse_integer!(24);

fn verify_me<P, Q, R>(_p: P, _q: Q, _r: R) where
P: IsInteger,
Q: IsInteger,
R: IsInteger,
P: TypedMul<Q, Output = R>
{}

verify_me(a, b, c);
```

Accordingly, this fails to compile:
```rust
use const_arithmetic::*;
let a = parse_integer!(6);
let b = parse_integer!(5);
let c = parse_integer!(25);

fn verify_me<P, Q, R>(_p: P, _q: Q, _r: R) where
P: IsInteger,
Q: IsInteger,
R: IsInteger,
P: TypedMul<Q, Output = R>
{}

verify_me(a, b, c); // Compilation error
```

## Documentation

Please refer to the [API documentation](https://docs.rs/const_arithmetic) for detailed information and more examples.

## Contributing

Please open a PR if you want to change anything. Contributions are welcome.

## License

This crate is distributed under the terms of the [MIT License](LICENSE).
