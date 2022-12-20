#![warn(missing_docs)]
//! A helper crate for prse, the small string parsing library.

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use invocation::ParseInvocation;
use proc_macro::TokenStream;
use quote::ToTokens;

mod instructions;
mod invocation;
/// The `parse` macro allows you to parse a string into any type that implements [`LendingFromStr`](trait.LendingFromStr.html).
///
/// ```ignore
/// let input = "5 + -2 = 3";
///
/// let mut total = 0_i32;
/// let (lhs, rhs): (i32, i32) = parse!("{} + {} = {total}");
///
/// assert_eq!(lhs + rhs, total);
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
/// let mut many: Vec<bool> = vec![];
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
/// One important note is that since the iterator is evaluated lazily it will always return an iterator of [`Results`](Result).
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
/// # Syntax
///
/// The [`parse!`] macro uses a literal with `{}` brackets to denote where it should
/// try to parse into a type. The macro must fully consume the string as it will otherwise
/// return an error. If this is not what you want you can create a dummy capture variable
/// that captures the rest of the string.
///
/// ```ignore
/// let input = "Hello world! Today is a great day!";
/// let mut world = "";
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
/// let mut num = 0_u128;
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

/// Returns a [`Result`] instead of unwrapping like [`parse!`]. The [`Result`] has
/// [`ParseError`](enum.ParseError.html) as an error type.
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
