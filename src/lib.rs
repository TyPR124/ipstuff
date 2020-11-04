//! # ipstuff
//!
//! Various utilities for working with IP addresses and subnet masks.
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod masked;
pub use masked::*;
mod ops;
pub use ops::*;

#[cfg(test)]
mod tests;
