Prse
==================
[<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/miam-miam100/prse)
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/prse)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/prse)

Provides a new type called Named Tups that can be called using the [`tup!`] macro. Named Tups are structs that can
contain a set of named arguments, effectively they work like normal tuples that can be accessed and created using an
actual name.

[`tup!`]: https://docs.rs/named-tup/latest/named-tup/macro.tup.html

The idea of named tuples is to provide a way to quickly iterate on ideas without having to create a builder struct or
losing the ability to type check at compile time. Named tuples also allow the creation of default values that can
replace nonexistent arguments.

```toml
[dependencies]
prse = "0.1.0"
```

## Examples

```rust
use named_tup::tup;
let count = 5;

// This will have the type of Tup!(count: i32, ingredients: [&str; 3], eggs: bool)
let cakes = tup!(count, ingredients: ["milk", "flower", "sugar"], eggs: true);

// We can just add a price afterwards
let mut cakes = cakes + tup!(price: 3);
// And now it has the type of Tup!(eggs: bool, ingredients: [&str; 3], count: i32, price: i32)

// Once the price is in the tup we can just update it!
cakes.price = 4;

// Will print tup { count: 5, eggs: true, ingredients: ["milk", "flower", "sugar"], price: 4 }
println!("{cakes:?}");
```

To use defaults just annotate the item where you set a field
with [`#[tup_default]`](https://docs.rs/named-tup/latest/named-tup/attr.tup_default.html). Additionally since the
defaulted [`tup!`] is a type you need to convert into it by calling [`.into_tup()`] which can be accessed through
the [`TupInto`] trait.

[`.into_tup()`]: https://docs.rs/named-tup/latest/named-tup/trait.TupInto.html

[`TupInto`]: https://docs.rs/named-tup/latest/named-tup/trait.TupInto.html

```rust
use named_tup::{tup, Tup, tup_default, TupInto};

let options = tup!(read: false, write: true);

// Converts to Tup!(read: false, write: true, create: false, timeout: 5)
open_file("main.rs", options.into_tup());

#[tup_default]
fn open_file(
    path: &str,
    options: Tup!(
        read: bool = true,
        write: bool = false,
        create: bool = false,
        timeout: i32 = 5
    ))
{
    // Open the file
}
```

To test the crate enable the feature `dev-test`.

## Roadmap

- Write some more tests
- Serialise and Deserialise using Serde
- Provide nice looking types for cargo doc

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