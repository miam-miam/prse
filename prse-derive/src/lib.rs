#![warn(missing_docs)]
//! A helper crate for prse, the small string parsing library.

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use derive::Derive;
use invocation::ParseInvocation;
use proc_macro::TokenStream;
use quote::ToTokens;

mod derive;
mod expand_derive;
mod instructions;
mod invocation;
mod var;

/// The `parse` macro allows you to parse a string into any type that implements [`Parse`](trait.Parse.html).
/// (Which can be derived with the [`Parse`](derive.Parse.html) macro)
///
/// ```ignore
/// let input = "5 + -2 = 3";
///
/// let total: i32;
/// let (lhs, rhs): (i32, i32) = parse!(input, "{} + {} = {total}");
///
///
/// assert_eq!(lhs + rhs, total);
///
/// // You can also specify the variable's position in the tuple.
///
/// let (rhs, lhs): (u32, u32) = parse!("10 / 2", "{1} / {0}");
///
/// assert_eq!(lhs / rhs, 5);
/// ```
///
/// # Repetition
///
/// You can parse multiple parts of a string using one of the following methods:
///
/// ## Array
///
/// You can parse a string into an array of parsed elements using the following syntax `{<var>:<sep>:<count>}`.
///
/// ```ignore
/// let input = "My farm contains exactly 3 animals: Beatrice, Betsy, Callum";
///
/// // var = nothing, sep = ", " and count = 3
/// let array: [&str; 3] = parse!(input, "My farm contains exactly 3 animals: {:, :3}");
///
/// assert_eq!(array, ["Beatrice", "Betsy", "Callum"]);
/// ```
/// ## Vec
///
/// You can parse a string into a Vec of parsed elements using the following syntax `{<var>:<sep>:}`.
/// This way of parsing is only available if the alloc feature has been enabled.
///
/// ```ignore
/// let input = "My farm contains some amount of booleans: true || false || true || false";
/// let many: Vec<bool>;
///
/// // var = many and sep = " || "
/// parse!(input, "My farm contains some amount of booleans: {many: || :}");
///
/// assert_eq!(many, vec![true, false, true, false]);
/// ```
/// ## Iterator
///
/// Alternatively if you are unable to allocate anything then you can use a lazy iterator
/// by using the following syntax `{<var>:<sep>:0}`.
///
/// One important note is that since the iterator is evaluated lazily it will always return an iterator of [`Results`](Result).
///
/// The returned iterator will either be [`ParseIter`](struct.ParseIter.html) or [`ParseChars`](struct.ParseChars.html) if the separator is empty.  
///
///
/// ```ignore
/// let input = "My farm has this many animals: [5,23,42,1,3,5]";
///
/// // var = nothing and sep = ","
/// let animal_count: u32 = parse!(input, "My farm has this many animals: [{:,:0}]")
///     .flat_map(|c: Result<u32, _>| c.ok())
///     .sum();
///
/// assert_eq!(animal_count, 79);
/// ```
///
///
/// ## Modifiers
///
/// All three multi-parsers (Array, Vec and Iterator) allow `{<var>:<sep>:!<kind>}` syntax to skip multiple separators, for example
/// ```ignore
/// assert_eq!([1, 2, 3], parse!("1-2---3", "{:-:!3}"));
/// ```
/// ## Empty separators
///
/// If the separator is an empty string slice (e.g. `{::}`) then the multi-parsers will iterate over every [char].
/// ```ignore
/// assert_eq!([3, 2, 1], parse!("321", "{::3}"))
/// ```
/// # Syntax
///
/// The [`parse!`] macro uses a literal with `{}` brackets to denote where it should
/// try to parse into a type. The macro must fully consume the string as it will otherwise
/// return an error. If this is not what you want you can create a dummy capture variable
/// that captures the rest of the string.
///
/// ```ignore
/// let input = "Hello world! Today is a great day!";
/// let world: &str;
///
/// // We only care about world so capture everything else as a string to prevent an error.
/// let _capture: &str = parse!(input, "Hello {world}!{}")
///
/// assert_eq!(world, "world");
/// ```
///
///
/// ## Escaping
///
/// The `{` and `}` brackets can be escaped by doubling them up. This leads to the following syntax:
///
/// ```ignore
/// let input = "Stuff in {} is really important: {42}";
/// let num: u128;
///
/// parse!(input, "Stuff in {{}} is really important: {{{num}}}");
///
/// assert_eq!(num, 42);
/// ```
#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ParseInvocation);
    input.to_token_stream().into()
}

/// Returns a [`Result`](https://doc.rust-lang.org/stable/std/result/enum.Result.html) instead of unwrapping like [`parse!`]. The [`Result`](https://doc.rust-lang.org/stable/std/result/enum.Result.html) has
/// [`ParseError`](enum.ParseError.html) as the error type.
///
/// For more information please look at [`parse!`].
/// ```ignore
/// let input = "cd C:\\windows\\system32";
/// let path: Result<PathBuf, _> = try_parse!(input, "cd {}");
///
/// assert_eq!(path.unwrap(), PathBuf::from("C:\\windows\\system32"));
/// ```
#[proc_macro]
pub fn try_parse(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ParseInvocation);
    input.try_parse = true;
    input.to_token_stream().into()
}

/// Automatically implements the [`Parse`](trait.Parse.html) trait using one of two methods.
///
/// You can define how each field should be parsed using the `prse` attribute.
/// The prse attribute uses the same syntax as [`parse!`] and even supports Vec, Iterator and Array repetition.
///
///```ignore
/// use prse::{parse, Parse};
///
/// #[derive(Debug, Parse)]
/// #[prse = "({x}, {y})"]
/// struct Position {
///     x: i32,
///     y: i32,
/// }
///
/// let pos: Position = parse!("This is a position: (1, 2)", "This is a position: {}");
/// assert_eq!(pos.x, 1);
/// assert_eq!(pos.y, 2);
///```
///
/// This can also be done on enums.
///
///```ignore
/// use prse::{parse, Parse};
///
/// #[derive(Debug, Parse, Eq, PartialEq)]
/// enum Position {
///     #[prse = "({x}, {y})"]
///     Pos { x: i32, y: i32 },
///     #[prse = "({})"]
///     SinglePos(i32),
///     #[prse = "()"]
///     #[prse = ""]
///     NoPos,
/// }
///
/// // the first prse attribute to match is used.
/// let pos0: Position = parse!("This is a position: (1, 2)", "This is a position: {}");
/// let pos1: Position = parse!("This is a position: (3)", "This is a position: {}");
/// let pos2: Position = parse!("This is a position: ()", "This is a position: {}");
/// let pos3: Position = parse!("This is a position: ", "This is a position: {}");
///
/// assert_eq!(pos0, Position::Pos { x: 1, y: 2 });
/// assert_eq!(pos1, Position::SinglePos(3));
/// assert_eq!(pos2, Position::NoPos);
/// assert_eq!(pos3, Position::NoPos);
///```
/// If no prse attributes are found, it will use your [`FromStr`](core::str::FromStr) implementation.
/// ```ignore
/// use prse::{parse, Parse};
///
/// #[derive(Debug, Parse)]
/// struct Position {
///     x: i32,
///     y: i32,
/// }
///
/// impl std::str::FromStr for Position {
///     type Err = ();
///
///     fn from_str(mut s: &str) -> Result<Self, Self::Err> {
///         s = s.strip_prefix('(').ok_or(())?;
///         s = s.strip_suffix(')').ok_or(())?;
///         let (x, y) = s.split_once(',').ok_or(())?;
///         Ok(Position {
///             x: x.parse().map_err(|_| ())?,
///             y: y.trim().parse().map_err(|_| ())?,
///         })
///     }
/// }
///
/// let pos: Position = parse!("This is a position: (1, 2)", "This is a position: {}");
/// assert_eq!(pos.x, 1);
/// assert_eq!(pos.y, 2);
/// ```
#[proc_macro_derive(Parse, attributes(prse))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Derive);
    input.into_token_stream().into()
}
