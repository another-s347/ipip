ipip
==========================================================
A small extension to network address representation
It has:
 - Masked ip address
 - MAC address
 - compile-time checked macro

### Example
```rust
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

assert_eq!(mac!(01:23:45:67:89:af),MAC([0x01,0x23,0x45,0x67,0x89,0xaf]));
assert_eq!(MAC::from_str("01:23:45:67:89:af"),Some(MAC([0x01,0x23,0x45,0x67,0x89,0xaf])));
```
#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>