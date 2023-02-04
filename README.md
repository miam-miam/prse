Prse
==================
[<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/miam-miam100/prse)
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/prse)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/prse)

Prse is a small string parsing library with an emphasis on speed and ease of use. (It's also no-std compatible!)

It provides the [`parse!`] macro which allows you to easily parse strings into any type using a format args like syntax.

<sup>Prse currently supports rustc 1.59 and above.</sup>

[`parse!`]: https://docs.rs/prse/latest/prse/macro.parse.html

## Examples

```rust
use prse::parse;

let input = "5 + -2 = 3";

let total: i32;
let (lhs, rhs): (i32, i32) = parse!(input, "{} + {} = {total}");

assert_eq!(lhs + rhs, total);
```

It also allows you to parse into multiple variables separated by a separator in a single go.

```rust
use prse::parse;

let input = "My farm contains some amount of booleans: true || false || true || false";
let many: Vec<bool>;

// the variable to store the answer in is many and the separator is equal to " || "
parse!(input, "My farm contains some amount of booleans: {many: || :}");

assert_eq!(many, vec![true, false, true, false]);
```

You can use the [`try_parse!`] macro if you don't want to panic when the parsing fails.

[`try_parse!`]: https://docs.rs/prse/latest/prse/macro.try_parse.html

```rust
use prse::try_parse;
use std::path::PathBuf;

let input = "cd C:\\windows\\system32";
let path: Result<PathBuf, _> = try_parse!(input, "cd {}");

assert_eq!(path.unwrap(), PathBuf::from("C:\\windows\\system32"));
```

Additionally you can use the [`Parse`] derive macro to help you parse custom types.
```rust
use prse::{parse, Parse};

#[derive(Parse, PartialEq, Eq, Debug)]
#[prse = "({x}, {y})"]
struct Position { 
    x: i32, 
    y: i32,
}

let input = "(1, 3) + (-2, 9)";

let (lhs, rhs): (Position, Position) = parse!(input, "{} + {}");

assert_eq!(lhs, Position {x: 1, y: 3});
assert_eq!(rhs, Position {x: -2, y: 9});
```

[`Parse`]: https://docs.rs/prse/latest/prse/derive.Parse.html

## Alternatives


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>