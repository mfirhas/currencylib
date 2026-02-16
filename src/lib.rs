#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

/// The Currency properties
pub trait Currency {
    /// Currency alphabethical code (e.g., "USD", "EUR")
    const CODE: &'static str;

    /// Currency symbol (e.g., "$", "€")
    const SYMBOL: &'static str;

    /// Full currency name (e.g., "US Dollar", "Euro")
    const NAME: &'static str;

    /// numerical code
    const NUMERIC: u16;

    /// Number of decimal places (minor units)
    const MINOR_UNIT: u16;

    /// Symbol of currency's minor unit
    const MINOR_UNIT_SYMBOL: &'static str;

    /// thousand separator
    const THOUSAND_SEPARATOR: &'static str;

    /// decimal separator
    const DECIMAL_SEPARATOR: &'static str;
}

mod iso_currencies;
pub use iso_currencies::*;

#[cfg(test)]
mod iso_currencies_test;
