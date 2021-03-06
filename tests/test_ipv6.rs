use ipip::{ipv6, Ipv6AddrPrefixed};
use std::net::Ipv6Addr;

#[test]
fn test_macro() {
    assert_eq!(ipv6!(2001:0DB8:0000:0000:0000:0000:1428:57ab/8), Ipv6AddrPrefixed {
        addr: Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab),
        prefix: 8
    });
    assert_eq!(ipv6!(2001:0DB8:0:0:0:0:1428:57ab),Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab));
    assert_eq!(ipv6!(2001:0DB8:0000:0000:0000::1428:57ab),Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab));
    assert_eq!(ipv6!(2001:0DB8:0000:0000:0000:0000:1428:57ab),Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab));
    assert_eq!(ipv6!(2001:0DB8::1428:57ab),Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab));
    assert_eq!(ipv6!(::),Ipv6Addr::new(0,0,0,0,0,0,0,0));
    assert_eq!(ipv6!(2001:db8::),Ipv6Addr::new(0x2001,0xdb8,0,0,0,0,0,0));
    assert_eq!(ipv6!(::1234:5678),Ipv6Addr::new(0,0,0,0,0,0,0x1234,0x5678));
//    assert_eq!(ipv6!(2001::25de::cade),Ipv6Addr::new(0,0,0,0,0,0,0,0)); // Failed to build
    assert_eq!(ipv6!(::135.75.43.52),Ipv6Addr::new(0,0,0,0,0,0,0x874B,0x2B34));
    assert_eq!(ipv6!(2001:0DB8::135.75.43.52),Ipv6Addr::new(0x2001,0xdb8,0,0,0,0,0x874B,0x2B34));
//    assert_eq!(ipv6!(0DB8:135.75.43.52:1428:57ab),Ipv6Addr::new(0x2001,0xdb8,0,0,0,0,0x874B,0x2B34)); // Failed to build
}

#[test]
fn test_other() {
    assert_eq!(Ipv6AddrPrefixed::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/8").and_then(|a|a.base_address()),
               Some(Ipv6Addr::new(0x2000,0,0,0,0,0,0,0)));
//    assert_eq!(Ipv6AddrPrefixed::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/8").and_then(|a|a.usable_networks_len()),
//               Some(72057594037927940));
    assert_eq!(Ipv6AddrPrefixed::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/8").map(|a|a.usable_address_len()),
               Some(1329227995784915872903807060280344576));
    assert_eq!(Ipv6AddrPrefixed::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/100").map(|a|a.usable_address_len()),
               Some(268435456));
//    assert_eq!(Ipv6AddrMasked::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/8").and_then(|a|a.broadcast_address()),
//               Some(Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab)));
//    assert_eq!(Ipv6AddrMasked::from_str("2001:0DB8:0000:0000:0000:0000:1428:57ab/8").and_then(|a|a.subnet_mask()),
//               Some(Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab)));
}