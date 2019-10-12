use ipip::{ipv4, Ipv4AddrMasked, UsableIpv4HostsIter};
use std::net::Ipv4Addr;

#[test]
fn test_macro() {
    assert_eq!(ipv4!(10.0.0.1/8),Ipv4AddrMasked {
        addr: Ipv4Addr::new(10,0,0,1),
        mask: 8
    });
    assert_eq!(ipv4!(10.0.0.1),Ipv4Addr::new(10,0,0,1));
}

#[test]
fn test_from_str() {
    assert_eq!(Ipv4AddrMasked::from_str("10.0.0.1/8"),Some(Ipv4AddrMasked {
        addr: Ipv4Addr::new(10,0,0,1),
        mask: 8
    }));
    assert_eq!(Ipv4AddrMasked::from_str("10.256.0.1/8"),None);
    assert_eq!(Ipv4AddrMasked::from_str("10.256.0.1/8"),None);
}

#[test]
fn test_subnet() {
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
}

#[test]
fn test_iter() {
    let iter:UsableIpv4HostsIter = ipv4!(10.0.0.1/24).usable_hosts();
    assert_eq!(iter.size_hint(),(0,Some(254)));
    let vec:Vec<Ipv4Addr> = iter.collect();
    assert_eq!(vec.len(),254);
    assert_eq!(vec.get(0),Some(&ipv4!(10.0.0.1)));
    assert_eq!(vec.get(253),Some(&ipv4!(10.0.0.254)));
}

#[test]
fn test_other() {
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/28").and_then(|a|a.broadcast_address()),
        Some(Ipv4Addr::new(10,255,0,15))
    );
    assert_eq!(
        Ipv4AddrMasked::from_str("10.255.0.1/28").and_then(|a|a.base_address()),
        Some(Ipv4Addr::new(10,255,0,0))
    );
    assert_eq!(ipv4!(10.0.0.1/24).usable_hosts_len(),254);
    assert_eq!(ipv4!(10.0.0.1/8).usable_hosts_len(),16777214);
}