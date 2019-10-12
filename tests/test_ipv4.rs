use ipip::{ipv4, Ipv4AddrMasked};
use std::net::Ipv4Addr;

#[test]
fn test() {
    assert_eq!(ipv4!(10.0.0.1/8),Ipv4AddrMasked {
        addr: Ipv4Addr::new(10,0,0,1),
        mask: 8
    });
    assert_eq!(ipv4!(10.0.0.1),Ipv4Addr::new(10,0,0,1));
    assert_eq!(Ipv4AddrMasked::from_str("10.0.0.1/8"),Some(Ipv4AddrMasked {
        addr: Ipv4Addr::new(10,0,0,1),
        mask: 8
    }));
    assert_eq!(Ipv4AddrMasked::from_str("10.256.0.1/8"),None);
    assert_eq!(Ipv4AddrMasked::from_str("10.256.0.1/8"),None);
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/8").and_then(|a|a.subnet_mask()),
        Some(Ipv4Addr::new(255,0,0,0))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/10").and_then(|a|a.subnet_mask()),
        Some(Ipv4Addr::new(255,192,0,0))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/18").and_then(|a|a.subnet_mask()),
        Some(Ipv4Addr::new(255,255,192,0))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/28").and_then(|a|a.subnet_mask()),
        Some(Ipv4Addr::new(255,255,255,240))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/28").and_then(|a|a.broadcast_address()),
        Some(Ipv4Addr::new(10,255,0,15))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/28").and_then(|a|a.base_address()),
        Some(Ipv4Addr::new(10,255,0,0))
    );
}