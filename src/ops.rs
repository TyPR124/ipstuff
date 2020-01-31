use std::net::Ipv4Addr;

use crate::Ipv4Mask;

/// An extension trait providing bitwise binary operations
/// for Ipv4Addr and Ipv6Addr types.
pub trait IpBitwiseExt<Rhs = Self> {
    type Output;
    fn bitand(self, rhs: Rhs) -> Self::Output;
    fn bitor(self, rhs: Rhs) -> Self::Output;
    fn bitxor(self, rhs: Rhs) -> Self::Output;
}

pub trait IpBitwiseNotExt<Output = Self> {
    fn bitnot(self) -> Output;
}

impl IpBitwiseNotExt<Ipv4Addr> for Ipv4Addr {
    fn bitnot(self) -> Self {
        let bytes = self.octets();
        Self::new(!bytes[0], !bytes[1], !bytes[2], !bytes[3])
    }
}

impl IpBitwiseNotExt<[u8; 4]> for Ipv4Addr {
    fn bitnot(self) -> [u8; 4] {
        IpBitwiseNotExt::<Ipv4Addr>::bitnot(self).octets()
    }
}

impl IpBitwiseNotExt<u32> for Ipv4Addr {
    fn bitnot(self) -> u32 {
        !u32::from(self)
    }
}

impl IpBitwiseExt<[u8; 4]> for Ipv4Addr {
    type Output = Self;
    fn bitand(self, rhs: [u8; 4]) -> Self {
        let lhs = self.octets();
        Ipv4Addr::new(
            lhs[0] & rhs[0],
            lhs[1] & rhs[1],
            lhs[2] & rhs[2],
            lhs[3] & rhs[3],
        )
    }
    fn bitor(self, rhs: [u8; 4]) -> Self {
        let lhs = self.octets();
        Ipv4Addr::new(
            lhs[0] | rhs[0],
            lhs[1] | rhs[1],
            lhs[2] | rhs[2],
            lhs[3] | rhs[3],
        )
    }
    fn bitxor(self, rhs: [u8; 4]) -> Self {
        let lhs = self.octets();
        Ipv4Addr::new(
            lhs[0] ^ rhs[0],
            lhs[1] ^ rhs[1],
            lhs[2] ^ rhs[2],
            lhs[3] ^ rhs[3],
        )
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
