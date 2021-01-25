use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Ipv4Mask, Ipv6Mask};

/// An extension trait providing bitwise binary operations
/// for Ipv4Addr and Ipv6Addr types.
pub trait IpBitwiseExt<Rhs = Self> {
    /// The output type of this operation.
    type Output;
    /// Bitwise And operation.
    fn bitand(self, rhs: Rhs) -> Self::Output;
    /// Bitwise Or operation.
    fn bitor(self, rhs: Rhs) -> Self::Output;
    /// Bitwise Xor operation.
    fn bitxor(self, rhs: Rhs) -> Self::Output;
}
/// An extention trait providing bitwise not operation
/// for Ipv4Addr and Ipv6Addr types.
pub trait IpBitwiseNotExt {
    /// Bitwise Not operation.
    fn bitnot(self) -> Self;
}

impl IpBitwiseNotExt for Ipv4Addr {
    fn bitnot(self) -> Self {
        let bytes = self.octets();
        let x = !u32::from_ne_bytes(bytes);
        Self::from(x.to_ne_bytes())
    }
}

impl IpBitwiseExt<[u8; 4]> for Ipv4Addr {
    type Output = Self;
    fn bitand(self, rhs: [u8; 4]) -> Self {
        let lhs = u32::from_ne_bytes(self.octets());
        let rhs = u32::from_ne_bytes(rhs);
        Self::from((lhs & rhs).to_ne_bytes())
    }
    fn bitor(self, rhs: [u8; 4]) -> Self {
        let lhs = u32::from_ne_bytes(self.octets());
        let rhs = u32::from_ne_bytes(rhs);
        Self::from((lhs | rhs).to_ne_bytes())
    }
    fn bitxor(self, rhs: [u8; 4]) -> Self {
        let lhs = u32::from_ne_bytes(self.octets());
        let rhs = u32::from_ne_bytes(rhs);
        Self::from((lhs ^ rhs).to_ne_bytes())
    }
}

impl IpBitwiseExt<Self> for Ipv4Addr {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        self.bitand(rhs.octets())
    }
    fn bitor(self, rhs: Self) -> Self {
        self.bitor(rhs.octets())
    }
    fn bitxor(self, rhs: Self) -> Self {
        self.bitxor(rhs.octets())
    }
}

impl IpBitwiseExt<Ipv4Mask> for Ipv4Addr {
    type Output = Self;
    fn bitand(self, rhs: Ipv4Mask) -> Self {
        self.bitand(rhs.octets())
    }
    fn bitor(self, rhs: Ipv4Mask) -> Self {
        self.bitor(rhs.octets())
    }
    fn bitxor(self, rhs: Ipv4Mask) -> Self {
        self.bitxor(rhs.octets())
    }
}

impl IpBitwiseExt<u32> for Ipv4Addr {
    type Output = Self;
    fn bitand(self, rhs: u32) -> Self {
        self.bitand(rhs.to_be_bytes())
    }
    fn bitor(self, rhs: u32) -> Self {
        self.bitor(rhs.to_be_bytes())
    }
    fn bitxor(self, rhs: u32) -> Self {
        self.bitxor(rhs.to_be_bytes())
    }
}

impl IpBitwiseNotExt for Ipv6Addr {
    fn bitnot(self) -> Self {
        let bytes = self.octets();
        let x = !u128::from_ne_bytes(bytes);
        Self::from(x.to_ne_bytes())
    }
}

impl IpBitwiseExt<[u8; 16]> for Ipv6Addr {
    type Output = Self;
    fn bitand(self, rhs: [u8; 16]) -> Self::Output {
        let lhs = u128::from_ne_bytes(self.octets());
        let rhs = u128::from_ne_bytes(rhs);
        Self::from((lhs & rhs).to_ne_bytes())
    }
    fn bitor(self, rhs: [u8; 16]) -> Self::Output {
        let lhs = u128::from_ne_bytes(self.octets());
        let rhs = u128::from_ne_bytes(rhs);
        Self::from((lhs | rhs).to_ne_bytes())
    }
    fn bitxor(self, rhs: [u8; 16]) -> Self::Output {
        let lhs = u128::from_ne_bytes(self.octets());
        let rhs = u128::from_ne_bytes(rhs);
        Self::from((lhs ^ rhs).to_ne_bytes())
    }
}

impl IpBitwiseExt<Self> for Ipv6Addr {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        self.bitand(rhs.octets())
    }
    fn bitor(self, rhs: Self) -> Self {
        self.bitor(rhs.octets())
    }
    fn bitxor(self, rhs: Self) -> Self {
        self.bitxor(rhs.octets())
    }
}

impl IpBitwiseExt<Ipv6Mask> for Ipv6Addr {
    type Output = Self;
    fn bitand(self, rhs: Ipv6Mask) -> Self {
        self.bitand(rhs.octets())
    }
    fn bitor(self, rhs: Ipv6Mask) -> Self {
        self.bitor(rhs.octets())
    }
    fn bitxor(self, rhs: Ipv6Mask) -> Self {
        self.bitxor(rhs.octets())
    }
}

impl IpBitwiseExt<[u16; 8]> for Ipv6Addr {
    type Output = Self;
    fn bitand(self, rhs: [u16; 8]) -> Self::Output {
        self.bitand(Ipv6Addr::from(rhs))
    }
    fn bitor(self, rhs: [u16; 8]) -> Self::Output {
        self.bitor(Ipv6Addr::from(rhs))
    }
    fn bitxor(self, rhs: [u16; 8]) -> Self::Output {
        self.bitxor(Ipv6Addr::from(rhs))
    }
}

impl IpBitwiseExt<u128> for Ipv6Addr {
    type Output = Self;
    fn bitand(self, rhs: u128) -> Self {
        self.bitand(rhs.to_be_bytes())
    }
    fn bitor(self, rhs: u128) -> Self {
        self.bitor(rhs.to_be_bytes())
    }
    fn bitxor(self, rhs: u128) -> Self {
        self.bitxor(rhs.to_be_bytes())
    }
}
