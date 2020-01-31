use crate::Ipv4Mask;

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
