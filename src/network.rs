use std::{
    fmt::{self, Display, Formatter},
    net::{Ipv4Addr, Ipv6Addr},
};

use crate::{IpBitwiseExt, Ipv4Mask, Ipv6Mask, MaskedIpv4, MaskedIpv6};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NetworkV4 {
    ip: Ipv4Addr,
    mask: Ipv4Mask,
}

pub struct NetworkV6 {
    ip: Ipv6Addr,
    mask: Ipv6Mask,
}

impl NetworkV4 {
    pub fn new(ip: Ipv4Addr, mask: Ipv4Mask) -> Self {
        let ip = ip.bitand(mask);
        Self { ip, mask }
    }
    pub fn cidr(ip: Ipv4Addr, len: u8) -> Option<Self> {
        Ipv4Mask::new(len).map(|mask| Self::new(ip, mask))
    }
    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }
    pub fn mask(&self) -> Ipv4Mask {
        self.mask
    }
}

impl NetworkV6 {
    pub fn new(ip: Ipv6Addr, mask: Ipv6Mask) -> Self {
        let ip = ip.bitand(mask);
        Self { ip, mask }
    }
    pub fn ip(&self) -> Ipv6Addr {
        self.ip
    }
    pub fn mask(&self) -> Ipv6Mask {
        self.mask
    }
}

impl Display for NetworkV4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{} {:#}", self.ip, self.mask)
        } else {
            write!(f, "{}/{}", self.ip, self.mask.len())
        }
    }
}

impl Display for NetworkV6 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{} {:#}", self.ip, self.mask)
        } else {
            write!(f, "{}/{}", self.ip, self.mask.len())
        }
    }
}
