#![recursion_limit = "128"]
//#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::ext::IdentExt;
use syn::{LitFloat, LitInt, parse_macro_input, Token, Error, Ident,Lit};
use syn::parse::{Parse, ParseStream, Result, ParseBuffer};

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::token::Token;

struct IPv4 {
    a:u8,
    b:u8,
    c:u8,
    d:u8,
    mask:Option<u8>
}

impl Parse for IPv4 {
    fn parse(input: ParseStream) -> Result<Self> {
        let high:LitFloat = input.parse()?;
        input.parse::<Token![.]>()?;
        let low:LitFloat = input.parse()?;
        let x:f32 = high.base10_parse().unwrap();
        let a = x.floor();
        if a>255.0 {
            return Err(input.error("a > 255"));
        }
        let b = (x.fract()*1000.0).round();
        if b>255.0 {
            return Err(input.error("b > 255"));
        }
        let a = a as u8;
        let mut b = b as u8;
        if b%10 == 0 {
            b = b/10;
        }
        if b%10 == 0 {
            b = b/10;
        }
        let x:f32 = low.base10_parse()?;
        let c = x.floor();
        if c>255.0 {
            return Err(input.error("c > 255"));
        }
        let d = (x.fract()*1000.0).round();
        if d>255.0 {
            return Err(input.error("d > 255"));
        }
        let c = c as u8;
        let mut d = d as u8;
        if d%10 == 0 {
            d = d/10;
        }
        if d%10 == 0 {
            d = d/10;
        }
        let mask = if let Ok(_) = input.parse::<Token![/]>() {
            Some(input.parse::<LitInt>()?.base10_parse()?)
        }
        else {
            None
        };
        Ok(IPv4 {
            a,
            b,
            c,
            d,
            mask
        })
    }
}

struct MAC {
    a:u8,
    b:u8,
    c:u8,
    d:u8,
    e:u8,
    f:u8
}

struct HexLitInt<T>(pub T);

impl<T> Parse for HexLitInt<T>
    where T:MyNumBound+Default
{
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let mut result:T = T::default();
        let mut width = T::width();
        loop {
            if input.peek(LitInt) {
                let lit = input.parse::<LitInt>()?;
                let d = lit.to_string();
                let is:&[u8] = d.as_ref();
                if width<is.len() {
                    return Err(input.error(format!("Hex length exceeded width == {}",T::width())));
                }
                else {
                    width-=is.len();
                }
                for i in is {
                    if let Some(i) = hex_to_u8(*i) {
                        result = result.add_hex(i);
                    }
                    else {
                        break;
                    }
                }
            }
            else if input.peek(Ident::peek_any) {
                let ident = input.parse::<Ident>()?;
                let ident_str = ident.to_string();
                let is:&[u8] = ident_str.as_ref();
                if width<is.len() {
                    return Err(input.error(format!("Hex length exceeded width == {}",T::width())));
                }
                else {
                    width-=is.len();
                }
                for i in is {
                    if let Some(i) = hex_to_u8(*i) {
                        result = result.add_hex(i);
                    }
                    else {
                        break;
                    }
                }
            } else {
                return Ok(HexLitInt(result))
            };
        }
    }
}

trait MyNumBound {
    fn width() -> usize;

    fn add_hex(&self,other:u8) -> Self;
}

impl MyNumBound for u8 {
    fn width() -> usize {
        2
    }

    fn add_hex(&self, other: u8) -> Self {
        self*16+other
    }
}

impl MyNumBound for u16 {
    fn width() -> usize {
        4
    }

    fn add_hex(&self, other: u8) -> Self {
        self*16+(other as u16)
    }
}

impl Parse for MAC {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let a:u8 = input.parse::<HexLitInt<u8>>()?.0;
        input.parse::<Token![:]>()?;
        let b:u8 = input.parse::<HexLitInt<u8>>()?.0;
        input.parse::<Token![:]>()?;
        let c:u8 = input.parse::<HexLitInt<u8>>()?.0;
        input.parse::<Token![:]>()?;
        let d:u8 = input.parse::<HexLitInt<u8>>()?.0;
        input.parse::<Token![:]>()?;
        let e:u8 = input.parse::<HexLitInt<u8>>()?.0;
        input.parse::<Token![:]>()?;
        let f:u8 = input.parse::<HexLitInt<u8>>()?.0;
        Ok(MAC {
            a,
            b,
            c,
            d,
            e,
            f
        })
    }
}

struct IPv6 {
    a:u16,
    b:u16,
    c:u16,
    d:u16,
    e:u16,
    f:u16,
    mask:Option<u8>
}

impl Parse for IPv6 {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let a:u16 = input.parse::<HexLitInt<u16>>()?.0;
        input.parse::<Token![:]>()?;
        let b:u16 = input.parse::<HexLitInt<u16>>()?.0;
        input.parse::<Token![:]>()?;
        let c:u16 = input.parse::<HexLitInt<u16>>()?.0;
        input.parse::<Token![:]>()?;
        let d:u16 = input.parse::<HexLitInt<u16>>()?.0;
        input.parse::<Token![:]>()?;
        let e:u16 = input.parse::<HexLitInt<u16>>()?.0;
        input.parse::<Token![:]>()?;
        let f:u16 = input.parse::<HexLitInt<u16>>()?.0;
        let mask = if let Ok(_) = input.parse::<Token![/]>() {
            Some(input.parse::<LitInt>()?.base10_parse()?)
        }
        else {
            None
        };
        Ok(IPv6 {
            a,
            b,
            c,
            d,
            e,
            f,
            mask
        })
    }
}

#[proc_macro_hack]
pub fn ipv4(input:TokenStream) -> TokenStream {
    let IPv4 {
        a, b, c, d, mask
    } = parse_macro_input!(input as IPv4);

    if let Some(mask) = mask {
        TokenStream::from(quote! {
            ::ipip::Ipv4AddrMasked {
                addr:std::net::Ipv4Addr::new(#a,#b,#c,#d),
                mask:#mask
            }
        })
    }
    else {
        TokenStream::from(quote! {
            std::net::Ipv4Addr::new(#a,#b,#c,#d)
        })
    }
}

#[proc_macro_hack]
pub fn mac(input:TokenStream) -> TokenStream {
    let MAC {
        a, b, c, d, e, f
    } = parse_macro_input!(input as MAC);

    TokenStream::from(quote! {
        ::ipip::MAC([#a,#b,#c,#d,#e,#f])
    })
}

#[proc_macro_hack]
pub fn ipv6(input:TokenStream) -> TokenStream {
    let IPv6 {
        a, b, c, d, e, f, mask
    } = parse_macro_input!(input as IPv6);

    if let Some(mask) = mask {
        TokenStream::from(quote! {
            ::ipip::Ipv6AddrMasked {
                addr:std::net::Ipv6Addr::new(#a,#b,#c,#d,#e,#f),
                mask:#mask
            }
        })
    }
    else {
        TokenStream::from(quote! {
            std::net::Ipv6Addr::new(#a,#b,#c,#d,#e,#f)
        })
    }
}

fn hex_to_u8(s:u8) -> Option<u8> {
    if in_ascii_hex_range(s) {
        Some(s-87)
    }
    else if in_ascii_num_range(s) {
        Some(s-48)
    }
    else {
        None
    }
}

fn in_ascii_num_range(a:u8)->bool {
    48<=a && a<=57
}

fn in_ascii_hex_range(a:u8)->bool {
    97<=a && a<=102
}