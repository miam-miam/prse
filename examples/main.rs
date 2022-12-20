#![no_std]

use prse::parse;

fn main() {
    let instructions = include_str!("input.txt");

    let x: i32 = instructions
        .lines()
        .map(|l| parse!(l, "{}|{:,:2}"))
        .map(|(x, [y, yy]): (u32, [i32; 2])| (x as i32) + y + yy)
        .sum();
    assert_eq!(x, 14)
}
