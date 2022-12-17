use prse::parse;

fn main() {
    let instructions = include_str!("input.txt");

    let x: i32 = instructions
        .lines()
        .map(|l| parse!(l, "{}|{}|"))
        .map(|(x, y): (u32, i32)| (x as i32) + y)
        .sum();
    println!("The register x would equal {x}.");
}

// fn main() {
//     let instructions = include_str!("input.txt");
//     let x: i32 = instructions
//         .lines()
//         .map(|l| {
//             use prse::*;
//             let __prse_func = || {
//                 let mut __prse_input: &str = &l;
//                 let mut __prse_parse;
//                 (__prse_parse, __prse_input) =
//                     __prse_input
//                         .split_once('|')
//                         .ok_or_else(|| ParseError::Literal {
//                             expected: ('|').into(),
//                             found: __prse_input.into(),
//                         })?;
//                 let __prse_0 = __prse_parse.lending_parse()?;
//                 __prse_parse = __prse_input;
//                 let mut __prse_iter = __prse_parse.split(',').map(|p| p.lending_parse());
//                 let __prse_2 = [
//                     __prse_iter.next().ok_or_else(|| ParseError::Multi {
//                         expected: 2u8,
//                         found: 0u8,
//                     })??,
//                     __prse_iter.next().ok_or_else(|| ParseError::Multi {
//                         expected: 2u8,
//                         found: 1u8,
//                     })??,
//                 ];
//                 Ok::<_, ParseError>((__prse_0, __prse_2))
//             };
//             __prse_func().unwrap()
//         })
//         .map(|(x, [y, yy]): (u32, [i32; 2])| (x as i32) + y + yy)
//         .sum();
//
//     println!("The register x would equal {x}.");
// }
