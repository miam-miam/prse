use core::num::*;
use core::str::FromStr;
use std::ffi::OsString;
use std::net::*;
use std::path::PathBuf;

use super::{LendingFromStr, ParseError};

macro_rules! impl_lending_from_str {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> LendingFromStr<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    <Self as FromStr>::from_str(&s).map_err(|e| e.into())
                }
            }
        )+
    };
}

macro_rules! impl_lending_from_str_infallible {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> LendingFromStr<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    Ok(<Self as FromStr>::from_str(&s).unwrap())
                }
            }
        )+
    };
}

impl_lending_from_str!(isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128);
impl_lending_from_str!(bool char f32 f64);
impl_lending_from_str!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);
impl_lending_from_str!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);
impl_lending_from_str!(IpAddr SocketAddr Ipv4Addr Ipv6Addr SocketAddrV4 SocketAddrV6);
impl_lending_from_str_infallible!(String OsString PathBuf);
