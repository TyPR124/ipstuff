use crate::IpBitwiseExt;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use std::ops::Not;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// A 4-byte type representing a subnet mask in big-endian byte-order. This type can only be a valid subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv4Mask {
    mask: [u8; 4],
}

#[test]
fn test_mask_layout() {
    assert_eq!(Ipv4Mask::new(9), Ipv4Mask::from_bytes([255, 128, 0, 0]).unwrap());
}

#[allow(clippy::len_without_is_empty)]
impl Ipv4Mask {
    /// Returns a mask with the specified length.
    ///
    /// # Panics
    ///
    /// Will panic if provided length is > 32
    pub const fn new(len: u8) -> Self {
        #[rustfmt::skip]
        const MASKS: [[u8; 4]; 33] = [
            // /0
            [0, 0, 0, 0],
            // /1 - /8
            [128, 0, 0, 0], [192, 0, 0, 0], [224, 0, 0, 0], [240, 0, 0, 0],
            [248, 0, 0, 0], [252, 0, 0, 0], [254, 0, 0, 0], [255, 0, 0, 0],
            // /9 - /16
            [255, 128, 0, 0], [255, 192, 0, 0], [255, 224, 0, 0], [255, 240, 0, 0],
            [255, 248, 0, 0], [255, 252, 0, 0], [255, 254, 0, 0], [255, 255, 0, 0],
            // /17 - /24
            [255, 255, 128, 0], [255, 255, 192, 0], [255, 255, 224, 0], [255, 255, 240, 0],
            [255, 255, 248, 0], [255, 255, 252, 0], [255, 255, 254, 0], [255, 255, 255, 0],
            // /25 - /32
            [255, 255, 255, 128], [255, 255, 255, 192], [255, 255, 255, 224], [255, 255, 255, 240],
            [255, 255, 255, 248], [255, 255, 255, 252], [255, 255, 255, 254], [255, 255, 255, 255],
        ];
        let mask = MASKS[len as usize];
        Self { mask }
    }
    /// Constructs a subnet mask from the provided bytes, if they represent a valid mask.
    pub fn from_bytes(bytes: [u8; 4]) -> Option<Self> {
        Self::from_u32(u32::from_be_bytes(bytes))
    }
    /// Constructs a subnet mask from the provided u32, if it represents a valid mask.
    pub fn from_u32(x: u32) -> Option<Self> {
        let ones = if cfg!(target_feature = "popcnt") {
            x.count_ones() as u8 // popcnt;
        } else {
            (!x).leading_zeros() as u8 // not; bsr;
        };
        let zeros = x.trailing_zeros() as u8; // tzcnt / bsf;
        // add; test; sete;
        if ones + zeros == 32 {
            let mask = x.to_be_bytes();
            Some(Self { mask })
        } else {
            None
        }
    }
    /// Returns the subnet mask as an array of bytes.
    pub const fn octets(self) -> [u8; 4] {
        self.mask
    }
    /// Returns this mask in u32 representation
    pub const fn as_u32(self) -> u32 {
        let bytes = self.octets();
        (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
    }
    /// Returns the length of the mask. That is, the number of 1 bits in this mask.
    pub const fn len(self) -> u8 {
        let x = self.as_u32();
        #[cfg(target_feature = "popcnt")]
        let len = x.count_ones() as u8;
        #[cfg(not(target_feature = "popcnt"))]
        let len = (!x).leading_zeros() as u8;
        len
    }
}

impl Display for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            write!(f, "/{}", self.len())
        } else {
            let bytes = self.octets();
            write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
        }
    }
}

impl Debug for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Not for Ipv4Mask {
    type Output = [u8; 4];
    fn not(self) -> [u8; 4] {
        let x = u32::from_ne_bytes(self.octets());
        (!x).to_ne_bytes()
    }
}

impl FromStr for Ipv4Mask {
    type Err = InvalidIpv4Mask;
    fn from_str(s: &str) -> Result<Self, InvalidIpv4Mask> {
        let bytes = s.parse::<Ipv4Addr>().map_err(|_| InvalidIpv4Mask)?.octets();
        Self::from_bytes(bytes).ok_or(InvalidIpv4Mask)
    }
}
/// Error when failing to parse an Ipv4Mask.
#[derive(Debug)]
pub struct InvalidIpv4Mask;
/// An 8-byte type representing an IPv4 address and subnet mask pair. The IP may be any ip
/// within the represented network, and the mask may be any valid subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MaskedIpv4 {
    /// The IP address
    pub ip: Ipv4Addr,
    /// The subnet mask
    pub mask: Ipv4Mask,
}

impl MaskedIpv4 {
    /// Constructs a MaskedIpv4 from the provided ip and mask.
    pub const fn new(ip: Ipv4Addr, mask: Ipv4Mask) -> Self {
        Self { ip, mask }
    }
    /// Constructs a MaskedIpv4 from the provided ip and mask length.
    ///
    /// # Panics
    ///
    /// Will panic if provided length > 32
    pub const fn cidr(ip: Ipv4Addr, mask_len: u8) -> Self {
        let mask = Ipv4Mask::new(mask_len);
        Self::new(ip, mask)
    }
    /// Constructs a new MaskedIpv4 from the provided CIDR string.
    pub fn from_cidr_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let ip = parts.next()?.parse::<Ipv4Addr>().ok()?;
        let mask_len = parts.next()?.parse::<u8>().ok()?;
        if mask_len > 32 {
            None
        } else {
            Some(Self::cidr(ip, mask_len))
        }
    }
    /// Constructs a new MaskedIpv4 from the provided IP and subnet mask. There should be exactly one space between the IP and mask.
    pub fn from_network_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, ' ');
        let ip = parts.next()?.parse().ok()?;
        let mask = parts.next()?.parse().ok()?;
        Some(Self::new(ip, mask))
    }
    /// Returns a String with the IP and mask in CIDR format. Shortcut for `format!("{:#}", self)`
    pub fn to_cidr_string(&self) -> String {
        format!("{:#}", self)
    }
    /// Returns a String with the IP and mask in dotted decimal format. Shortcut for `format!("{}", self)`
    pub fn to_network_string(&self) -> String {
        format!("{}", self)
    }
    /// Returns the network adderss by setting all host bits to 0.
    pub fn network_address(&self) -> Ipv4Addr {
        self.ip.bitand(self.mask)
    }
    /// Constructs a new MaskedIpv4 using the network address and mask of this MaskedIpv4.
    pub fn network(&self) -> MaskedIpv4 {
        Self::new(self.network_address(), self.mask)
    }
    /// Returns true if all host bits in the IP are 0. Always returns false if the mask length is 31 or 32.
    pub fn is_network_address(&self) -> bool {
        self.mask.len() <= 30 && self.ip == self.network_address()
    }
    /// Returns the broadcast address by setting all host bits to 1.
    pub fn broadcast_address(&self) -> Ipv4Addr {
        self.ip.bitor(!self.mask)
    }
    /// Returns true if all host bits in the IP are 1. Always returns false if the mask length is 31 or 32.
    pub fn is_broadcast_address(&self) -> bool {
        self.mask.len() <= 30 && self.ip == self.broadcast_address()
    }
    /// Returns the number of network bits. That is, the length of the mask.
    pub fn network_bits(&self) -> u8 {
        self.mask.len()
    }
    /// Returns the number of host bits. That is, the number of 0 bits in the mask.
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
    /// Returns the number of host addresses in the network as u64. Unlike host_count, this will never panic.
    pub fn host_count_u64(&self) -> u64 {
        let host_bits = self.host_bits();
        match host_bits {
            0 => 1,
            1 => 2,
            _ => (2 << host_bits) - 2,
        }
    }
    /// Returns the number of networks of the provided mask length will fit in this network.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 32, or if the number of networks does not fit in usize
    pub fn network_count(&self, len: u8) -> usize {
        if len < self.mask.len() {
            0
        } else if len > 32 {
            panic!("Invalid mask length > 32")
        } else {
            let borrowed_bits = len - self.mask.len();
            2usize.checked_shl(borrowed_bits as u32).unwrap()
        }
    }
    /// Returns the number of networks of the provided mask length will fit in this network as u64. Unlike network_count, this will not panic
    /// due to overflow. May still panic if the provided length is too long.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 32
    pub fn network_count_u64(&self, len: u8) -> u64 {
        if len < self.mask.len() {
            0
        } else if len > 32 {
            panic!("Invalid mask length > 32")
        } else {
            let borrowed_bits = len - self.mask.len();
            2 << borrowed_bits
        }
    }
    /// Returns true if this network contains the provided IP address, even if the provided IP is the network or broadcast address.
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        self.ip.bitand(self.mask) == ip.bitand(self.mask)
    }
}

impl Display for MaskedIpv4 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            write!(f, "{}/{}", self.ip, self.mask.len())
        } else {
            write!(f, "{} {}", self.ip, self.mask)
        }
    }
}

impl Debug for MaskedIpv4 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl FromStr for MaskedIpv4 {
    type Err = InvalidMaskedIpv4;
    fn from_str(s: &str) -> Result<Self, InvalidMaskedIpv4> {
        Self::from_cidr_str(s)
            .or_else(|| Self::from_network_str(s))
            .ok_or(InvalidMaskedIpv4)
    }
}
/// Error when failing to parse a MaskedIpv4.
#[derive(Debug)]
pub struct InvalidMaskedIpv4;
