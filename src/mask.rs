use crate::mk_zst_error_type;

use std::fmt::{self, Debug, Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::Not;
use std::str::FromStr;

/// A 4-byte type representing a subnet mask in big-endian byte-order. This type
/// can only be a valid subnet mask.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ipv4Mask {
    mask: [u8; 4],
}
/// A 16-byte type representing a subnet mask in big-endian byte-order. This
/// type can only be a valid subnet mask.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ipv6Mask {
    mask: [u8; 16],
}
mk_zst_error_type!(InvalidIpv4Mask = "invalid IPv4 mask");
mk_zst_error_type!(InvalidIpv6Mask = "invalid IPv6 mask");
#[test]
fn ipv4mask_is_big_endian() {
    assert_eq!(
        Ipv4Mask::new(9).unwrap(),
        Ipv4Mask::from_bytes([255, 128, 0, 0]).unwrap()
    );
}
#[test]
fn ipv6mask_is_big_endian() {
    assert_eq!(
        Ipv6Mask::new(9).unwrap(),
        Ipv6Mask::from_bytes([255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap()
    );
}
impl Ipv4Mask {
    /// Returns a mask with the specified length, if it is a valid length.
    #[allow(clippy::manual_unwrap_or)] // for const
    pub const fn new(len: u8) -> Option<Self> {
        // let shift = match 32u8.checked_sub(len) {
        //     Some(shift) => shift,
        //     None => return None,
        // };
        // let mask = match u32::MAX.checked_shl(shift as u32) {
        //     Some(mask) => mask,
        //     None => 0,
        // }
        // .to_be_bytes();
        // Some(Self { mask })
        let mask = (!match u32::MAX.checked_shr(len as u32) {
            Some(mask) => mask,
            None if len == 32 => 0,
            None => return None,
        })
        .to_be_bytes();
        Some(Self { mask })
    }
    pub const fn new_unchecked(len: u8) -> Self {
        // Can't use debug_assert!() in const fn
        // debug_assert!(len <= 32);
        let _ = 32 - len;
        Self::new_saturating(len)
    }
    pub const fn new_saturating(len: u8) -> Self {
        let mask = match u32::MAX.checked_shr(len as u32) {
            Some(x) => !x,
            None => {
                return Self {
                    mask: [255, 255, 255, 255],
                }
            }
        }
        .to_be_bytes();
        Self { mask }
    }
    /// Constructs a subnet mask from the provided bytes, if they represent a
    /// valid mask.
    pub const fn from_bytes(bytes: [u8; 4]) -> Option<Self> {
        Self::from_u32(u32::from_be_bytes(bytes))
    }
    /// Constructs a subnet mask from the provided u32, if it represents a valid
    /// mask.
    pub const fn from_u32(x: u32) -> Option<Self> {
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
    /// Constructs a subnet mask from the provided [`Ipv4Addr`], if it
    /// represents a valid mask.
    ///
    /// # Const
    ///
    /// This fn will be made const when [`Ipv4Addr::octets`] becomes const
    /// stable.
    pub fn from_ip(ip: Ipv4Addr) -> Option<Self> {
        Self::from_bytes(ip.octets())
    }
    /// Returns the subnet mask as an array of bytes.
    pub const fn octets(self) -> [u8; 4] {
        self.mask
    }
    /// Returns the subnet mask as a native-endian u32.
    pub const fn as_u32(self) -> u32 {
        u32::from_be_bytes(self.octets())
    }
    /// Returns the length of the mask. That is, the number of 1 bits in this
    /// mask.
    ///
    /// # Optimized
    ///
    /// With `target-feature=+popcnt`
    ///
    /// ```asm
    /// popcnt  eax, edi
    /// ret
    /// ```
    ///
    /// Otherwise
    ///
    /// ```asm
    ///
    /// ```
    pub const fn len(self) -> u8 {
        let x = self.as_u32();
        if cfg!(target_feature = "popcnt") {
            x.count_ones() as u8
        } else if cfg!(target_feature = "lzcnt") {
            (!x).leading_zeros() as u8
        } else {
            32 - x.trailing_zeros() as u8
        }
    }
    pub const fn is_empty(self) -> bool {
        self.octets()[0] == 0
    }
    pub const fn is_full(self) -> bool {
        self.octets()[3] == 255
    }
}
#[allow(clippy::len_without_is_empty)]
impl Ipv6Mask {
    /// Returns a mask with the specified length, if it is a valid length.
    #[allow(clippy::manual_unwrap_or)] // for const
    pub const fn new(len: u8) -> Option<Self> {
        let shift = match 128u8.checked_sub(len) {
            Some(shift) => shift,
            None => return None,
        };
        let mask = match u128::MAX.checked_shl(shift as u32) {
            Some(mask) => mask,
            None => 0,
        }
        .to_be_bytes();
        Some(Self { mask })
    }
    /// Returns a mask with the specified length, if it is valid, or any
    /// unspecified but valid mask if the length is invalid.
    ///
    /// # Safety
    ///
    /// This is both memory and type safe. If an invalid mask length is provided,
    /// the returned mask will still be some valid mask value.
    #[allow(clippy::manual_unwrap_or)] // for const
    pub const fn new_unchecked(len: u8) -> Self {
        let shift = 128u8.saturating_sub(len);
        let mask = match u128::MAX.checked_shl(shift as u32) {
            Some(mask) => mask,
            None => 0,
        }
        .to_be_bytes();
        Self { mask }
    }
    /// Constructs a subnet mask from the provided segments, if they represent a
    /// valid mask.
    pub const fn from_segments(segments: [u16; 8]) -> Option<Self> {
        Self::from_bytes([
            segments[0].to_be_bytes()[0],
            segments[0].to_be_bytes()[1],
            segments[1].to_be_bytes()[0],
            segments[1].to_be_bytes()[1],
            segments[2].to_be_bytes()[0],
            segments[2].to_be_bytes()[1],
            segments[3].to_be_bytes()[0],
            segments[3].to_be_bytes()[1],
            segments[4].to_be_bytes()[0],
            segments[4].to_be_bytes()[1],
            segments[5].to_be_bytes()[0],
            segments[5].to_be_bytes()[1],
            segments[6].to_be_bytes()[0],
            segments[6].to_be_bytes()[1],
            segments[7].to_be_bytes()[0],
            segments[7].to_be_bytes()[1],
        ])
    }
    /// Constructs a subnet mask from the provided bytes, if they represent a
    /// valid mask.
    pub const fn from_bytes(bytes: [u8; 16]) -> Option<Self> {
        Self::from_u128(u128::from_be_bytes(bytes))
    }
    /// Constructs a subnet mask from the provided u128, if it represents a
    /// valid mask.
    pub const fn from_u128(x: u128) -> Option<Self> {
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
    /// Constructs a subnet mask from the provided [`Ipv6Addr`], if it
    /// represents a valid mask.
    pub const fn from_ip(ip: Ipv6Addr) -> Option<Self> {
        Self::from_bytes(ip.octets())
    }
    /// Returns the subnet mask as an array of bytes.
    pub const fn octets(self) -> [u8; 16] {
        self.mask
    }
    /// Returns the subnet mask as an array of segments.
    pub const fn segments(self) -> [u16; 8] {
        [
            u16::from_be_bytes([self.mask[0], self.mask[1]]),
            u16::from_be_bytes([self.mask[2], self.mask[3]]),
            u16::from_be_bytes([self.mask[4], self.mask[5]]),
            u16::from_be_bytes([self.mask[6], self.mask[7]]),
            u16::from_be_bytes([self.mask[8], self.mask[9]]),
            u16::from_be_bytes([self.mask[10], self.mask[11]]),
            u16::from_be_bytes([self.mask[12], self.mask[13]]),
            u16::from_be_bytes([self.mask[14], self.mask[15]]),
        ]
    }
    /// Returns the subnet mask as a native-endian u128.
    pub const fn as_u128(self) -> u128 {
        u128::from_be_bytes(self.octets())
    }
    /// Returns the length of the mask. That is, the number of 1 bits in this
    /// mask.
    pub const fn len(self) -> u8 {
        let x = self.as_u128();
        #[cfg(target_feature = "popcnt")]
        let len = x.count_ones() as u8;
        #[cfg(not(target_feature = "popcnt"))]
        let len = (!x).leading_zeros() as u8;
        len
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
impl Display for Ipv4Mask {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, out: &mut Formatter) -> fmt::Result {
        if out.alternate() {
            let [a, b, c, d] = self.octets();
            write!(out, "{}.{}.{}.{}", a, b, c, d)
        } else {
            write!(out, "/{}", self.len())
        }
    }
}
impl Display for Ipv6Mask {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, out: &mut Formatter) -> fmt::Result {
        if out.alternate() {
            let [a, b, c, d, e, f, g, h] = self.segments();
            write!(out, "{}", Ipv6Addr::new(a, b, c, d, e, f, g, h))
        } else {
            write!(out, "/{}", self.len())
        }
    }
}
impl Debug for Ipv4Mask {
    fn fmt(&self, out: &mut Formatter) -> fmt::Result {
        Display::fmt(self, out)
    }
}
impl Debug for Ipv6Mask {
    fn fmt(&self, out: &mut Formatter) -> fmt::Result {
        Display::fmt(self, out)
    }
}
impl FromStr for Ipv4Mask {
    type Err = InvalidIpv4Mask;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix('/').map(u8::from_str) {
            Some(Ok(len)) => Self::new(len),
            Some(Err(_)) => None,
            None => s.parse::<Ipv4Addr>().ok().and_then(Self::from_ip),
        }
        .ok_or(InvalidIpv4Mask)
    }
}
impl FromStr for Ipv6Mask {
    type Err = InvalidIpv6Mask;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix('/').map(u8::from_str) {
            Some(Ok(len)) => Self::new(len),
            Some(Err(_)) => None,
            None => s.parse::<Ipv6Addr>().ok().and_then(Self::from_ip),
        }
        .ok_or(InvalidIpv6Mask)
    }
}
