use crate::*;

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
        assert_eq!(x.trailing_zeros(), 32-len as u32);
        display.clear();
        write!(display, "{}", mask).unwrap();
        assert_eq!(
            display,
            format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
        );
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
fn parse_v4_mask_strings() {
    const SLASH_32: &str = "255.255.255.255";
    const SLASH_30: &str = "255.255.255.252";
    const SLASH_12: &str = "255.240.0.0";
    const SLASH_8: &str = "255.0.0.0";
    const SLASH_0: &str = "0.0.0.0";

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