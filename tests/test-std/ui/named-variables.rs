use prse::parse;

fn main() {
    let l = "test: 5";

    parse!(l, "test: {var_that_definitely_exists}");
    parse!(l, "test: {var_that_definitely_exists:,:}");
    parse!(l, "test: {var_that_definitely_exists:,:5}");
    parse!(l, "test: {var_that_definitely_exists:,:0}");
}
