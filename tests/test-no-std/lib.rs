#![no_std]

#[cfg(test)]
mod tests {
    use prse::{parse, try_parse, ParseError};

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("ui/*.rs");
    }

    #[test]
    fn use_other_strings() {
        let r = r#"Dashing through the snow."#;
        let thing: &str = parse!(r, "Dashing through the {}.");
        assert_eq!(thing, "snow");

        let thing: &str = parse!(r, r#"Dashing through the {}."#);
        assert_eq!(thing, "snow");

        let c = "a is a char.";
        let char: char = parse!(c, "{} is a char.");
        assert_eq!(char, 'a');
    }

    #[test]
    fn errors() {
        let l = "I love the following: bananas, apples, oranges.";
        let case: Result<&str, _> = try_parse!(l, "I love the followin: {}.");
        assert_eq!(case, Err(ParseError::Literal));
        let case: Result<u32, _> = try_parse!(l, "I love the following: {}.");
        assert!(case.is_err());

        let case: Result<[&str; 2], _> = try_parse!(l, "I love the following: {:, :2}.");
        assert_eq!(
            case,
            Err(ParseError::Multi {
                expected: 2,
                found: 3
            })
        );
        let case: Result<[&str; 4], _> = try_parse!(l, "I love the following: {:, :4}.");
        assert_eq!(
            case,
            Err(ParseError::Multi {
                expected: 4,
                found: 3
            })
        );
    }

    #[test]
    fn general_tests() {
        let l = "(5,6) has [0,2,42]";
        let mut x = 5_u32;
        let mut y = 5_i32;
        let v: [u32; 3] = parse!(l, "({x},{y}) has [{:,:3}]");
        assert_eq!((x, y, v), (5, 6, [0, 2, 42]));

        let p: u32 = parse!(l, "(5,6) has [{:,:0}]")
            .flat_map(|i: Result<u32, _>| i.ok())
            .sum();

        assert_eq!(p, 44);
    }

    #[test]
    fn positional_tests() {
        let numbers = "6 1 5 3 4 2 0";
        let (zero, one, two, three, four, five, six): (i32, i32, i32, i32, i32, i32, i32) =
            parse!(numbers, "{6} {1} {5} {3} {4} {2} {0}");
        assert_eq!(
            [0, 1, 2, 3, 4, 5, 6],
            [zero, one, two, three, four, five, six]
        );

        let numbers = "test: 5 - 9";
        let mut var: &str = "";
        let (rhs, lhs): (u32, u32) = parse!(numbers, "{var} {1} - {0}");
        assert_eq!(lhs, 5);
        assert_eq!(rhs, 9);
    }
}
