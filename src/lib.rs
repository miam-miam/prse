#![cfg_attr(not(feature = "std"), no_std)]
// #![warn(missing_docs)]

pub use prse_derive::{parse, try_parse};

pub use crate::lending_parse::{ExtParseStr, LendingFromStr};
pub use crate::parse_error::ParseError;

mod lending_parse;
mod parse_error;
