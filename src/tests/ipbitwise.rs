use crate::*;

use std::net::Ipv4Addr;

#[test]
fn bitand_ipv4() {
    let ip = Ipv4Addr::new(0x77, 255, 255, 255);
    assert_eq!(ip.bitand(0xF8_00_FF_00), Ipv4Addr::new(0x70, 0, 255, 0));
    assert_eq!(ip.bitand([0xF8, 0, 255, 0]), Ipv4Addr::new(0x70, 0, 255, 0));
    assert_eq!(ip.bitand(Ipv4Addr::new(0xF8, 0, 255, 0)), Ipv4Addr::new(0x70, 0, 255, 0));
}
#[test]
fn bitor_ipv4() {
    let ip = Ipv4Addr::new(0x77, 0xFF, 0x80, 0x33);
    assert_eq!(ip.bitor(0x80_00_0F_78), Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B));
    assert_eq!(ip.bitor([0x80, 0x00, 0x0F, 0x78]), Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B));
    assert_eq!(ip.bitor(Ipv4Addr::new(0x80, 0x00, 0x0F, 0x78)), Ipv4Addr::new(0xF7, 0xFF, 0x8F, 0x7B));
}
// #[test]
// fn bitxor_ipv4() {

// }