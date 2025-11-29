use core::{
    fmt,
    hash::Hash,
    net::{AddrParseError, IpAddr as StdIpAddr, Ipv4Addr as StdIpv4Addr, Ipv6Addr as StdIpv6Addr},
    ops::{BitAnd, BitOr, BitXor, Not},
    str::FromStr,
};

use crate::{Ipv4Mask, Ipv6Mask};

/// This type is designed to be a drop-in replacement for [`std::net::Ipv4Addr`], with
/// the exception of having a stable layout.
///
/// Layout: this type is gaurenteed to be the same representation as a `[u8; 4]`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Ipv4Addr {
    bytes: [u8; 4],
}

/// This type is designed to be a drop-in replacement for [`std::net::Ipv6Addr`], with
/// the exception of having a stable layout.
///
/// Layout: this type is gaurenteed to be the same representation as a `[u8; 16]`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Ipv6Addr {
    bytes: [u8; 16],
}

/// This type is designed to be a drop-in replacement for [`std::net::Ipv6Addr`].
/// Unlike [`Ipv4Addr`] and [`Ipv6Addr`], this type does not make any layout gaurentees.
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl Ipv4Addr {
    pub const BITS: u32 = 32;
    pub const LOCALHOST: Self = Self::new(127, 0, 0, 1);
    pub const UNSPECIFIED: Self = Self::new(0, 0, 0, 0);
    pub const fn from_std(std_ipv4: StdIpv4Addr) -> Self {
        Self {
            bytes: std_ipv4.octets(),
        }
    }
    pub const fn to_std(self) -> StdIpv4Addr {
        StdIpv4Addr::from_octets(self.bytes)
    }
    #[cfg(feature = "nightly")]
    pub const fn as_octets(&self) -> &[u8; 4] {
        &self.bytes
    }
    pub const fn from_bits(bits: u32) -> Self {
        Self::from_std(StdIpv4Addr::from_bits(bits))
    }
    pub const fn from_octets(octets: [u8; 4]) -> Self {
        Self { bytes: octets }
    }
    #[cfg(feature = "nightly")]
    pub const fn is_benchmarking(&self) -> bool {
        self.to_std().is_benchmarking()
    }
    pub const fn is_broadcast(&self) -> bool {
        self.to_std().is_broadcast()
    }
    pub const fn is_documentation(&self) -> bool {
        self.to_std().is_documentation()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_global(&self) -> bool {
        self.to_std().is_global()
    }
    pub const fn is_link_local(&self) -> bool {
        self.to_std().is_link_local()
    }
    pub const fn is_loopback(&self) -> bool {
        self.to_std().is_loopback()
    }
    pub const fn is_multicast(&self) -> bool {
        self.to_std().is_multicast()
    }
    pub const fn is_private(&self) -> bool {
        self.to_std().is_private()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_reserved(&self) -> bool {
        self.to_std().is_reserved()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_shared(&self) -> bool {
        self.to_std().is_shared()
    }
    pub const fn is_unspecified(&self) -> bool {
        self.to_std().is_unspecified()
    }
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            bytes: [a, b, c, d],
        }
    }
    pub const fn octets(&self) -> [u8; 4] {
        self.bytes
    }
    #[cfg(feature = "nightly")]
    pub fn parse_ascii(b: &[u8]) -> Result<Self, core::net::AddrParseError> {
        Self::from_std(StdIpv4Addr::parse_ascii(b))
    }
    pub const fn to_bits(self) -> u32 {
        self.to_std().to_bits()
    }
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        Ipv6Addr::from_std(self.to_std().to_ipv6_compatible())
    }
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        Ipv6Addr::from_std(self.to_std().to_ipv6_mapped())
    }
}

impl Ipv6Addr {
    pub const BITS: u32 = 128;
    pub const LOCALHOST: Self = Self::new(0, 0, 0, 0, 0, 0, 0, 1);
    pub const UNSPECIFIED: Self = Self::new(0, 0, 0, 0, 0, 0, 0, 0);
    pub const fn from_std(std_ipv6: StdIpv6Addr) -> Self {
        Self {
            bytes: std_ipv6.octets(),
        }
    }
    pub const fn to_std(self) -> StdIpv6Addr {
        StdIpv6Addr::from_octets(self.bytes)
    }
    #[cfg(feature = "nightly")]
    pub const fn as_octets(&self) -> &[u8; 16] {
        &self.bytes
    }
    pub const fn from_bits(bits: u128) -> Self {
        Self::from_std(StdIpv6Addr::from_bits(bits))
    }
    pub const fn from_octets(octets: [u8; 16]) -> Self {
        Self { bytes: octets }
    }
    pub const fn from_segments(segments: [u16; 8]) -> Self {
        Self::from_std(StdIpv6Addr::from_segments(segments))
    }
    #[cfg(feature = "nightly")]
    pub const fn is_benchmarking(&self) -> bool {
        self.to_std().is_benchmarking()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_documentation(&self) -> bool {
        self.to_std().is_documentation()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_global(&self) -> bool {
        self.to_std().is_global()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_ipv4_mapped(&self) -> bool {
        self.to_std().is_ipv4_mapped()
    }
    pub const fn is_loopback(&self) -> bool {
        self.to_std().is_loopback()
    }
    pub const fn is_multicast(&self) -> bool {
        self.to_std().is_multicast()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_unicast(&self) -> bool {
        self.to_std().is_unicast()
    }
    #[cfg(feature = "nightly")]
    pub const fn is_unicast_global(&self) -> bool {
        self.to_std().is_unicast_global()
    }
    pub const fn is_unicast_link_local(&self) -> bool {
        self.to_std().is_unicast_link_local()
    }
    pub const fn is_unspecified(&self) -> bool {
        self.to_std().is_unspecified()
    }
    #[cfg(feature = "nightly")]
    pub const fn multicast_scope(&self) -> Option<core::net::Ipv6MulticastScope> {
        self.to_std().multicast_scope()
    }
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self::from_std(StdIpv6Addr::new(a, b, c, d, e, f, g, h))
    }
    pub const fn octets(&self) -> [u8; 16] {
        self.bytes
    }
    #[cfg(feature = "nightly")]
    pub fn parse_ascii(b: &[u8]) -> Result<Self, core::net::AddrParseError> {
        Self::from_std(StdIpv6Addr::parse_ascii(b))
    }
    pub const fn segments(&self) -> [u16; 8] {
        self.to_std().segments()
    }
    pub const fn to_bits(self) -> u128 {
        self.to_std().to_bits()
    }
    pub const fn to_canonical(&self) -> IpAddr {
        IpAddr::from_std(self.to_std().to_canonical())
    }
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        match self.to_std().to_ipv4() {
            Some(v4) => Some(Ipv4Addr::from_std(v4)),
            None => None,
        }
    }
    pub const fn to_ipv4_mapped(&self) -> Option<Ipv4Addr> {
        match self.to_std().to_ipv4_mapped() {
            Some(v4) => Some(Ipv4Addr::from_std(v4)),
            None => None,
        }
    }
}

impl IpAddr {
    pub const fn from_std(std_ip: StdIpAddr) -> Self {
        match std_ip {
            StdIpAddr::V4(v4) => Self::V4(Ipv4Addr::from_std(v4)),
            StdIpAddr::V6(v6) => Self::V6(Ipv6Addr::from_std(v6)),
        }
    }
    pub const fn to_std(self) -> StdIpAddr {
        match self {
            IpAddr::V4(ipv4_addr) => StdIpAddr::V4(ipv4_addr.to_std()),
            IpAddr::V6(ipv6_addr) => StdIpAddr::V6(ipv6_addr.to_std()),
        }
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_std(), f)
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_std(), f)
    }
}

impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_std(), f)
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_std(), f)
    }
}

impl fmt::Debug for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_std(), f)
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_std(), f)
    }
}

impl Hash for Ipv4Addr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self.to_std(), state)
    }
}

impl Ord for Ipv4Addr {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Ord::cmp(&self.to_std(), &other.to_std())
    }
}

impl PartialOrd for Ipv4Addr {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.to_std(), &other.to_std())
    }
}

impl Hash for Ipv6Addr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self.to_std(), state)
    }
}

impl Ord for Ipv6Addr {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Ord::cmp(&self.to_std(), &other.to_std())
    }
}

impl PartialOrd for Ipv6Addr {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.to_std(), &other.to_std())
    }
}

impl From<[u8; 4]> for Ipv4Addr {
    fn from(value: [u8; 4]) -> Self {
        Self::from_std(StdIpv4Addr::from(value))
    }
}

impl From<u32> for Ipv4Addr {
    fn from(value: u32) -> Self {
        Self::from_std(StdIpv4Addr::from(value))
    }
}

impl From<Ipv4Addr> for u32 {
    fn from(value: Ipv4Addr) -> Self {
        value.to_std().into()
    }
}

impl FromStr for Ipv4Addr {
    type Err = AddrParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StdIpv4Addr::from_str(s).map(Self::from_std)
    }
}

impl From<Ipv6Addr> for u128 {
    fn from(value: Ipv6Addr) -> Self {
        value.to_std().into()
    }
}

impl From<[u8; 16]> for Ipv6Addr {
    fn from(value: [u8; 16]) -> Self {
        Self::from_std(StdIpv6Addr::from(value))
    }
}

impl From<[u16; 8]> for Ipv6Addr {
    fn from(value: [u16; 8]) -> Self {
        Self::from_std(StdIpv6Addr::from(value))
    }
}

impl From<u128> for Ipv6Addr {
    fn from(value: u128) -> Self {
        Self::from_std(StdIpv6Addr::from(value))
    }
}

impl FromStr for Ipv6Addr {
    type Err = AddrParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StdIpv6Addr::from_str(s).map(Self::from_std)
    }
}

impl From<Ipv4Addr> for IpAddr {
    fn from(value: Ipv4Addr) -> Self {
        Self::V4(value)
    }
}

impl From<Ipv6Addr> for IpAddr {
    fn from(value: Ipv6Addr) -> Self {
        Self::V6(value)
    }
}

impl From<StdIpv4Addr> for IpAddr {
    fn from(value: StdIpv4Addr) -> Self {
        Self::V4(value.into())
    }
}

impl From<StdIpv6Addr> for IpAddr {
    fn from(value: StdIpv6Addr) -> Self {
        Self::V6(value.into())
    }
}

impl From<StdIpAddr> for IpAddr {
    fn from(value: StdIpAddr) -> Self {
        match value {
            StdIpAddr::V4(v4) => IpAddr::V4(v4.into()),
            StdIpAddr::V6(v6) => IpAddr::V6(v6.into()),
        }
    }
}

impl From<IpAddr> for StdIpAddr {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(v4) => StdIpAddr::V4(v4.into()),
            IpAddr::V6(v6) => StdIpAddr::V6(v6.into()),
        }
    }
}

impl From<Ipv4Addr> for StdIpv4Addr {
    fn from(value: Ipv4Addr) -> Self {
        value.to_std()
    }
}

impl From<StdIpv4Addr> for Ipv4Addr {
    fn from(value: StdIpv4Addr) -> Self {
        Self::from_std(value)
    }
}

impl From<Ipv6Addr> for StdIpv6Addr {
    fn from(value: Ipv6Addr) -> Self {
        value.to_std()
    }
}

impl From<StdIpv6Addr> for Ipv6Addr {
    fn from(value: StdIpv6Addr) -> Self {
        Self::from_std(value)
    }
}

impl From<[u8; 4]> for IpAddr {
    fn from(value: [u8; 4]) -> Self {
        Self::V4(Ipv4Addr::from(value))
    }
}

impl From<[u8; 16]> for IpAddr {
    fn from(value: [u8; 16]) -> Self {
        Self::V6(Ipv6Addr::from(value))
    }
}

impl From<[u16; 8]> for IpAddr {
    fn from(value: [u16; 8]) -> Self {
        Self::V6(Ipv6Addr::from(value))
    }
}

impl FromStr for IpAddr {
    type Err = AddrParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StdIpAddr::from_str(s).map(Self::from_std)
    }
}

impl BitAnd<Ipv4Addr> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitand(self, rhs: Ipv4Addr) -> Self::Output {
        Self::from_bits(self.to_bits() & rhs.to_bits())
    }
}

impl BitAnd<Ipv4Mask> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitand(self, rhs: Ipv4Mask) -> Self::Output {
        Self::from_bits(self.to_bits() & rhs.to_bits())
    }
}

impl BitAnd<[u8; 4]> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitand(self, rhs: [u8; 4]) -> Self::Output {
        self & Self::from_octets(rhs)
    }
}

impl BitAnd<u32> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitand(self, rhs: u32) -> Self::Output {
        self & Self::from(rhs)
    }
}

impl BitOr<Ipv4Addr> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitor(self, rhs: Ipv4Addr) -> Self::Output {
        Self::from_bits(self.to_bits() | rhs.to_bits())
    }
}

impl BitOr<Ipv4Mask> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitor(self, rhs: Ipv4Mask) -> Self::Output {
        Self::from_bits(self.to_bits() | rhs.to_bits())
    }
}

impl BitOr<[u8; 4]> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitor(self, rhs: [u8; 4]) -> Self::Output {
        self | Self::from_octets(rhs)
    }
}

impl BitOr<u32> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitor(self, rhs: u32) -> Self::Output {
        self | Self::from(rhs)
    }
}

impl BitXor<Ipv4Addr> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitxor(self, rhs: Ipv4Addr) -> Self::Output {
        Self::from_bits(self.to_bits() ^ rhs.to_bits())
    }
}

impl BitXor<Ipv4Mask> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitxor(self, rhs: Ipv4Mask) -> Self::Output {
        Self::from_bits(self.to_bits() ^ rhs.to_bits())
    }
}

impl BitXor<[u8; 4]> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitxor(self, rhs: [u8; 4]) -> Self::Output {
        self ^ Self::from_octets(rhs)
    }
}

impl BitXor<u32> for Ipv4Addr {
    type Output = Ipv4Addr;
    fn bitxor(self, rhs: u32) -> Self::Output {
        self ^ Self::from(rhs)
    }
}

impl Not for Ipv4Addr {
    type Output = Ipv4Addr;
    fn not(self) -> Self::Output {
        Self::from_bits(!self.to_bits())
    }
}

impl BitAnd<Ipv6Addr> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitand(self, rhs: Ipv6Addr) -> Self::Output {
        Self::from_bits(self.to_bits() & rhs.to_bits())
    }
}

impl BitAnd<Ipv6Mask> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitand(self, rhs: Ipv6Mask) -> Self::Output {
        Self::from_bits(self.to_bits() & rhs.to_bits())
    }
}

impl BitAnd<[u8; 16]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitand(self, rhs: [u8; 16]) -> Self::Output {
        self & Self::from_octets(rhs)
    }
}

impl BitAnd<[u16; 8]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitand(self, rhs: [u16; 8]) -> Self::Output {
        self & Self::from_segments(rhs)
    }
}

impl BitAnd<u128> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitand(self, rhs: u128) -> Self::Output {
        self & Self::from(rhs)
    }
}

impl BitOr<Ipv6Addr> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitor(self, rhs: Ipv6Addr) -> Self::Output {
        Self::from_bits(self.to_bits() | rhs.to_bits())
    }
}

impl BitOr<Ipv6Mask> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitor(self, rhs: Ipv6Mask) -> Self::Output {
        Self::from_bits(self.to_bits() | rhs.to_bits())
    }
}

impl BitOr<[u8; 16]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitor(self, rhs: [u8; 16]) -> Self::Output {
        self | Self::from_octets(rhs)
    }
}

impl BitOr<[u16; 8]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitor(self, rhs: [u16; 8]) -> Self::Output {
        self | Self::from_segments(rhs)
    }
}

impl BitOr<u128> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitor(self, rhs: u128) -> Self::Output {
        self | Self::from(rhs)
    }
}

impl BitXor<Ipv6Addr> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitxor(self, rhs: Ipv6Addr) -> Self::Output {
        Self::from_bits(self.to_bits() ^ rhs.to_bits())
    }
}

impl BitXor<Ipv6Mask> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitxor(self, rhs: Ipv6Mask) -> Self::Output {
        Self::from_bits(self.to_bits() ^ rhs.to_bits())
    }
}

impl BitXor<[u8; 16]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitxor(self, rhs: [u8; 16]) -> Self::Output {
        self ^ Self::from_octets(rhs)
    }
}

impl BitXor<[u16; 8]> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitxor(self, rhs: [u16; 8]) -> Self::Output {
        self ^ Self::from_segments(rhs)
    }
}

impl BitXor<u128> for Ipv6Addr {
    type Output = Ipv6Addr;
    fn bitxor(self, rhs: u128) -> Self::Output {
        self ^ Self::from(rhs)
    }
}

impl Not for Ipv6Addr {
    type Output = Ipv6Addr;
    fn not(self) -> Self::Output {
        Self::from_bits(!self.to_bits())
    }
}
