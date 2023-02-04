#![allow(unused)]
use prse::{parse, Parse};

#[derive(Parse)]
#[prse = "{x} - {y}"]
union A {
    x: usize,
    y: u32,
}

#[derive(Parse)]
struct B {
    #[prse = "{x} - {y}"]
    x: usize,
    y: usize,
}

#[derive(Parse)]
struct C {
    y: usize,
    #[prse = "{x} - {y}"]
    x: usize,
}

#[derive(Parse)]
#[prse = "D: {}"]
struct D;

#[derive(Parse)]
#[prse = "E: {0}"]
struct E;

#[derive(Parse)]
#[prse = "F: {x}"]
struct F;

#[derive(Parse)]
#[prse = "F: {x} {y}"]
struct G {
    y: usize,
    #[prse = "{x} - {y}"]
    x: usize,
}

#[derive(Parse)]
#[prse = "G: {a}"]
#[prse = "G: {a} "]
struct H {
    a: usize,
}

#[derive(Parse)]
#[prse = "I"]
enum I {
    Unit,
}

#[derive(Parse)]
#[prse = "J"]
enum J {
    #[prse = "J"]
    Unit,
}

#[derive(Parse)]
enum K {
    Tup(#[prse = "K: {}"] usize, usize),
}

#[derive(Parse)]
enum L {
    #[prse = "L"]
    Unit,
    #[prse = "L1"]
    Unit2,
    Unit3,
}

#[derive(Parse)]
enum M {
    S {
        x: usize,
        #[prse = "Test: {y}"]
        y: usize,
    },
}

#[derive(Parse)]
enum N {
    #[prse("N")]
    N,
}

#[derive(Parse)]
enum O {
    #[prse]
    O,
}

#[derive(Parse)]
enum P {
    #[prse = "{:,:0}"]
    P1(u32),
}

#[derive(Parse)]
#[prse = "{x:,:0}"]
struct Q {
    x: u32,
}

fn main() {}
