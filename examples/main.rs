use prse_derive::parse;

fn main() {
    let instructions = include_str!("input.txt");
    let x: i32 = instructions
        .lines()
        .filter(|l| l != &"noop|")
        .map(|l| parse!(l, "addx {} {}|"))
        .map(|(x, y): (u32, i32)| (x as i32) + y)
        .sum();
    println!("The register x would equal {x}.")
}

fn _main1() {
    let instructions = include_str!("input.txt");
    let x: i32 = instructions
        .lines()
        .filter(|l| l != &"noop")
        .map(|l| {
            let mut __prse_input: &str = l;
            let mut __prse_parse;
            (__prse_parse, __prse_input) = l.split_once("addx ").unwrap();
            (__prse_parse, __prse_input) = l.split_once(' ').unwrap();
            let __prse_1 = __prse_parse.parse().unwrap();
            __prse_parse = __prse_input;
            let __prse_3 = __prse_parse.parse().unwrap();
            (__prse_1, __prse_3)
        })
        .map(|(x, y): (u32, i32)| (x as i32) + y)
        .sum();
    println!("The register x would equal {x}.")
}

// fn main() {
//     let instructions = include_str!("input.txt");
//     let x: i32 = instructions
//         .lines()
//         .filter(|l| l != &"noop")
//         .map(|mut l: &str| {
//             let mut p;
//             (p, l) = l.split_once("addx ").unwrap();
//             (p, l) = l.split_once(" ").unwrap();
//             let x = p.parse().unwrap();
//             let y = l.parse().unwrap();
//             (x, y)
//         })
//         .map(|(x, y): (u32, i32)| (x as i32) + y)
//         .sum();
//     println!("The register x would equal to {x}.")
// }
