use prse::{parse, Parse};

#[derive(Parse, Eq, PartialEq)]
#[prse = "({:,:})"]
struct Wrapper(Vec<i32>);

fn main() {
    let l = "test: 5,8,9";
    let v: Vec<u32> = parse!(l, "test: {:,:   }");
}
