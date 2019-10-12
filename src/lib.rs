use proc_macro_hack::proc_macro_hack;
use std::net::Ipv4Addr;

#[proc_macro_hack(support_nested)]
pub use ipip_macro_impl::ipv4;
#[proc_macro_hack(support_nested)]
pub use ipip_macro_impl::mac;
use std::fmt::{Display, Formatter, Error, Debug};
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct MAC(pub [u8; 6]);

impl MAC {
    pub fn from_str(s:&str) -> Option<MAC> {
        let s = &s[0..17];
        let split:Vec<&str> = s.split(":").collect();
        if split.len() != 6 {
            return None;
        }
        if split[0].len()!=2 {
            return None;
        }
        let a = hex_to_u8(split[0])?;
        if split[1].len()!=2 {
            return None;
        }
        let b = hex_to_u8(split[1])?;
        if split[2].len()!=2 {
            return None;
        }
        let c = hex_to_u8(split[2])?;
        if split[3].len()!=2 {
            return None;
        }
        let d = hex_to_u8(split[3])?;
        if split[4].len()!=2 {
            return None;
        }
        let e = hex_to_u8(split[4])?;
        if split[5].len()!=2 {
            return None;
        }
        let f = hex_to_u8(split[5])?;
        Some(MAC([a,b,c,d,e,f]))
    }

    pub fn broadcast() -> MAC {
        MAC([0xff; 6])
    }

    pub fn zero() -> MAC {
        MAC([0x00; 6])
    }

    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xff; 6]
    }

    pub fn is_multicast(&self) -> bool {
        self.0[0] == 0x33 && self.0[1] == 0x33
    }

    pub fn as_ref(&self) -> &[u8; 6] {
        &self.0
    }

    pub fn from_slice(b: &[u8]) -> MAC {
        let mut s = [0u8; 6];
        s.copy_from_slice(b.as_ref());
        MAC(s)
    }

    pub fn display_slice(b: &[u8; 6], f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5]
        )
    }
}

impl AsRef<[u8]> for MAC {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for MAC {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Ipv4AddrMasked {
    pub addr:Ipv4Addr,
    pub mask:u8
}

impl Ipv4AddrMasked {
    pub fn from_str(s:&str) -> Option<Self> {
        let split:Vec<&str> = s.split("/").collect();
        if split.len()!=2 {
            return None;
        }
        let addr = Ipv4Addr::from_str(split[0]).ok()?;
        let mask = split[1].parse().ok()?;
        Some(Self {
            addr,
            mask
        })
    }

    pub fn subnet_mask(&self) -> Option<Ipv4Addr> {
        if 1<=self.mask&&self.mask<=32 {
            let x = !(0xffffffffu32 >> (self.mask as u32));
            let a = ((x >> 24) & 0b11111111) as u8;
            let b = ((x >> 16) & 0b11111111) as u8;
            let c = ((x >> 8) & 0b11111111) as u8;
            let d = (x & 0b11111111) as u8;
            Some(Ipv4Addr::new(a,b,c,d))
        }
        else {
            None
        }
    }
}

impl Debug for Ipv4AddrMasked {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,"{}/{}",self.addr,self.mask)
    }
}

fn hex_to_u8(s:&str) -> Option<u8> {
    let r:&[u8] = s.as_ref();
    let low:u8 = r[0];
    let high:u8 = r[1];
    let low = if in_ascii_hex_range(low) {
        low-87
    }
    else if in_ascii_num_range(low) {
        low-48
    }
    else {
        return None;
    };
    let high = if in_ascii_hex_range(high) {
        high-87
    }
    else if in_ascii_num_range(high) {
        high-48
    }
    else {
        return None;
    };
    Some(low*16 + high)
}

fn in_ascii_num_range(a:u8)->bool {
    48<=a && a<=57
}

fn in_ascii_hex_range(a:u8)->bool {
    97<=a && a<=102
}