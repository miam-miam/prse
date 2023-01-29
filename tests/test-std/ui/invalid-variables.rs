use prse::parse;

fn main() {
    let l = "test: 5";

    parse!(l, "test: {foo()}");
    parse!(l, "test: {-8}");
    parse!(l, "test: {256}")
}
