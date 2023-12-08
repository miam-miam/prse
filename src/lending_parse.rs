use core::num::*;
use core::str::FromStr;

use crate::parse_error::ParseError;

/// Parse a string into the implemented type, unlike [`FromStr`] this trait allows
/// you to borrow the string. It can be automatically derived using
/// [`Parse`](prse_derive::Parse).
pub trait Parse<'a> {
    /// Parses a string `s` to a return value of this type.
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return a [`ParseError`].
    ///
    /// ```
    /// # use prse::{parse, Parse, ParseError};
    /// # #[derive(PartialEq, Debug)]
    /// struct Count<'b>(&'b str, u32);
    ///
    /// impl<'a> Parse<'a> for Count<'a> {
    ///     fn from_str(s: &'a str) -> Result<Self, ParseError> {
    ///         let (fruit, count) = s.split_once(':').ok_or_else(|| ParseError::new("expected a colon."))?;    
    ///         Ok(Count(<&'a str>::from_str(fruit)?, <u32>::from_str(count.trim())?))
    ///     }
    /// }
    ///
    /// let c: Count = parse!("I have: {apple: 8}.", "I have: {{{}}}.");
    /// assert_eq!(c, Count("apple", 8));
    /// ```
    ///
    /// It also allows you to add your own custom parsing function.
    ///
    /// ```
    /// # use prse::{parse, Parse, ParseError};
    /// # #[derive(PartialEq, Debug)]
    /// struct Hex(i32);
    ///
    /// impl<'a> Parse<'a> for Hex {
    ///     fn from_str(s: &'a str) -> Result<Self, ParseError> {
    ///         Ok(Hex(i32::from_str_radix(s, 16)?)) // Using 16 for Hexadecimal numbers
    ///     }
    /// }
    ///
    /// let v: Hex = parse!("A", "{}");
    /// assert_eq!(v, Hex(10));
    /// ```
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl<'a> Parse<'a> for &'a str {
    fn from_str(s: &'a str) -> Result<Self, ParseError> {
        Ok(s)
    }
}

macro_rules! impl_parse {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> Parse<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    <Self as FromStr>::from_str(s.trim()).map_err(|e| e.into())
                }
            }
        )+
    };
}

#[cfg(feature = "alloc")]
macro_rules! impl_parse_infallible {
    ( $( $Ty: ty )+) => {
        $(
            impl<'a> Parse<'a> for $Ty {
                fn from_str(s: &'a str) -> Result<Self, ParseError> {
                    Ok(<Self as FromStr>::from_str(&s).unwrap())
                }
            }
        )+
    };
}

impl_parse!(isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128);
impl_parse!(bool char f32 f64);
impl_parse!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);
impl_parse!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);

#[cfg(feature = "alloc")]
mod impl_alloc {
    extern crate alloc;

    use alloc::string::String;

    use super::{FromStr, Parse, ParseError};

    impl_parse_infallible!(String);
}

#[cfg(feature = "std")]
mod impl_std {
    use std::ffi::OsString;
    use std::net::*;
    use std::path::PathBuf;

    use super::{FromStr, Parse, ParseError};

    impl_parse_infallible!(OsString PathBuf);
    impl_parse!(IpAddr SocketAddr Ipv4Addr Ipv6Addr SocketAddrV4 SocketAddrV6);
}

/// An str extension trait to allow you to call the `from_str` from [`Parse`]
/// without specifying the type.
///
/// The trait is sealed and cannot be implemented on any other type.
pub trait ExtParseStr: __private::Sealed {
    /// Parses the string slice into another type.
    ///
    /// lending_parse can parse into any type that implements the [`Parse`] trait.
    fn lending_parse<'a, F: Parse<'a>>(&'a self) -> Result<F, ParseError>;
}

impl ExtParseStr for str {
    fn lending_parse<'a, F: Parse<'a>>(&'a self) -> Result<F, ParseError> {
        Parse::from_str(self)
    }
}

#[doc(hidden)]
mod __private {
    pub trait Sealed {}

    impl Sealed for str {}
}
