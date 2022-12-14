mod implementations;

use core::char::ParseCharError;
use core::num::{ParseFloatError, ParseIntError};
use core::str::ParseBoolError;
use std::error;
use std::net::AddrParseError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to parse as an integer.")]
    Int(#[from] ParseIntError),
    #[error("Unable to parse as a boolean.")]
    Bool(#[from] ParseBoolError),
    #[error("Unable to parse as a character.")]
    Char(#[from] ParseCharError),
    #[error("Unable to parse as a float.")]
    Float(#[from] ParseFloatError),
    #[error("Unable to parse as an address.")]
    Addr(#[from] AddrParseError),
    #[error("Unable to parse into type.")]
    Dyn(#[from] Box<dyn error::Error>),
    #[error("Invalid literal match (expected to find {expected:?}, found {found:?}).")]
    Literal { expected: String, found: String },
}

pub trait LendingFromStr<'a> {
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl<'a> LendingFromStr<'a> for &'a str {
    fn from_str(s: &'a str) -> Result<Self, ParseError> {
        Ok(s)
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for str {}
}

pub trait ExtStr: private::Sealed {
    fn lending_parse<'a, F: LendingFromStr<'a>>(&'a self) -> Result<F, ParseError>;
}

impl ExtStr for str {
    fn lending_parse<'a, F: LendingFromStr<'a>>(&'a self) -> Result<F, ParseError> {
        LendingFromStr::from_str(self)
    }
}
