//! # ipstuff
//!
//! Various utilities for working with IP addresses and subnet masks.
//!
#![no_std]
#![forbid(unsafe_code)]

mod addrs;
pub use addrs::*;

mod masked;
pub use masked::*;

#[cfg(test)]
mod tests;
