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
}