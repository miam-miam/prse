use prse::parse;

fn main() {
    let l = "test: 5,8,9";
    let v: Vec<u32> = parse!(l, "test: {:,:   }");
}
