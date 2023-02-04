#![allow(unused)]
use prse::{parse, LendingFromStr};

#[derive(LendingFromStr)]
#[prse = "{x} - {y}"]
union A {
    x: usize,
    y: u32,
}

#[derive(LendingFromStr)]
struct B {
    #[prse = "{x} - {y}"]
    x: usize,
    y: usize,
}

#[derive(LendingFromStr)]
struct C {
    y: usize,
    #[prse = "{x} - {y}"]
    x: usize,
}

#[derive(LendingFromStr)]
#[prse = "D: {}"]
struct D;

#[derive(LendingFromStr)]
#[prse = "E: {0}"]
struct E;

#[derive(LendingFromStr)]
#[prse = "F: {x}"]
struct F;

#[derive(LendingFromStr)]
#[prse = "F: {x} {y}"]
struct G {
    y: usize,
    #[prse = "{x} - {y}"]
    x: usize,
}

#[derive(LendingFromStr)]
#[prse = "G: {a}"]
#[prse = "G: {a} "]
struct H {
    a: usize,
}

#[derive(LendingFromStr)]
#[prse = "I"]
enum I {
    Unit,
}

#[derive(LendingFromStr)]
#[prse = "J"]
enum J {
    #[prse = "J"]
    Unit,
}

#[derive(LendingFromStr)]
enum K {
    Tup(#[prse = "K: {}"] usize, usize),
}

#[derive(LendingFromStr)]
enum L {
    #[prse = "L"]
    Unit,
    #[prse = "L1"]
    Unit2,
    Unit3,
}

#[derive(LendingFromStr)]
enum M {
    S {
        x: usize,
        #[prse = "Test: {y}"]
        y: usize,
    },
}

#[derive(LendingFromStr)]
enum N {
    #[prse("N")]
    N,
}

#[derive(LendingFromStr)]
enum O {
    #[prse]
    O,
}

#[derive(LendingFromStr)]
enum P {
    #[prse = "{:,:0}"]
    P1(u32),
}

#[derive(LendingFromStr)]
#[prse = "{x:,:0}"]
struct Q {
    x: u32,
}

fn main() {}
