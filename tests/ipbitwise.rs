use ipstuff::*;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[test]
fn bitand_ipv4() {
    let ip = Ipv4Addr::new(0x77, 255, 255, 255);
    assert_eq!(ip.bitand(0xF8_00_FF_00), Ipv4Addr::new(0x70, 0, 255, 0));
    assert_eq!(ip.bitand([0xF8, 0, 255, 0]), Ipv4Addr::new(0x70, 0, 255, 0));
    assert_eq!(
        ip.bitand(Ipv4Addr::new(0xF8, 0, 255, 0)),
        Ipv4Addr::new(0x70, 0, 255, 0)
    );
    assert_eq!(
        ip.bitand(Ipv4Mask::new(24).unwrap()),
        Ipv4Addr::new(0x77, 0xFF, 0xFF, 0)
    );
}
#[test]
fn bitor_ipv4() {
    let ip = Ipv4Addr::new(0x77, 0xFF, 0x80, 0x33);
    assert_eq!(
        ip.bitor(0x80_00_0F_78),
        Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B)
    );
    assert_eq!(
        ip.bitor([0x80, 0x00, 0x0F, 0x78]),
        Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B)
    );
    assert_eq!(
        ip.bitor(Ipv4Addr::new(0x80, 0x00, 0x0F, 0x78)),
        Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B)
    );
    assert_eq!(
        ip.bitor(Ipv4Mask::new(24).unwrap()),
        Ipv4Addr::new(255, 255, 255, 0x33)
    );
}
#[test]
fn bitxor_ipv4() {
    let ip = Ipv4Addr::new(0x77, 0xFF, 0x80, 0x33);
    assert_eq!(
        ip.bitxor(0xA0_F0_83_78),
        Ipv4Addr::new(0xD7, 0x0F, 0x03, 0x4B)
    );
    assert_eq!(
        ip.bitxor([0xA0, 0xF0, 0x83, 0x78]),
        Ipv4Addr::new(0xD7, 0x0F, 0x03, 0x4B)
    );
    assert_eq!(
        ip.bitxor(Ipv4Addr::new(0xA0, 0xF0, 0x83, 0x78)),
        Ipv4Addr::new(0xD7, 0x0F, 0x03, 0x4B)
    );
    assert_eq!(
        ip.bitxor(Ipv4Mask::new(24).unwrap()),
        Ipv4Addr::new(0x88, 00, 0x7F, 0x33)
    );
}
#[test]
fn bitnot_ipv4() {
    let ip = Ipv4Addr::new(0x77, 0xFF, 0x80, 0x33);
    assert_eq!(ip.bitnot(), Ipv4Addr::new(0x88, 0x00, 0x7F, 0xCC));
}

#[test]
fn bitand_ipv6() {
    let ip = Ipv6Addr::from_str("F9::1").unwrap();
    assert_eq!(
        ip.bitand(0xFFFF_FFFF_FFFF_FFFF_F000_0000_0000_0000),
        Ipv6Addr::from_str("F9::").unwrap()
    );
    assert_eq!(
        ip.bitand([255, 255, 255, 255, 255, 255, 255, 255, 240, 0, 0, 0, 0, 0, 0, 0]),
        Ipv6Addr::from_str("F9::").unwrap()
    );
    assert_eq!(
        ip.bitand([0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xF000, 0, 0, 0]),
        Ipv6Addr::from_str("F9::").unwrap()
    );
    assert_eq!(
        ip.bitand(Ipv6Mask::new(68).unwrap()),
        Ipv6Addr::from_str("F9::").unwrap()
    );
}

#[test]
fn bitor_ipv6() {
    let ip = Ipv6Addr::from_str("F9::1").unwrap();
    assert_eq!(
        ip.bitor(0xFFFF_FFFF_FFFF_FFFF_F000_0000_0000_0000),
        Ipv6Addr::from_str("FFFF:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitor([255, 255, 255, 255, 255, 255, 255, 255, 240, 0, 0, 0, 0, 0, 0, 0]),
        Ipv6Addr::from_str("FFFF:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitor([0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xF000, 0, 0, 0]),
        Ipv6Addr::from_str("FFFF:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitor(Ipv6Mask::new(68).unwrap()),
        Ipv6Addr::from_str("FFFF:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
}

#[test]
fn bitxor_ipv6() {
    let ip = Ipv6Addr::from_str("F9::1").unwrap();
    assert_eq!(
        ip.bitxor(0xFFFF_FFFF_FFFF_FFFF_F000_0000_0000_0000),
        Ipv6Addr::from_str("FF06:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitxor([255, 255, 255, 255, 255, 255, 255, 255, 240, 0, 0, 0, 0, 0, 0, 0]),
        Ipv6Addr::from_str("FF06:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitxor([0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xF000, 0, 0, 0]),
        Ipv6Addr::from_str("FF06:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
    assert_eq!(
        ip.bitxor(Ipv6Mask::new(68).unwrap()),
        Ipv6Addr::from_str("FF06:FFFF:FFFF:FFFF:F000::1").unwrap()
    );
}
