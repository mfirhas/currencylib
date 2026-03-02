#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]

/// The Currency properties
pub trait Currency {
    /// Currency alphabethical code (e.g., "USD", "EUR")
    const CODE: &'static str;

    /// Currency symbol (e.g., "$", "€")
    const SYMBOL: &'static str;

    /// Full currency name (e.g., "US Dollar", "Euro")
    const NAME: &'static str;

    /// Currency numerical identification
    const NUMERIC: u16;

    /// Number of decimal places/minor units (e.g., USD -> cents -> minor unit = 2)
    const MINOR_UNIT: u16;

    /// Currency minor symbol (e.g., USD -> "¢")
    const MINOR_UNIT_SYMBOL: &'static str;

    /// thousand separator (e.g., USD -> ",")
    const THOUSAND_SEPARATOR: &'static str;

    /// decimal separator (e.g., USD -> ".")
    const DECIMAL_SEPARATOR: &'static str;
}

mod iso_currencies;
pub use iso_currencies::*;

#[cfg(test)]
mod iso_currencies_test;
