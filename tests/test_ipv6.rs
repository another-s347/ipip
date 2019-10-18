use ipip::{ipv6, Ipv6AddrMasked};
use std::net::Ipv6Addr;

#[test]
fn test_macro() {
    assert_eq!(ipv6!(2001:0DB8:0000:0000:0000:0000:1428:57ab/8),Ipv6AddrMasked {
        addr: Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab),
        mask: 8
    });
    assert_eq!(ipv6!(2001:0DB8:0000:0000:0000:0000:1428:57ab),Ipv6Addr::new(0x2001,0x0DB8,0,0,0,0,0x1428,0x57ab));
}