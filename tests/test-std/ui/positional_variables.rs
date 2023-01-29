use prse::parse;

fn main() {
    let l = "zero one two";
    let num;

    parse!(l, "{0} {0} {1}");
    parse!(l, "{0} {num} {2}");
    parse!(l, "{0} {1} {}");
    parse!(l, "{2} one {0}");
}
