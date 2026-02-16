#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

mod currencies;
pub use currencies::*;

#[cfg(test)]
mod currencies_test;
