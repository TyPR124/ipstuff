use crate::{IpBitwiseExt, Ipv4Mask, Ipv6Mask, NetworkV4, NetworkV6};

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
// use std::ops::Not;
use std::str::FromStr;

/// An 8-byte type representing an IPv4 address and subnet mask pair. The IP may
/// be any IP within the represented network, and the mask may be any valid
/// subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MaskedIpv4 {
    /// The IP address
    pub ip: Ipv4Addr,
    /// The subnet mask
    pub mask: Ipv4Mask,
}
/// A 32-byte type representing an IPv6 address and subnet mask pair. The IP may
/// be any IP within the represented network, and the mask may be any valid
/// subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MaskedIpv6 {
    /// The IP address
    pub ip: Ipv6Addr,
    /// The subnet mask
    pub mask: Ipv6Mask,
}
/// An enum which may represent either a V4 or V6 Masked IP Address.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MaskedIp {
    /// A [`MaskedIpv4`]
    V4(MaskedIpv4),
    /// A [`MaskedIpv6`]
    V6(MaskedIpv6),
}

impl MaskedIpv4 {
    /// Constructs a MaskedIpv4 from the provided IP and mask.
    pub const fn new(ip: Ipv4Addr, mask: Ipv4Mask) -> Self {
        Self { ip, mask }
    }
    /// Constructs a MaskedIpv4 from the provided IP and mask length.
    pub const fn cidr(ip: Ipv4Addr, mask_len: u8) -> Option<Self> {
        match Ipv4Mask::new(mask_len) {
            Some(mask) => Some(Self::new(ip, mask)),
            None => None,
        }
    }
    /// Constructs a new MaskedIpv4 from the provided CIDR string.
    pub fn from_cidr_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let ip = parts.next()?.parse::<Ipv4Addr>().ok()?;
        let mask_len = parts.next()?.parse::<u8>().ok()?;
        Self::cidr(ip, mask_len)
    }
    /// Constructs a new MaskedIpv4 from the provided IP and subnet mask. There
    /// must be exactly one space between the IP and mask.
    pub fn from_network_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, ' ');
        let ip = parts.next()?.parse().ok()?;
        let mask = parts.next()?.parse().ok()?;
        Some(Self::new(ip, mask))
    }
    /// Returns a String with the IP and mask in CIDR format. Shortcut for
    /// `format!("{:#}", self)`
    pub fn to_cidr_string(&self) -> String {
        format!("{:#}", self)
    }
    /// Returns a String with the IP and mask in dotted decimal format. Shortcut
    /// for `format!("{}", self)`
    pub fn to_network_string(&self) -> String {
        format!("{}", self)
    }
    /// Returns the network adderss by setting all host bits to 0.
    pub fn network_address(&self) -> Ipv4Addr {
        self.ip.bitand(self.mask)
    }
    /// Constructs a new MaskedIpv4 using the network address and mask of this
    /// MaskedIpv4.
    pub fn network(&self) -> MaskedIpv4 {
        Self::new(self.network_address(), self.mask)
    }
    /// Returns true if all host bits in the IP are 0. Always returns false if
    /// the mask length is 31 or 32.
    pub fn is_network_address(&self) -> bool {
        self.mask.len() <= 30 && self.ip == self.network_address()
    }
    /// Returns the broadcast address by setting all host bits to 1.
    pub fn broadcast_address(&self) -> Ipv4Addr {
        self.ip.bitor(!self.mask)
    }
    /// Returns true if all host bits in the IP are 1. Always returns false if
    /// the mask length is 31 or 32.
    pub fn is_broadcast_address(&self) -> bool {
        self.mask.len() <= 30 && self.ip == self.broadcast_address()
    }
    /// Returns the number of network bits. That is, the length of the mask.
    pub fn network_bits(&self) -> u8 {
        self.mask.len()
    }
    /// Returns the number of host bits. That is, the number of 0 bits in the
    /// mask.
    pub fn host_bits(&self) -> u8 {
        32 - self.network_bits()
    }
    /// Returns the number of host addresses in the network.
    ///
    /// # Panics
    ///
    /// Will panic if usize is not large enough to hold the host count.
    pub fn host_count(&self) -> usize {
        let host_bits = self.host_bits();
        match host_bits {
            0 => 1,
            1 => 2,
            _ => 2usize.checked_shl(host_bits as u32).unwrap() - 2,
        }
    }
    /// Returns the number of host addresses in the network as u64. Unlike
    /// host_count, this will never panic.
    pub fn host_count_u64(&self) -> u64 {
        let host_bits = self.host_bits();
        match host_bits {
            0 => 1,
            1 => 2,
            _ => (2 << host_bits) - 2,
        }
    }
    /// Returns the number of networks of the provided mask length will fit in
    /// this network.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 32, or if the number of networks
    /// does not fit in usize
    pub fn network_count(&self, new_len: u8) -> usize {
        let curr_len = self.mask.len();
        if new_len < curr_len {
            0
        } else if new_len > 32 {
            panic!("Invalid mask length > 32")
        } else {
            let borrowed_bits = new_len - curr_len;
            2usize.checked_shl(borrowed_bits as u32).unwrap()
        }
    }
    /// Returns the number of networks of the provided mask length will fit in
    /// this network as u64. Unlike network_count, this will not panic
    /// due to overflow. May still panic if the provided length is too long.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 32
    pub fn network_count_u64(&self, new_len: u8) -> u64 {
        let curr_len = self.mask.len();
        if new_len < curr_len {
            0
        } else if new_len > 32 {
            panic!("Invalid mask length > 32")
        } else {
            let borrowed_bits = new_len - curr_len;
            2 << borrowed_bits
        }
    }
    /// Returns true if this network contains the provided IP address, even if
    /// the provided IP is the network or broadcast address.
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        self.ip.bitand(self.mask) == ip.bitand(self.mask)
    }
    pub fn to_network(&self) -> NetworkV4 {
        NetworkV4::new(self.ip, self.mask)
    }
}

impl MaskedIpv6 {
    /// Constructs a MaskedIpv6 from the provided IP and mask.
    pub const fn new(ip: Ipv6Addr, mask: Ipv6Mask) -> Self {
        Self { ip, mask }
    }
    /// Constructs a MaskedIpv6 from the provided IP and mask length.
    pub const fn cidr(ip: Ipv6Addr, mask_len: u8) -> Option<Self> {
        match Ipv6Mask::new(mask_len) {
            Some(mask) => Some(Self::new(ip, mask)),
            None => None,
        }
    }
    /// Constructs a MaskedIpv6 from the provided CIDR string.
    pub fn from_cidr_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let ip = parts.next()?.parse::<Ipv6Addr>().ok()?;
        let mask_len = parts.next()?.parse::<u8>().ok()?;
        Self::cidr(ip, mask_len)
    }
    /// Returns a String with the IP and mask in CIDR format. Shortcut for
    /// `format!("{}", self)`
    pub fn to_cidr_str(&self) -> String {
        format!("{}", self)
    }
    /// Returns the "network" address by setting all host bits to 0.
    ///
    /// Note: Ipv6 does not technically have a "network" address, however this
    /// method can still be useful to determine the base address of a
    /// network.
    pub fn network_address(&self) -> Ipv6Addr {
        self.ip.bitand(self.mask)
    }
    /// Constructs a new MaskedIpv6 using the network address and mask of this
    /// MaskedIpv6.
    pub fn network(&self) -> MaskedIpv6 {
        Self::new(self.network_address(), self.mask)
    }
    /// Returns true if all host bits in the IP are 0. Always returns false if
    /// the mask len is 127 or 128.
    pub fn is_network_address(&self) -> bool {
        self.mask.len() <= 126 && self.ip == self.network_address()
    }
    /// Returns the number of network bits. That is, the length of the mask.
    pub fn network_bits(&self) -> u8 {
        self.mask.len()
    }
    /// Returns the number of host bits. That is, the number of 0 bits in the
    /// mask.
    pub fn host_bits(&self) -> u8 {
        128 - self.network_bits()
    }
    /// Returns the number of hosts in the network.
    ///
    /// # Panics
    ///
    /// Will panic if u128 is not large enough to hold the host count.
    pub fn host_count(&self) -> u128 {
        2u128.checked_shl(self.host_bits() as u32).unwrap()
    }
    /// Returns the number of networks of the provided mask length will fit in
    /// this network.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 128, or if the number of networks
    /// does not fit in usize
    pub fn network_count(&self, new_len: u8) -> u128 {
        let curr_len = self.mask.len();
        if new_len < curr_len {
            0
        } else if new_len > 128 {
            panic!("Invalid mask length > 128")
        } else {
            let borrowed_bits = new_len - curr_len;
            2u128.checked_pow(borrowed_bits as u32).unwrap()
        }
    }
    /// Returns true if this network contains the provided IP address.
    pub fn contains(&self, ip: Ipv6Addr) -> bool {
        self.ip.bitand(self.mask) == ip.bitand(self.mask)
    }
    pub fn to_network(&self) -> NetworkV6 {
        NetworkV6::new(self.ip, self.mask)
    }
}

impl MaskedIp {
    pub fn from_cidr_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let ip = parts.next()?.parse().ok()?;
        let mask_len = parts.next()?.parse().ok()?;
        match ip {
            IpAddr::V4(ip) => MaskedIpv4::cidr(ip, mask_len).map(Self::V4),
            IpAddr::V6(ip) => MaskedIpv6::cidr(ip, mask_len).map(Self::V6),
        }
    }
    pub fn to_cidr_string(&self) -> String {
        format!("{:#}", self)
    }
    pub fn network_address(&self) -> IpAddr {
        match self {
            Self::V4(m) => IpAddr::V4(m.network_address()),
            Self::V6(m) => IpAddr::V6(m.network_address()),
        }
    }
    pub fn network(&self) -> Self {
        match self {
            Self::V4(m) => Self::V4(m.network()),
            Self::V6(m) => Self::V6(m.network()),
        }
    }
    pub fn is_network_address(&self) -> bool {
        match self {
            Self::V4(m) => m.is_network_address(),
            Self::V6(m) => m.is_network_address(),
        }
    }
    pub fn is_broadcast_address(&self) -> bool {
        match self {
            Self::V4(m) => m.is_broadcast_address(),
            Self::V6(_) => false,
        }
    }
    pub fn network_bits(&self) -> u8 {
        match self {
            Self::V4(m) => m.network_bits(),
            Self::V6(m) => m.network_bits(),
        }
    }
    pub fn host_bits(&self) -> u8 {
        match self {
            Self::V4(m) => m.host_bits(),
            Self::V6(m) => m.host_bits(),
        }
    }
    pub fn host_count(&self) -> u128 {
        match self {
            Self::V4(m) => m.host_count_u64() as u128,
            Self::V6(m) => m.host_count(),
        }
    }
    pub fn network_count(&self, len: u8) -> u128 {
        match self {
            Self::V4(m) => m.network_count_u64(len) as u128,
            Self::V6(m) => m.network_count(len),
        }
    }
    pub fn contains(&self, ip: IpAddr) -> bool {
        match (self, ip) {
            (Self::V4(m), IpAddr::V4(ip)) => m.contains(ip),
            (Self::V6(m), IpAddr::V6(ip)) => m.contains(ip),
            _ => false,
        }
    }
}

impl Display for MaskedIpv4 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            write!(f, "{} {:#}", self.ip, self.mask)
        } else {
            write!(f, "{}/{}", self.ip, self.mask.len())
        }
    }
}

impl Display for MaskedIpv6 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            write!(f, "{} {:#}", self.ip, self.mask)
        } else {
            write!(f, "{}/{}", self.ip, self.mask.len())
        }
    }
}

impl Display for MaskedIp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MaskedIp::V4(masked) => Display::fmt(masked, f),
            MaskedIp::V6(masked) => Display::fmt(masked, f),
        }
    }
}

impl Debug for MaskedIpv4 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Debug for MaskedIpv6 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Debug for MaskedIp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl FromStr for MaskedIpv4 {
    type Err = InvalidMaskedIpv4;
    fn from_str(s: &str) -> Result<Self, InvalidMaskedIpv4> {
        let mut cidr = true;
        let mut parts = s.splitn(2, |ch| match ch {
            '/' => true,
            ' ' => {
                cidr = false;
                true
            }
            _ => false,
        });
        let ip = parts
            .next()
            .and_then(|ip| ip.parse().ok())
            .ok_or(InvalidMaskedIpv4)?;
        let mask = parts
            .next()
            .and_then(|mask| {
                if cidr {
                    mask.parse().ok().and_then(Ipv4Mask::new)
                } else {
                    mask.parse().ok().and_then(Ipv4Mask::from_ip)
                }
            })
            .ok_or(InvalidMaskedIpv4)?;
        Ok(Self::new(ip, mask))
    }
}

impl FromStr for MaskedIpv6 {
    type Err = InvalidMaskedIpv6;
    fn from_str(s: &str) -> Result<Self, InvalidMaskedIpv6> {
        let mut cidr = true;
        let mut parts = s.splitn(2, |ch| match ch {
            '/' => true,
            ' ' => {
                cidr = false;
                true
            }
            _ => false,
        });
        let ip = parts
            .next()
            .and_then(|ip| ip.parse().ok())
            .ok_or(InvalidMaskedIpv6)?;
        let mask = parts
            .next()
            .and_then(|mask| {
                if cidr {
                    mask.parse().ok().and_then(Ipv6Mask::new)
                } else {
                    mask.parse().ok().and_then(Ipv6Mask::from_ip)
                }
            })
            .ok_or(InvalidMaskedIpv6)?;
        Ok(Self::new(ip, mask))
    }
}

// impl FromStr for MaskedIp {
//     type Err = InvalidMaskedIp;
//     fn from_str(s: &str) -> Result<Self, InvalidMaskedIp> {
//         let mut is_v4 = false;
//         let mut is_cidr = false;
//         let mut first_index = 0;
//         let split_index = s
//             .find(|ch| match ch {
//                 ':' => {
//                     is_v4 = false;
//                     true
//                 }
//                 '.' => {
//                     is_v4 = true;
//                     true
//                 }
//                 _ => false,
//             })
//             .and_then(|ind| {
//                 first_index = ind;
//                 s[ind + 1..].find(|ch| match ch {
//                     '/' => {
//                         is_cidr = true;
//                         true
//                     }
//                     ' ' => {
//                         is_cidr = false;
//                         true
//                     }
//                     _ => false,
//                 })
//             })
//             .ok_or(InvalidMaskedIp)?;
//         let (ip, mask) = s.split_at(first_index + split_index + 1);
//         let mask = &mask[1..];
//         if is_v4 {
//             let ip = ip.parse().map_err(|_| InvalidMaskedIp)?;
//             let mask = if is_cidr {
//                 let len = mask.parse::<u8>().map_err(|_| InvalidMaskedIp)?;
//                 if len > 32 {
//                     return Err(InvalidMaskedIp);
//                 }
//                 Ipv4Mask::new(len).ok_or(InvalidMaskedIpv4)?
//             } else {
//                 let mask_bytes = mask
//                     .parse::<Ipv4Addr>()
//                     .map_err(|_| InvalidMaskedIp)?
//                     .octets();
//                 Ipv4Mask::from_bytes(mask_bytes).ok_or(InvalidMaskedIp)?
//             };
//             Ok(Self::V4(MaskedIpv4 { ip, mask }))
//         } else {
//             // v6
//             if !is_cidr {
//                 return Err(InvalidMaskedIp);
//             }
//             let ip = ip.parse().map_err(|_| InvalidMaskedIp)?;
//             let len = mask.parse::<u8>().map_err(|_| InvalidMaskedIp)?;
//             if len > 128 {
//                 return Err(InvalidMaskedIp);
//             }
//             let mask = Ipv6Mask::new(len);
//             Ok(Self::V6(MaskedIpv6 { ip, mask }))
//         }
//     }
// }
/// Error when failing to parse a [`MaskedIpv4`].
pub struct InvalidMaskedIpv4;
/// Error when failing to parse a [`MaskedIpv6`].
pub struct InvalidMaskedIpv6;
/// Error when failing to parse a [`MaskedIp`].
pub struct InvalidMaskedIp;

impl Display for InvalidMaskedIpv4 {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        out.write_str("invalid masked IPv4")
    }
}
impl Debug for InvalidMaskedIpv4 {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        Display::fmt(self, out)
    }
}
impl Error for InvalidMaskedIpv4 {}
impl Display for InvalidMaskedIpv6 {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        out.write_str("invalid masked IPv6")
    }
}
impl Debug for InvalidMaskedIpv6 {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        Display::fmt(self, out)
    }
}
impl Error for InvalidMaskedIpv6 {}
impl Display for InvalidMaskedIp {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        out.write_str("invalid masked IP")
    }
}
impl Debug for InvalidMaskedIp {
    fn fmt(&self, out: &mut Formatter) -> FmtResult {
        Display::fmt(self, out)
    }
}
impl Error for InvalidMaskedIp {}
