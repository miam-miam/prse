use prse::parse;

fn main() {
    let l = "test: 5";

    let mut x = 0;

    parse!(l, "test: {x");
    let y: i32 = parse!(l, "test: {");

    parse!(l, "test: x}");
    let y: i32 = parse!(l, "test: }");

    assert_eq!(5_i32, x);
}
