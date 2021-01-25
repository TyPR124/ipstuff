#![cfg(feature = "serde")]

use std::{
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
};

use serde::Serialize;

use ipstuff::{se::alternate, IpBitwiseExt, Ipv4Mask, Ipv6Mask, MaskedIpv4, MaskedIpv6, NetworkV4};

#[derive(Serialize)]
#[serde(transparent)]
struct Alt<T: Display + Serialize>(#[serde(serialize_with = "alternate")] T);

#[test]
fn serde_maskv4_human_readable() {
    let mask: Ipv4Mask = serde_yaml::from_str("255.255.255.0").unwrap();
    let mask2: Ipv4Mask = serde_yaml::from_str("/24").unwrap();
    assert_eq!(mask.len(), 24);
    assert_eq!(mask, mask2);
    assert_eq!(serde_yaml::to_string(&mask).unwrap(), "---\n/24");
    assert_eq!(
        serde_yaml::to_string(&Alt(mask)).unwrap(),
        "---\n255.255.255.0"
    );
}
#[test]
fn serde_maskv4_non_human_readable() {
    for len in 0..=32 {
        let mask: Ipv4Mask = bincode::deserialize(&[len]).unwrap();
        assert_eq!(mask.len(), len);
        assert_eq!(bincode::serialize(&mask).unwrap(), &[len]);
    }
    bincode::deserialize::<Ipv4Mask>(&[33]).unwrap_err();
}
#[test]
fn serde_maskv6_human_readable() {
    let mask: Ipv6Mask = serde_yaml::from_str("'ffff:ff80::'").unwrap();
    let mask2: Ipv6Mask = serde_yaml::from_str("/25").unwrap();
    assert_eq!(mask.len(), 25);
    assert_eq!(mask, mask2);
    assert_eq!(serde_yaml::to_string(&mask).unwrap(), "---\n/25");
    assert_eq!(
        serde_yaml::to_string(&Alt(mask)).unwrap(),
        "---\n\"ffff:ff80::\""
    );
}
#[test]
fn serde_maskv6_non_human_readable() {
    for len in 0..=128 {
        let mask: Ipv6Mask = bincode::deserialize(&[len]).unwrap();
        assert_eq!(mask.len(), len);
        assert_eq!(bincode::serialize(&mask).unwrap(), &[len]);
    }
    bincode::deserialize::<Ipv6Mask>(&[129]).unwrap_err();
}
#[test]
fn serde_maskedv4_human_readable() {
    let masked: MaskedIpv4 = serde_yaml::from_str("192.168.1.1 255.255.255.0").unwrap();
    let masked2: MaskedIpv4 = serde_yaml::from_str("192.168.1.1/24").unwrap();
    assert_eq!(
        masked,
        MaskedIpv4::cidr(Ipv4Addr::new(192, 168, 1, 1), 24).unwrap()
    );
    assert_eq!(masked, masked2);
    assert_eq!(
        serde_yaml::to_string(&masked).unwrap(),
        "---\n192.168.1.1/24"
    );
    assert_eq!(
        serde_yaml::to_string(&Alt(masked)).unwrap(),
        "---\n192.168.1.1 255.255.255.0"
    );
}
#[test]
fn serde_maskedv4_non_human_readable() {
    for len in 0..=32 {
        let mask = Ipv4Mask::new(len).unwrap();
        let ip = Ipv4Addr::new(10, 1, 2, 3);
        let masked = MaskedIpv4::new(ip, mask);
        let bytes = bincode::serialize(&masked).unwrap();
        assert_eq!(bytes.len(), 5);
        assert_eq!(len, bytes[0]);
        assert_eq!(&ip.octets()[..], &bytes[1..]);
        let masked2 = bincode::deserialize(&bytes).unwrap();
        assert_eq!(masked, masked2);
    }
}
#[test]
fn serde_maskedv6_human_readable() {
    let masked: MaskedIpv6 = serde_yaml::from_str("'fe80:: ffc0::'").unwrap();
    let masked2: MaskedIpv6 = serde_yaml::from_str("'fe80::/10'").unwrap();
    assert_eq!(
        masked,
        MaskedIpv6::cidr(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10).unwrap()
    );
    assert_eq!(masked, masked2);
    assert_eq!(
        serde_yaml::to_string(&masked).unwrap(),
        "---\n\"fe80::/10\""
    );
    assert_eq!(
        serde_yaml::to_string(&Alt(masked)).unwrap(),
        "---\n\"fe80:: ffc0::\""
    );
}
#[test]
fn serde_maskedv6_non_human_readable() {
    for len in 0..=32 {
        let mask = Ipv6Mask::new(len).unwrap();
        let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 1, 2, 3);
        let masked = MaskedIpv6::new(ip, mask);
        let bytes = bincode::serialize(&masked).unwrap();
        assert_eq!(bytes.len(), 17);
        assert_eq!(len, bytes[0]);
        assert_eq!(&ip.octets()[..], &bytes[1..]);
        let masked2 = bincode::deserialize(&bytes).unwrap();
        assert_eq!(masked, masked2);
    }
}
#[test]
fn serde_networkv4_human_readable() {
    let net: NetworkV4 = serde_yaml::from_str("192.168.1.0 255.255.255.0").unwrap();
    let net2: NetworkV4 = serde_yaml::from_str("192.168.1.0/24").unwrap();
    assert_eq!(
        net,
        NetworkV4::cidr(Ipv4Addr::new(192, 168, 1, 1), 24).unwrap()
    );
    assert_eq!(net, net2);
    assert_eq!(serde_yaml::to_string(&net).unwrap(), "---\n192.168.1.0/24");
    assert_eq!(
        serde_yaml::to_string(&Alt(net)).unwrap(),
        "---\n192.168.1.0 255.255.255.0"
    );
}
#[test]
fn serde_networkv4_non_human_readable() {
    let ip = Ipv4Addr::new(1, 2, 3, 4);
    for len in 0..=32 {
        let mask = Ipv4Mask::new(len).unwrap();
        let net = NetworkV4::new(ip, mask);
        let bytes = bincode::serialize(&net).unwrap();
        assert_eq!(bytes[0], len);
        assert_eq!(bytes.len() - 1, (len as usize + 7) / 8);
        for (i, b) in bytes[1..].iter().copied().enumerate() {
            assert_eq!(b, ip.bitand(mask).octets()[i]);
        }
        let net2 = bincode::deserialize(&bytes).unwrap();
        assert_eq!(net, net2);
    }
}
