use crate::IpBitwiseExt;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use std::ops::Not;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// A 4-byte type representing a subnet mask in big-endian byte-order. This type can only be a valid subnet mask.
#[repr(align(4))]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv4Mask {
    mask: [u8; 4],
}
/// A 16-byte type representing a subnet mask in big-endian byte-order. this type can only be a valid subnet mask.
#[repr(align(16))]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ipv6Mask {
    mask: [u8; 16],
}

#[test]
fn test_mask_layout() {
    assert_eq!(
        Ipv4Mask::new(9),
        Ipv4Mask::from_bytes([255, 128, 0, 0]).unwrap()
    );
    assert_eq!(
        Ipv6Mask::new(9),
        Ipv6Mask::from_bytes([255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap()
    );
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
    /// Returns the subnet mask as a native-endian u32.
    pub const fn as_u32(self) -> u32 {
        u32::from_be_bytes(self.octets())
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
#[allow(clippy::len_without_is_empty)]
impl Ipv6Mask {
    /// Returns a mask with the specified length.
    ///
    /// # Panics
    ///
    /// Will panic if provided length is > 128
    pub const fn new(len: u8) -> Self {
        const MASKS: [u128; 129] = [
            0x0,
            0x80000000000000000000000000000000,
            0xC0000000000000000000000000000000,
            0xE0000000000000000000000000000000,
            0xF0000000000000000000000000000000,
            0xF8000000000000000000000000000000,
            0xFC000000000000000000000000000000,
            0xFE000000000000000000000000000000,
            0xFF000000000000000000000000000000,
            0xFF800000000000000000000000000000,
            0xFFC00000000000000000000000000000,
            0xFFE00000000000000000000000000000,
            0xFFF00000000000000000000000000000,
            0xFFF80000000000000000000000000000,
            0xFFFC0000000000000000000000000000,
            0xFFFE0000000000000000000000000000,
            0xFFFF0000000000000000000000000000,
            0xFFFF8000000000000000000000000000,
            0xFFFFC000000000000000000000000000,
            0xFFFFE000000000000000000000000000,
            0xFFFFF000000000000000000000000000,
            0xFFFFF800000000000000000000000000,
            0xFFFFFC00000000000000000000000000,
            0xFFFFFE00000000000000000000000000,
            0xFFFFFF00000000000000000000000000,
            0xFFFFFF80000000000000000000000000,
            0xFFFFFFC0000000000000000000000000,
            0xFFFFFFE0000000000000000000000000,
            0xFFFFFFF0000000000000000000000000,
            0xFFFFFFF8000000000000000000000000,
            0xFFFFFFFC000000000000000000000000,
            0xFFFFFFFE000000000000000000000000,
            0xFFFFFFFF000000000000000000000000,
            0xFFFFFFFF800000000000000000000000,
            0xFFFFFFFFC00000000000000000000000,
            0xFFFFFFFFE00000000000000000000000,
            0xFFFFFFFFF00000000000000000000000,
            0xFFFFFFFFF80000000000000000000000,
            0xFFFFFFFFFC0000000000000000000000,
            0xFFFFFFFFFE0000000000000000000000,
            0xFFFFFFFFFF0000000000000000000000,
            0xFFFFFFFFFF8000000000000000000000,
            0xFFFFFFFFFFC000000000000000000000,
            0xFFFFFFFFFFE000000000000000000000,
            0xFFFFFFFFFFF000000000000000000000,
            0xFFFFFFFFFFF800000000000000000000,
            0xFFFFFFFFFFFC00000000000000000000,
            0xFFFFFFFFFFFE00000000000000000000,
            0xFFFFFFFFFFFF00000000000000000000,
            0xFFFFFFFFFFFF80000000000000000000,
            0xFFFFFFFFFFFFC0000000000000000000,
            0xFFFFFFFFFFFFE0000000000000000000,
            0xFFFFFFFFFFFFF0000000000000000000,
            0xFFFFFFFFFFFFF8000000000000000000,
            0xFFFFFFFFFFFFFC000000000000000000,
            0xFFFFFFFFFFFFFE000000000000000000,
            0xFFFFFFFFFFFFFF000000000000000000,
            0xFFFFFFFFFFFFFF800000000000000000,
            0xFFFFFFFFFFFFFFC00000000000000000,
            0xFFFFFFFFFFFFFFE00000000000000000,
            0xFFFFFFFFFFFFFFF00000000000000000,
            0xFFFFFFFFFFFFFFF80000000000000000,
            0xFFFFFFFFFFFFFFFC0000000000000000,
            0xFFFFFFFFFFFFFFFE0000000000000000,
            0xFFFFFFFFFFFFFFFF0000000000000000,
            0xFFFFFFFFFFFFFFFF8000000000000000,
            0xFFFFFFFFFFFFFFFFC000000000000000,
            0xFFFFFFFFFFFFFFFFE000000000000000,
            0xFFFFFFFFFFFFFFFFF000000000000000,
            0xFFFFFFFFFFFFFFFFF800000000000000,
            0xFFFFFFFFFFFFFFFFFC00000000000000,
            0xFFFFFFFFFFFFFFFFFE00000000000000,
            0xFFFFFFFFFFFFFFFFFF00000000000000,
            0xFFFFFFFFFFFFFFFFFF80000000000000,
            0xFFFFFFFFFFFFFFFFFFC0000000000000,
            0xFFFFFFFFFFFFFFFFFFE0000000000000,
            0xFFFFFFFFFFFFFFFFFFF0000000000000,
            0xFFFFFFFFFFFFFFFFFFF8000000000000,
            0xFFFFFFFFFFFFFFFFFFFC000000000000,
            0xFFFFFFFFFFFFFFFFFFFE000000000000,
            0xFFFFFFFFFFFFFFFFFFFF000000000000,
            0xFFFFFFFFFFFFFFFFFFFF800000000000,
            0xFFFFFFFFFFFFFFFFFFFFC00000000000,
            0xFFFFFFFFFFFFFFFFFFFFE00000000000,
            0xFFFFFFFFFFFFFFFFFFFFF00000000000,
            0xFFFFFFFFFFFFFFFFFFFFF80000000000,
            0xFFFFFFFFFFFFFFFFFFFFFC0000000000,
            0xFFFFFFFFFFFFFFFFFFFFFE0000000000,
            0xFFFFFFFFFFFFFFFFFFFFFF0000000000,
            0xFFFFFFFFFFFFFFFFFFFFFF8000000000,
            0xFFFFFFFFFFFFFFFFFFFFFFC000000000,
            0xFFFFFFFFFFFFFFFFFFFFFFE000000000,
            0xFFFFFFFFFFFFFFFFFFFFFFF000000000,
            0xFFFFFFFFFFFFFFFFFFFFFFF800000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFC00000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFE00000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFF00000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFF80000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFC0000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFE0000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFF0000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFF8000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFC000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFE000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFF000000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFF800000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFC00000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFE00000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFF00000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFF80000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFC0000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFE0000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF8000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFC000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFE000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFF800,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC00,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE00,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF80,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC0,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE0,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE,
            0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
        ];

        let mask = MASKS[len as usize].to_be_bytes();
        Self { mask }
    }
    /// Constructs a subnet mask from the provided bytes, if they represent a valid mask.
    pub fn from_bytes(bytes: [u8; 16]) -> Option<Self> {
        Self::from_u128(u128::from_be_bytes(bytes))
    }
    /// Constructs a subnet mask from the provided segments, if they represent a valid mask.
    pub fn from_segments(segments: [u16; 8]) -> Option<Self> {
        Self::from_bytes(Ipv6Addr::from(segments).octets())
    }
    /// Constructs a subnet mask from the provided u128, if it represents a valid mask.
    pub fn from_u128(x: u128) -> Option<Self> {
        let ones = if cfg!(target_feature = "popcnt") {
            x.count_ones() as u8
        } else {
            (!x).leading_zeros() as u8
        };
        let zeros = x.trailing_zeros() as u8;
        if ones + zeros == 128 {
            let mask = x.to_be_bytes();
            Some(Self { mask })
        } else {
            None
        }
    }
    /// Returns the subnet mask as an array of bytes.
    pub const fn octets(self) -> [u8; 16] {
        self.mask
    }
    /// Returns the subnet mask as an array of segments.
    pub fn segments(self) -> [u16; 8] {
        Ipv6Addr::from(self.mask).segments()
    }
    /// Returns th subnet mask as a native-endian u128.
    pub const fn as_u128(self) -> u128 {
        u128::from_be_bytes(self.octets())
    }
    /// Returns the length of the mask. That is, the number of 1 bits in this mask.
    pub const fn len(self) -> u8 {
        let x = self.as_u128();
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

impl Display for Ipv6Mask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "/{}", self.len())
    }
}

impl Debug for Ipv4Mask {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Debug for Ipv6Mask {
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

impl Not for Ipv6Mask {
    type Output = [u8; 16];
    fn not(self) -> [u8; 16] {
        let x = u128::from_ne_bytes(self.octets());
        (!x).to_ne_bytes()
    }
}

impl FromStr for Ipv4Mask {
    type Err = InvalidIpv4Mask;
    fn from_str(s: &str) -> Result<Self, InvalidIpv4Mask> {
        if s.starts_with('/') {
            // let len = &s[1..];
            match s[1..].parse::<u8>() {
                Ok(len @ 0..=32) => Ok(Ipv4Mask::new(len)),
                _ => Err(InvalidIpv4Mask),
            }
        } else {
            let bytes = s.parse::<Ipv4Addr>().map_err(|_| InvalidIpv4Mask)?.octets();
            Self::from_bytes(bytes).ok_or(InvalidIpv4Mask)
        }
    }
}

impl FromStr for Ipv6Mask {
    type Err = InvalidIpv6Mask;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('/') {
            match s[1..].parse::<u8>() {
                Ok(len @ 0..=128) => Ok(Ipv6Mask::new(len)),
                _ => Err(InvalidIpv6Mask),
            }
        } else {
            let bytes = s.parse::<Ipv6Addr>().map_err(|_| InvalidIpv6Mask)?.octets();
            Self::from_bytes(bytes).ok_or(InvalidIpv6Mask)
        }
    }
}
/// Error when failing to parse an Ipv4Mask.
#[derive(Debug)]
pub struct InvalidIpv4Mask;
/// Error when failing to parse an Ipv6Mask.
#[derive(Debug)]
pub struct InvalidIpv6Mask;
/// An 8-byte type representing an IPv4 address and subnet mask pair. The IP may be any IP
/// within the represented network, and the mask may be any valid subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MaskedIpv4 {
    /// The IP address
    pub ip: Ipv4Addr,
    /// The subnet mask
    pub mask: Ipv4Mask,
}
/// A 32-byte type representing an IPv6 address and subnet mask pair. The IP may be any IP
/// within the represented network, and the mask may be any valid subnet mask.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MaskedIpv6 {
    /// The IP address
    pub ip: Ipv6Addr,
    /// The subnet mask
    pub mask: Ipv6Mask,
}

impl MaskedIpv4 {
    /// Constructs a MaskedIpv4 from the provided IP and mask.
    pub const fn new(ip: Ipv4Addr, mask: Ipv4Mask) -> Self {
        Self { ip, mask }
    }
    /// Constructs a MaskedIpv4 from the provided IP and mask length.
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
    /// Constructs a new MaskedIpv4 from the provided IP and subnet mask. There must be exactly one space between the IP and mask.
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
    /// Returns the number of networks of the provided mask length will fit in this network as u64. Unlike network_count, this will not panic
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
    /// Returns true if this network contains the provided IP address, even if the provided IP is the network or broadcast address.
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        self.ip.bitand(self.mask) == ip.bitand(self.mask)
    }
}

impl MaskedIpv6 {
    /// Constructs a MaskedIpv6 from the provided IP and mask.
    pub const fn new(ip: Ipv6Addr, mask: Ipv6Mask) -> Self {
        Self { ip, mask }
    }
    /// Constructs a MaskedIpv6 from the provided IP and mask length.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length > 128
    pub const fn cidr(ip: Ipv6Addr, mask_len: u8) -> Self {
        let mask = Ipv6Mask::new(mask_len);
        Self::new(ip, mask)
    }
    /// Constructs a MaskedIpv6 from the provided CIDR string.
    pub fn from_cidr_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let ip = parts.next()?.parse::<Ipv6Addr>().ok()?;
        let mask_len = parts.next()?.parse::<u8>().ok()?;
        if mask_len > 128 {
            None
        } else {
            Some(Self::cidr(ip, mask_len))
        }
    }
    /// Returns a String with the IP and mask in CIDR format. Shortcut for `format!("{}", self)`
    pub fn to_cidr_str(&self) -> String {
        format!("{}", self)
    }
    /// Returns the "network" address by setting all host bits to 0.
    ///
    /// Note: Ipv6 does not technically have a "network" address, however this method can still
    /// be useful to determine the base address of a network.
    pub fn network_address(&self) -> Ipv6Addr {
        self.ip.bitand(self.mask)
    }
    /// Constructs a new MaskedIpv6 using the network address and mask of this MaskedIpv6.
    pub fn network(&self) -> MaskedIpv6 {
        Self::new(self.network_address(), self.mask)
    }
    /// Returns true if all host bits in the IP are 0. Always returns false if the mask len is 127 or 128.
    pub fn is_network_address(&self) -> bool {
        self.mask.len() <= 126 && self.ip == self.network_address()
    }
    /// Returns the number of network bits. That is, the length of the mask.
    pub fn network_bits(&self) -> u8 {
        self.mask.len()
    }
    /// Returns the number of host bits. That is, the number of 0 bits in the mask.
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
    /// Returns the number of networks of the provided mask length will fit in this network.
    ///
    /// # Panics
    ///
    /// Will panic if the provided length is > 128, or if the number of networks does not fit in usize

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

impl Display for MaskedIpv6 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}/{}", self.ip, self.mask.len())
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

impl FromStr for MaskedIpv4 {
    type Err = InvalidMaskedIpv4;
    fn from_str(s: &str) -> Result<Self, InvalidMaskedIpv4> {
        let mut cidr = false;
        let split_index = s
            .find(|ch| {
                if ch == ' ' {
                    cidr = false;
                    true
                } else if ch == '/' {
                    cidr = true;
                    true
                } else {
                    false
                }
            })
            .ok_or(InvalidMaskedIpv4)?;
        let (ip, mask) = s.split_at(split_index);
        let ip = ip.parse::<Ipv4Addr>().map_err(|_| InvalidMaskedIpv4)?;
        let mask = &mask[1..];
        let mask = if cidr {
            let len = mask.parse::<u8>().map_err(|_| InvalidMaskedIpv4)?;
            if len > 32 {
                return Err(InvalidMaskedIpv4);
            }
            Ipv4Mask::new(len)
        } else {
            mask.parse::<Ipv4Mask>().map_err(|_| InvalidMaskedIpv4)?
        };
        Ok(Self::new(ip, mask))
    }
}

impl FromStr for MaskedIpv6 {
    type Err = InvalidMaskedIpv6;
    fn from_str(s: &str) -> Result<Self, InvalidMaskedIpv6> {
        Self::from_cidr_str(s).ok_or(InvalidMaskedIpv6)
    }
}
/// Error when failing to parse a MaskedIpv4.
#[derive(Debug)]
pub struct InvalidMaskedIpv4;
/// Error when failing to parse a MaskedIpv6.
#[derive(Debug)]
pub struct InvalidMaskedIpv6;
