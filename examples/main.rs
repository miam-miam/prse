use prse_derive::{parse, try_parse};

fn main() {
    let instructions = include_str!("input.txt");
    let x: i32 = instructions
        .lines()
        .filter(|l| l != &"noop|")
        .map(|l| parse!(l, "{} {}|"))
        .map(|(x, y): (u32, i32)| (x as i32) + y)
        .sum();
    println!("The register x would equal {x}.");

    for x in instructions
        .lines()
        .filter(|l| l != &"noop|")
        .map(|l| try_parse!(l, "{} {}||"))
    {
        let x: Result<(u32, i32), _> = x;
        if let Err(e) = x {
            println!("{}", e);
        }
    }
}

// fn _main1() {
//     let instructions = include_str!("input.txt");
//     let x: i32 = instructions
//         .lines()
//         .filter(|l| l != &"noop")
//         .map(|l| {
//             use prse::*;
//             let __prse_func = || {
//                 let mut __prse_input: &str = &l;
//                 let mut __prse_parse;
//                 (__prse_parse, __prse_input) =
//                     __prse_input
//                         .split_once(' ')
//                         .ok_or_else(|| ParseError::Literal {
//                             expected: ' '.into(),
//                             found: __prse_input.into(),
//                         })?;
//                 let __prse_0 = __prse_parse.lending_parse()?;
//                 (__prse_parse, __prse_input) = __prse_input.split_once('|').unwrap();
//                 let __prse_2 = __prse_parse.lending_parse()?;
//                 Ok::<_, ParseError>((__prse_0, __prse_2))
//             };
//             __prse_func().unwrap()
//         })
//         .map(|(x, y): (u32, i32)| (x as i32) + y)
//         .sum();
//     println!("The register x would equal {x}.")
// }

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
