use prse::parse;

fn main() {
    let l = "test: 5";
    let mut x = 0_i32;

    parse!("test: {x}");
    parse!(l, "test: {x}", 4);
    parse!("test: {x}",);
    parse!("test: {x}", l);
    parse!(l, 2);
}
