use core::char::ParseCharError;
use core::num::{ParseFloatError, ParseIntError};
use core::str::ParseBoolError;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::net::AddrParseError;

/// The error returned when trying to parse a type using [`try_parse`](crate::try_parse) or [`LendingFromStr`](crate::LendingFromStr).
#[derive(Debug)]
pub enum ParseError {
    /// The variant returned when an integer cannot be parsed.
    Int(ParseIntError),
    /// The variant returned when a bool cannot be parsed.
    Bool(ParseBoolError),
    /// The variant returned when a char cannot be parsed.
    Char(ParseCharError),
    /// The variant returned when a float cannot be parsed.
    Float(ParseFloatError),
    #[cfg(feature = "std")]
    /// The variant returned when an ip address cannot be parsed.
    /// This variant is only enabled with the `std` feature.
    Addr(AddrParseError),
    #[cfg(feature = "std")]
    /// The variant returned when you want to return an error that is not defined here.
    /// When not using the `std` feature, `Dyn` is a unit variant as the
    /// [`Error`](error::Error) trait is part of std.
    Dyn(Box<dyn error::Error + Send + Sync>),
    /// The variant returned when you want to return an error that is not defined here.
    /// When not using the `std` feature, `Dyn` is a unit variant as the
    /// [`Error`](error::Error) trait is part of std.
    #[cfg(not(feature = "std"))]
    Dyn,
    /// The variant returned when [`parse!`](crate::parse) found an unexpected literal.
    /// When not using the `alloc` feature, `Literal` is a unit variant.
    #[cfg(feature = "alloc")]
    Literal {
        /// What it expected.
        expected: String,
        /// What it actually found.
        found: String,
    },
    /// The variant returned when [`parse!`](crate::parse) found an unexpected literal.
    /// When not using the `alloc` feature, `Literal` is a unit variant.
    #[cfg(not(feature = "alloc"))]
    Literal,
    /// The variant returned when parsing an array and finding more or less elements than what was expected.
    Multi {
        /// The size of the array it was expecting.
        expected: u8,
        /// The size of the array it found.
        found: u8,
    },
}

#[cfg(feature = "std")]
impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseError::Int(source) => Some(source),
            ParseError::Bool(source) => Some(source),
            ParseError::Char(source) => Some(source),
            ParseError::Float(source) => Some(source),
            ParseError::Addr(source) => Some(source),
            ParseError::Dyn(source) => Some(&**source),
            ParseError::Literal { .. } => None,
            ParseError::Multi { .. } => None,
        }
    }
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ParseError::Int(_) => write!(fmt, "unable to parse as an integer"),
            ParseError::Bool(_) => write!(fmt, "unable to parse as a boolean"),
            ParseError::Char(_) => write!(fmt, "unable to parse as a character"),
            ParseError::Float(_) => write!(fmt, "unable to parse as a float"),
            #[cfg(feature = "std")]
            ParseError::Addr(_) => write!(fmt, "unable to parse as an address"),
            #[cfg(feature = "std")]
            ParseError::Dyn(_) => write!(fmt, "unable to parse into type"),
            #[cfg(not(feature = "std"))]
            ParseError::Dyn => write!(fmt, "unable to parse into type"),
            #[cfg(feature = "alloc")]
            ParseError::Literal { expected, found } => write!(
                fmt,
                "invalid literal match (expected to find {expected:?}, found {found:?})"
            ),
            #[cfg(not(feature = "alloc"))]
            ParseError::Literal => write!(fmt, "invalid literal match"),
            ParseError::Multi { expected, found } => write!(
                fmt,
                "invalid number of items (expected to find {expected:?}, found {found:?})"
            ),
        }
    }
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        use ParseError::*;

        match (self, other) {
            (Int(x), Int(y)) if x == y => true,
            (Bool(x), Bool(y)) if x == y => true,
            (Char(x), Char(y)) if x == y => true,
            (Float(x), Float(y)) if x == y => true,
            #[cfg(feature = "std")]
            (Addr(x), Addr(y)) if x == y => true,
            #[cfg(not(feature = "std"))]
            (Dyn, Dyn) => true,
            #[cfg(feature = "alloc")]
            (
                Literal {
                    expected: lx,
                    found: ly,
                },
                Literal {
                    expected: rx,
                    found: ry,
                },
            ) if lx == rx && ly == ry => true,
            #[cfg(not(feature = "alloc"))]
            (Literal, Literal) => true,
            (
                Multi {
                    expected: lx,
                    found: ly,
                },
                Multi {
                    expected: rx,
                    found: ry,
                },
            ) if lx == rx && ly == ry => true,
            _ => false,
        }
    }
}

macro_rules! impl_from_parse_error {
    ($Ty: ty, $Id: ident) => {
        impl From<$Ty> for ParseError {
            fn from(source: $Ty) -> Self {
                ParseError::$Id(source)
            }
        }
    };
}

impl_from_parse_error!(ParseIntError, Int);
impl_from_parse_error!(ParseBoolError, Bool);
impl_from_parse_error!(ParseCharError, Char);
impl_from_parse_error!(ParseFloatError, Float);
#[cfg(feature = "std")]
impl_from_parse_error!(AddrParseError, Addr);
#[cfg(feature = "std")]
impl_from_parse_error!(Box<dyn error::Error + Send + Sync>, Dyn);

#[cfg(test)]
mod test {
    use crate::ParseError;

    #[test]
    fn check_impl_traits() {
        fn is_send<T: Send>() {}
        fn is_sync<T: Sync>() {}

        is_send::<ParseError>();
        is_sync::<ParseError>();
    }
}
