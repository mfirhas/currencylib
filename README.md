# currencylib

[![MSRV](https://img.shields.io/crates/msrv/currencylib)](https://crates.io/crates/currencylib)
[![Crates.io](https://img.shields.io/crates/v/currencylib.svg)](https://crates.io/crates/currencylib)
[![ci](https://github.com/mfirhas/currencylib/actions/workflows/ci.yml/badge.svg)](https://github.com/mfirhas/currencylib/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/currencylib/badge.svg)](https://docs.rs/currencylib)
[![codecov](https://codecov.io/gh/mfirhas/currencylib/branch/master/graph/badge.svg)](https://codecov.io/gh/mfirhas/currencylib)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/mfirhas/currencylib/blob/master/LICENSE)

A library for currencies.

It contains a trait defining a currency and types for ISO 4217 currencies.

No std and no unsafe code.

## Usage
The crate contains 2 main component the trait(`Currency`) and the types implementing it from ISO 4217 standard.

Currencies are compared lexicographically by their code.

```rust
use currencylib::{Currency, USD /* ...and other iso 4217 currencies you want use, e.g. EUR, CAD, etc*/};
use std::str::FromStr;

assert_eq!(USD::CODE, "USD");
assert_eq!(USD::SYMBOL, "$");
assert_eq!(USD::NAME, "United States dollar");
assert_eq!(USD::NUMERIC, 840_u16);
assert_eq!(USD::MINOR_UNIT, 2);
assert_eq!(USD::MINOR_UNIT_SYMBOL, "¢");
assert_eq!(USD::MINOR_UNIT_NAME, "cent");
assert_eq!(USD::THOUSAND_SEPARATOR, ",");
assert_eq!(USD::DECIMAL_SEPARATOR, ".");
assert_eq!(USD::ORIGIN, "United States");
assert_eq!(USD::LOCALE, "en-US");

let usd = USD::from_str("USD").unwrap();
assert_eq!(usd, USD);
```
