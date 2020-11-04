use crate::*;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[test]
fn build_and_display_all_v4_masks() {
    use std::fmt::Write;
    let mut display = String::with_capacity(15);
    for len in 0..=32 {
        let mask = Ipv4Mask::new(len);
        assert_eq!(len, mask.len());
        display.clear();
        write!(display, "{:#}", mask).unwrap();
        assert_eq!(display, format!("/{}", len));
        let bytes = mask.octets();
        let x = u32::from_be_bytes(bytes);
        assert_eq!(x.count_ones(), len as u32);
        assert_eq!(x.trailing_zeros(), 32 - len as u32);
        display.clear();
        write!(display, "{}", mask).unwrap();
        assert_eq!(
            display,
            format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
        );
    }
}
#[test]
fn build_and_display_all_v6_masks() {
    use std::fmt::Write;
    let mut display = String::with_capacity(4);
    for len in 0..=128 {
        let mask = Ipv6Mask::new(len);
        assert_eq!(len, mask.len());
        display.clear();
        write!(display, "{}", mask).unwrap();
        assert_eq!(display, format!("/{}", len));
    }
}
#[test]
fn build_all_v4_masks_from_u32() {
    let mut x = 0;
    for len in 0..=32 {
        let mask = Ipv4Mask::from_u32(x).unwrap();
        assert_eq!(len, mask.len());
        x >>= 1;
        x |= 0x8000_0000;
    }
    // Invalid masks
    assert_eq!(None, Ipv4Mask::from_u32(0xFF));
    assert_eq!(None, Ipv4Mask::from_u32(0xA000_0000));
    assert_eq!(None, Ipv4Mask::from_u32(0xFFFF_F00F));
}
#[test]
fn build_all_v6_masks_from_u128() {
    let mut x = 0;
    for len in 0..=128 {
        let mask = Ipv6Mask::from_u128(x).unwrap();
        assert_eq!(len, mask.len());
        x >>= 1;
        x |= 0x8000_0000_0000_0000_0000_0000_0000_0000;
    }
    // Invalid masks
    assert_eq!(None, Ipv6Mask::from_u128(0xFF));
    assert_eq!(None, Ipv6Mask::from_u128(0xA000_0000));
    assert_eq!(
        None,
        Ipv6Mask::from_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_F00F)
    );
}
#[test]
fn parse_v4_mask_strings() {
    const SLASH_32: &str = "255.255.255.255";
    const SLASH_30: &str = "255.255.255.252";
    const SLASH_12: &str = "255.240.0.0";
    const SLASH_8: &str = "255.0.0.0";
    const SLASH_0: &str = "0.0.0.0";

    const SLASH_32_CIDR: &str = "/32";
    const SLASH_30_CIDR: &str = "/30";
    const SLASH_12_CIDR: &str = "/12";
    const SLASH_8_CIDR: &str = "/8";
    const SLASH_0_CIDR: &str = "/0";

    let m: Ipv4Mask = SLASH_32.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_32);
    assert_eq!(m.len(), 32);
    let m: Ipv4Mask = SLASH_30.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_30);
    assert_eq!(m.len(), 30);
    let m: Ipv4Mask = SLASH_12.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_12);
    assert_eq!(m.len(), 12);
    let m: Ipv4Mask = SLASH_8.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_8);
    assert_eq!(m.len(), 8);
    let m: Ipv4Mask = SLASH_0.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_0);
    assert_eq!(m.len(), 0);

    let m: Ipv4Mask = SLASH_32_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_32_CIDR);
    assert_eq!(m.len(), 32);
    let m: Ipv4Mask = SLASH_30_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_30_CIDR);
    assert_eq!(m.len(), 30);
    let m: Ipv4Mask = SLASH_12_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_12_CIDR);
    assert_eq!(m.len(), 12);
    let m: Ipv4Mask = SLASH_8_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_8_CIDR);
    assert_eq!(m.len(), 8);
    let m: Ipv4Mask = SLASH_0_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_0_CIDR);
    assert_eq!(m.len(), 0);
}
#[test]
fn parse_v6_mask_strings() {
    const SLASH_128: &str = "FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF";
    const SLASH_126: &str = "FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFC";
    const SLASH_12: &str = "FFF0::";
    const SLASH_8: &str = "FF00::";
    const SLASH_0: &str = "::";

    const SLASH_128_CIDR: &str = "/128";
    const SLASH_126_CIDR: &str = "/126";
    const SLASH_12_CIDR: &str = "/12";
    const SLASH_8_CIDR: &str = "/8";
    const SLASH_0_CIDR: &str = "/0";

    let m: Ipv6Mask = SLASH_128.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_128_CIDR);
    assert_eq!(m.len(), 128);
    let m: Ipv6Mask = SLASH_126.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_126_CIDR);
    assert_eq!(m.len(), 126);
    let m: Ipv6Mask = SLASH_12.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_12_CIDR);
    assert_eq!(m.len(), 12);
    let m: Ipv6Mask = SLASH_8.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_8_CIDR);
    assert_eq!(m.len(), 8);
    let m: Ipv6Mask = SLASH_0.parse().unwrap();
    assert_eq!(format!("{}", m), SLASH_0_CIDR);
    assert_eq!(m.len(), 0);

    let m: Ipv6Mask = SLASH_128_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_128_CIDR);
    assert_eq!(m.len(), 128);
    let m: Ipv6Mask = SLASH_126_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_126_CIDR);
    assert_eq!(m.len(), 126);
    let m: Ipv6Mask = SLASH_12_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_12_CIDR);
    assert_eq!(m.len(), 12);
    let m: Ipv6Mask = SLASH_8_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_8_CIDR);
    assert_eq!(m.len(), 8);
    let m: Ipv6Mask = SLASH_0_CIDR.parse().unwrap();
    assert_eq!(format!("{:#}", m), SLASH_0_CIDR);
    assert_eq!(m.len(), 0);
}
#[test]
fn parse_masked_ipv4_strings() {
    const TEN_ONE_8: &str = "10.0.0.1/8";
    const TEN_TWO_24: &str = "10.0.0.2/24";
    const TEN_FOUR_12: &str = "10.0.0.4 255.240.0.0";
    const TEN_FIVE_32: &str = "10.0.0.5 255.255.255.255";

    let net: MaskedIpv4 = TEN_ONE_8.parse().unwrap();
    assert_eq!(format!("{}", net), "10.0.0.1 255.0.0.0");
    let net: MaskedIpv4 = TEN_TWO_24.parse().unwrap();
    assert_eq!(format!("{}", net), "10.0.0.2 255.255.255.0");
    let net: MaskedIpv4 = TEN_FOUR_12.parse().unwrap();
    assert_eq!(format!("{:#}", net), "10.0.0.4/12");
    let net: MaskedIpv4 = TEN_FIVE_32.parse().unwrap();
    assert_eq!(format!("{:#}", net), "10.0.0.5/32");
}
#[test]
fn parse_masked_ipv6_strings() {
    const FE80_ONE_10: &str = "fe80::1/10";
    const LOCALHOST_128: &str = "::/128";
    const FFFF_AAAA_16: &str = "ffff::aaaa/16";

    let net: MaskedIpv6 = FE80_ONE_10.parse().unwrap();
    assert_eq!(format!("{}", net), FE80_ONE_10);
    assert_eq!(net.mask.len(), 10);
    let net: MaskedIpv6 = LOCALHOST_128.parse().unwrap();
    assert_eq!(format!("{}", net), LOCALHOST_128);
    assert_eq!(net.mask.len(), 128);
    let net: MaskedIpv6 = FFFF_AAAA_16.parse().unwrap();
    assert_eq!(format!("{}", net), FFFF_AAAA_16);
    assert_eq!(net.mask.len(), 16);
}
#[test]
fn masked_ipv4_contains() {
    const TEN_FOUR_12: &str = "10.0.0.4 255.240.0.0";
    let net: MaskedIpv4 = TEN_FOUR_12.parse().unwrap();
    assert!(net.contains(Ipv4Addr::new(10, 0, 0, 0)));
    assert!(net.contains(Ipv4Addr::new(10, 0, 0, 1)));
    assert!(net.contains(Ipv4Addr::new(10, 1, 0, 1)));
    assert!(net.contains(Ipv4Addr::new(10, 15, 255, 255)));
    assert!(!net.contains(Ipv4Addr::new(10, 16, 0, 0)));
}
#[test]
fn masked_ipv6_contains() {
    const FE80_ONE_10: &str = "fe80::1/10";
    let net: MaskedIpv6 = FE80_ONE_10.parse().unwrap();
    assert!(net.contains(Ipv6Addr::from_str("FE80:B::").unwrap()));
    assert!(net.contains(Ipv6Addr::from_str("FE80::").unwrap()));
    assert!(net.contains(Ipv6Addr::from_str("fe90::").unwrap()));
    assert!(net.contains(Ipv6Addr::from_str("FEB0::").unwrap()));
    assert!(!net.contains(Ipv6Addr::from_str("fe7f::").unwrap()));
    assert!(!net.contains(Ipv6Addr::from_str("fec0::").unwrap()));
}
#[test]
fn invalid_maskedv4_cidr() {
    assert!(MaskedIpv4::from_cidr_str("192.168.1.1/24/24").is_none());
    assert!(MaskedIpv4::from_cidr_str("8.8.8./32").is_none());
    assert!(MaskedIpv4::from_cidr_str("a/32").is_none());
    assert!(MaskedIpv4::from_cidr_str("192.168.1.1 255.255.255.0").is_none());
    assert!(MaskedIpv4::from_cidr_str("192.168.1.1/33").is_none());
    assert!(MaskedIpv4::from_cidr_str("192.256.1.1/32").is_none());
}
#[test]
fn invalid_maskedv6_cidr() {
    assert!(MaskedIpv6::from_cidr_str("fe80::1/129").is_none());
    assert!(MaskedIpv6::from_cidr_str("192.168.1.1/32").is_none());
    assert!(MaskedIpv6::from_cidr_str("abcd::/0.").is_none());
    assert!(MaskedIpv6::from_cidr_str("::/128/128").is_none());
}
#[test]
fn invalid_maskedv4_network() {
    assert!(MaskedIpv4::from_network_str("192.168.1.1255.255.255.255").is_none());
    assert!(MaskedIpv4::from_network_str("192.168.1.1\t255.255.255.255").is_none());
    assert!(MaskedIpv4::from_network_str("192.168.1.1  255.255.255.255").is_none());
    assert!(MaskedIpv4::from_network_str("192.168.1. 1 255.255.255.255").is_none());
    assert!(MaskedIpv4::from_network_str("192.168.1.1 255.255.255.255 ").is_none());
    assert!(MaskedIpv4::from_network_str("192.168.1.1 255.255.255").is_none());
}
