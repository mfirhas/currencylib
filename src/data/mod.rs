#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]

mod currencies;
#[allow(unused)]
pub use currencies::{Data, entries, get};

mod iso_currencies_data;

#[cfg(test)]
mod currencies_test;
