use core::char::ParseCharError;
use core::num::{ParseFloatError, ParseIntError};
use core::str::ParseBoolError;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "std")]
use std::borrow::Borrow;
#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::net::AddrParseError;

#[derive(Debug)]
pub enum ParseError {
    Int(ParseIntError),
    Bool(ParseBoolError),
    Char(ParseCharError),
    Float(ParseFloatError),
    #[cfg(feature = "std")]
    Addr(AddrParseError),
    #[cfg(feature = "std")]
    Dyn(Box<dyn error::Error>),
    #[cfg(not(feature = "std"))]
    Dyn,
    #[cfg(feature = "alloc")]
    Literal {
        expected: String,
        found: String,
    },
    #[cfg(not(feature = "alloc"))]
    Literal,
    Multi {
        expected: u8,
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
            ParseError::Dyn(source) => Some(source.borrow()),
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
impl_from_parse_error!(Box<dyn error::Error>, Dyn);
