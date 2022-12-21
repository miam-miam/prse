use prse::parse;

fn main() {
    let l = "test: 5";

    parse!(l, "test: {foo()}");
    parse!(l, "test: {0}");
    parse!(l, "test: {-8}");
}
