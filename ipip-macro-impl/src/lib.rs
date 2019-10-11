#![recursion_limit = "128"]
//#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::ext::IdentExt;
use syn::{LitFloat, LitInt, parse_macro_input, Token, Error, Ident};
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

struct HexLitInt(pub u8);

impl Parse for HexLitInt {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let d:u8 = if input.peek(LitInt) {
            let d:u8 = input.parse::<LitInt>()?.base10_parse()?;
            let low = d%10;
            let high = d/10;
            high*16+low
        }
        else if input.peek(Ident::peek_any) {
            let ident = input.parse::<Ident>()?;
            let ident_str = ident.to_string();
            match ident_str.len() {
                0 => {
                    unreachable!()
                }
                1 => {
                    let r:&[u8] = ident_str.as_ref();
                    let low:u8 = r[0];
                    if low < 97 || low > 102 {
                        return Err(input.error("Invalid hex"))
                    }
                    low - 87
                }
                2 => {
                    let r:&[u8] = ident_str.as_ref();
                    let low:u8 = r[0];
                    let high:u8 = r[1];
                    if low < 97 || low > 102 {
                        return Err(input.error("Invalid hex"))
                    }
                    if high < 97 || high > 102 {
                        return Err(input.error("Invalid hex"))
                    }
                    (low - 87)*16 + (high-87)
                }
                _ => {
                    return Err(input.error("Hex length > 2"))
                }
            }
        } else {
            unreachable!()
        };
        Ok(HexLitInt(d))
    }
}

impl Parse for MAC {
    fn parse(input: &ParseBuffer) -> Result<Self> {
        let a:u8 = input.parse::<HexLitInt>()?.0;
        input.parse::<Token![:]>()?;
        let b:u8 = input.parse::<HexLitInt>()?.0;
        input.parse::<Token![:]>()?;
        let c:u8 = input.parse::<HexLitInt>()?.0;
        input.parse::<Token![:]>()?;
        let d:u8 = input.parse::<HexLitInt>()?.0;
        input.parse::<Token![:]>()?;
        let e:u8 = input.parse::<HexLitInt>()?.0;
        input.parse::<Token![:]>()?;
        let f:u8 = input.parse::<HexLitInt>()?.0;
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