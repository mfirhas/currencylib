//! Contains currencies data type and accessors.
//!
//!

use crate::iso_currencies_data::ISO_CURRENCY_DATA;

/// Currency data.
///
/// `code` is the identity of a currency and used for equality.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Data {
    pub code: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    pub numeric: u16,
    pub minor_unit: u16,
    pub minor_unit_symbol: &'static str,
    pub minor_unit_name: &'static str,
    pub thousand_separator: &'static str,
    pub decimal_separator: &'static str,
    pub origin: &'static str,
    pub locale: &'static str,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

/// Get a currency's data by code.
pub fn get(code: &str) -> Option<Data> {
    ISO_CURRENCY_DATA.get(code).copied()
}

/// Get iterator to all currencies.
pub fn entries() -> impl Iterator<Item = (&'static str, Data)> {
    ISO_CURRENCY_DATA.entries().map(|(&k, &v)| (k, v))
}
