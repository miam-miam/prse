use prse::parse;

fn main() {
    let l = 5;
    let mut x = 0_i32;

    parse!(l, "test: {x}");
    parse!(342, "test: {x}");
    parse!(test(), "test: {x}")
}

fn test() -> usize {
    0
}
