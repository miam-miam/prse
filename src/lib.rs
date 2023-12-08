#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! [![github]](https://github.com/miam-miam100/prse)&ensp;[![crates-io]](https://crates.io/crates/prse)&ensp;[![docs-rs]](https://docs.rs/prse)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides the [`parse!`] macro to easily parse strings using a format args like syntax.
//!
//! Prse is a no-std compatible small string parsing library with an emphasis on speed and ease of use.
//!
//! The [`parse!`] and [`try_parse!`] macros can parse anything in the standard library
//! that implements [`FromStr`](std::str::FromStr), or any type that implements [`Parse`].
//! (Which can be derived with the [`Parse`](derive.Parse.html) macro)
//!
//! # Examples
//! ```
//!# use prse::parse;
//!#
//! let input = "Person 5: Hello Bob!";
//! let name: &str;
//!
//! // Calls try_parse and unwraps the result.
//! let five: u32 = parse!(input, "Person {}: Hello {name}!");
//! // We can also specify the return position
//! assert_eq!((name, five), parse!(input, "Person {1}: Hello {0}!"));
//!
//! assert_eq!(five, 5);
//! assert_eq!(name, "Bob");
//! ```
//!
//! ```
//!# use prse::try_parse;
//!#
//! let input = "I hate apples!\nI love apricots!";
//!
//! for fruit in input.lines().map(|l| try_parse!(l, "I love {}!")) {
//!     match fruit {
//!         Ok::<&str, _>(fruit) => assert_eq!(fruit, "apricots"),
//!         Err(e) => println!("{e:?}")
//!     }
//! }
//! ```
//!
//! Additionally you can use the [`Parse`](prse_derive::Parse) derive macro to help you parse
//! custom types. For even more flexibility you can implement the [`Parse`]
//! trait yourself for fully custom parsing such as hexadecimal number parsing.
//!
//! ```rust
//! use prse::{parse, Parse};
//!
//! #[derive(Parse, PartialEq, Eq, Debug)]
//! #[prse = "({x}, {y})"]
//! struct Position {
//!     x: i32,
//!     y: i32,
//! }
//!
//! let input = "(1, 3) + (-2, 9)";
//!
//! let (lhs, rhs): (Position, Position) = parse!(input, "{} + {}");
//!
//! assert_eq!(lhs, Position {x: 1, y: 3});
//! assert_eq!(rhs, Position {x: -2, y: 9});
//! ```
//!
//! # Repetition
//!
//! You can parse multiple parts of a string using one of the following methods:
//!
//! ## Array
//!
//! You can parse a string into an array of parsed elements using the following syntax `{<var>:<sep>:<count>}`.
//!
//! ```
//!# use prse::parse;
//!#
//! let input = "My farm contains exactly 3 animals: Beatrice, Betsy, Callum";
//!
//! // var = nothing, sep = ", " and count = 3
//! let array: [&str; 3] = parse!(input, "My farm contains exactly 3 animals: {:, :3}");
//!
//! assert_eq!(array, ["Beatrice", "Betsy", "Callum"]);
//! ```
//! ## Vec
//!
//! You can parse a string into a Vec of parsed elements using the following syntax `{<var>:<sep>:}`.
//! This way of parsing is only available if the alloc feature has been enabled.
//!
//! ```
//!# use prse::parse;
//!#
//! let input = "My farm contains some amount of booleans: true || false || true || false";
//! let many: Vec<bool>;
//!
//! // var = many and sep = " || "
//! parse!(input, "My farm contains some amount of booleans: {many: || :}");
//!
//! assert_eq!(many, vec![true, false, true, false]);
//! ```
//! ## Iterator
//!
//! Alternatively if you are unable to allocate anything then you can use a lazy iterator
//! by using the following syntax `{<var>:<sep>:0}`.
//!
//! One important note is that since the iterator is evaluated lazily it will always return an iterator of [`Results`](Result).
//!
//! The returned iterator will either be [`ParseIter`](struct.ParseIter.html) or [`ParseChars`](struct.ParseChars.html) if the separator is empty.  
//!
//! ```
//!# use prse::parse;
//!#
//! let input = "My farm has this many animals: [5,23,42,1,3,5]";
//!
//! // var = nothing and sep = ","
//! let animal_count: u32 = parse!(input, "My farm has this many animals: [{:,:0}]")
//!     .flat_map(|c: Result<u32, _>| c.ok())
//!     .sum();
//!
//! assert_eq!(animal_count, 79);
//! ```
//!
//! ## Modifiers
//!
//! All three multi-parsers (Array, Vec and Iterator) allow `{<var>:<sep>:!<kind>}` syntax to skip multiple separators, for example
//! ```
//! # use prse::parse;
//! #
//! assert_eq!([1, 2, 3], parse!("1-2---3", "{:-:!3}"));
//! ```
//!
//! ## Empty separators
//!
//! If the separator is an empty string slice (e.g. `{::}`) then the multi-parsers will iterate over every [char] in the string.
//! ```
//! # use prse::parse;
//! #
//! assert_eq!([3, 2, 1], parse!("321", "{::3}"))
//! ```
//!

pub use prse_derive::{parse, try_parse, Parse};

#[rustfmt::skip]
pub use crate::lending_parse::{ExtParseStr, Parse};
pub use crate::parse_error::ParseError;
#[doc(hidden)]
pub use crate::parse_error::__private;
pub use crate::parse_iterators::{ParseChars, ParseIter};

mod lending_parse;
mod parse_error;
mod parse_iterators;
