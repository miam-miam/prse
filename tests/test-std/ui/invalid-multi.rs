use prse::parse;

fn main() {
    let l = "test: 5";

    parse!(l, "test: {:}");
    parse!(l, "test: {::}");
    parse!(l, "test: {:,:999}");
    parse!(l, "test: {:,:-1}");
    parse!(l, "test: {:,:,}");
}
