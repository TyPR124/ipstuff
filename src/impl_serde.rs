use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    InvalidIpv4Mask, InvalidIpv6Mask, InvalidMaskedIpv4, InvalidMaskedIpv6, Ipv4Mask, Ipv6Mask,
    MaskedIpv4, MaskedIpv6, NetworkV4, NetworkV6,
};

use std::{
    fmt,
    net::{Ipv4Addr, Ipv6Addr},
};

struct FromStrVisitor<T> {
    expecting: &'static str,
    _type: std::marker::PhantomData<T>,
}

impl<'de, T> Visitor<'de> for FromStrVisitor<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: fmt::Display,
{
    type Value = T;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.expecting)
    }
    fn visit_str<E: Error>(self, s: &str) -> Result<T, E> {
        s.parse().map_err(Error::custom)
    }
}
impl<T> FromStrVisitor<T> {
    pub fn expecting(expecting: &'static str) -> Self {
        Self {
            expecting,
            _type: std::marker::PhantomData,
        }
    }
}
impl Serialize for Ipv4Mask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_u8(self.len())
        }
    }
}
impl<'de> Deserialize<'de> for Ipv4Mask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::expecting("IPv4 Mask"))
        } else {
            let len = u8::deserialize(deserializer)?;
            Ipv4Mask::new(len)
                .ok_or(InvalidIpv4Mask)
                .map_err(Error::custom)
        }
    }
}
impl Serialize for Ipv6Mask {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_u8(self.len())
        }
    }
}
impl<'de> Deserialize<'de> for Ipv6Mask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::expecting("IPv6 Mask"))
        } else {
            let len = u8::deserialize(deserializer)?;
            Ipv6Mask::new(len)
                .ok_or(InvalidIpv6Mask)
                .map_err(Error::custom)
        }
    }
}
impl Serialize for MaskedIpv4 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            let [a, b, c, d] = self.ip.octets();
            let len = self.mask.len();
            [len, a, b, c, d].serialize(serializer)
        }
    }
}
impl<'de> Deserialize<'de> for MaskedIpv4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::expecting("Masked IPv4 Address"))
        } else {
            let [len, a, b, c, d] = <[u8; 5]>::deserialize(deserializer)?;
            let mask = Ipv4Mask::new(len)
                .ok_or(InvalidMaskedIpv4)
                .map_err(Error::custom)?;
            let ip = Ipv4Addr::new(a, b, c, d);
            Ok(Self::new(ip, mask))
        }
    }
}
impl Serialize for MaskedIpv6 {
    #[allow(clippy::many_single_char_names)]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = self.ip.octets();
            let len = self.mask.len();
            [len, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p].serialize(serializer)
        }
    }
}
impl<'de> Deserialize<'de> for MaskedIpv6 {
    #[allow(clippy::many_single_char_names)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::expecting("Masked IPv6 Address"))
        } else {
            let [len, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                <[u8; 17]>::deserialize(deserializer)?;
            let mask = Ipv6Mask::new(len)
                .ok_or(InvalidMaskedIpv6)
                .map_err(Error::custom)?;
            let ip = Ipv6Addr::from([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]);
            Ok(Self::new(ip, mask))
        }
    }
}
impl Serialize for NetworkV4 {
    #[allow(clippy::many_single_char_names)]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            let [a, b, c, d] = self.ip().octets();
            let len = self.mask().len();
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&len)?;
            match len {
                0 => tuple.serialize_element(&())?,
                1..=8 => tuple.serialize_element(&a)?,
                9..=16 => tuple.serialize_element(&[a, b])?,
                17..=24 => tuple.serialize_element(&[a, b, c])?,
                25..=32 => tuple.serialize_element(&[a, b, c, d])?,
                _ => unreachable!(),
            }
            tuple.end()
        }
    }
}
impl<'de> Deserialize<'de> for NetworkV4 {
    #[allow(clippy::many_single_char_names)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            // deserializer.deserialize_str(FromStrVisitor::expecting("IPv4 Network"))
            todo!()
        } else {
            deserializer.deserialize_tuple(2, Net4BinaryVisitor)
        }
    }
}
struct Net4BinaryVisitor;
impl<'de> Visitor<'de> for Net4BinaryVisitor {
    type Value = NetworkV4;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("IPv4 Network")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let len: u8 = seq
            .next_element()?
            .ok_or(InvalidMaskedIpv4)
            .map_err(Error::custom)?;
        let mask = Ipv4Mask::new(len)
            .ok_or(InvalidIpv4Mask)
            .map_err(Error::custom)?;
        let ip = match len {
            0 => {
                let _: () = seq
                    .next_element()?
                    .ok_or(InvalidMaskedIpv4)
                    .map_err(Error::custom)?;
                Ipv4Addr::UNSPECIFIED
            }
            1..=8 => {
                let a: u8 = seq
                    .next_element()?
                    .ok_or(InvalidMaskedIpv4)
                    .map_err(Error::custom)?;
                Ipv4Addr::new(a, 0, 0, 0)
            }
            9..=16 => {
                let [a, b]: [u8; 2] = seq
                    .next_element()?
                    .ok_or(InvalidMaskedIpv4)
                    .map_err(Error::custom)?;
                Ipv4Addr::new(a, b, 0, 0)
            }
            17..=24 => {
                let [a, b, c]: [u8; 3] = seq
                    .next_element()?
                    .ok_or(InvalidMaskedIpv4)
                    .map_err(Error::custom)?;
                Ipv4Addr::new(a, b, c, 0)
            }
            25..=32 => {
                let [a, b, c, d]: [u8; 4] = seq
                    .next_element()?
                    .ok_or(InvalidMaskedIpv4)
                    .map_err(Error::custom)?;
                Ipv4Addr::new(a, b, c, d)
            }
            _ => unreachable!(),
        };
        Ok(NetworkV4::new(ip, mask))
    }
}
