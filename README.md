# currencylib

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![Crates.io](https://img.shields.io/crates/v/currencylib.svg)](https://crates.io/crates/currencylib)
[![Documentation](https://docs.rs/currencylib/badge.svg)](https://docs.rs/currencylib)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/mfirhas/currencylib/blob/master/LICENSE)

A library for currencies.

It contains a trait defining a currency and types for ISO 4217 currencies.

No std and no unsafe code.

## Usage
The crate contains 2 main component the trait(`Currency`) and the types implementing it from ISO 4217 standard.

```rust
use currencylib::{Currency, USD /* ...and other iso 4217 currencies you want use, e.g. EUR, CAD, etc*/};

assert_eq!(USD::CODE, "USD");
assert_eq!(USD::SYMBOL, "$");
assert_eq!(USD::NAME, "United States dollar");
assert_eq!(USD::MINOR_UNIT, 2);
assert_eq!(USD::MINOR_UNIT_SYMBOL, "¢");
assert_eq!(USD::NUMERIC, 840_u16);
assert_eq!(USD::THOUSAND_SEPARATOR, ",");
assert_eq!(USD::DECIMAL_SEPARATOR, ".");
```
