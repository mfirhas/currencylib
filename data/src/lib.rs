#![no_std]
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]

pub mod currencies;
mod iso_currencies_data;

#[cfg(test)]
mod currencies_test;
