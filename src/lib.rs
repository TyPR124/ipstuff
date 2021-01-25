//! # ipstuff
//!
//! Various utilities for working with IP addresses and subnet masks.
#![forbid(unsafe_code)]
// #![warn(missing_docs)] FIXME
// #![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
#[macro_export]
macro_rules! mk_zst_error_type {
    ($Error:ident = $msg:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $Error;
        impl Display for $Error {
            fn fmt(&self, out: &mut Formatter) -> fmt::Result {
                out.write_str($msg)
            }
        }
        impl Debug for $Error {
            fn fmt(&self, out: &mut Formatter) -> fmt::Result {
                Display::fmt(self, out)
            }
        }
        impl std::error::Error for $Error {}
    };
}

mod bitwise;
mod mask;
mod masked;
mod network;

pub use bitwise::{IpBitwiseExt, IpBitwiseNotExt};
pub use mask::{InvalidIpv4Mask, InvalidIpv6Mask, Ipv4Mask, Ipv6Mask};
pub use masked::*;
pub use network::*;

#[cfg(feature = "serde")]
mod impl_serde;
#[cfg(feature = "serde")]
pub mod se;
