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
        let s = String::from("Dashing through the snow.");
        let r = r#"Dashing through the snow."#;
        let thing: &str = parse!(s, "Dashing through the {}.");
        assert_eq!(thing, "snow");
        let thing: &str = parse!(r, "Dashing through the {}.");
        assert_eq!(thing, "snow");

        let thing: &str = parse!(s, r#"Dashing through the {}."#);
        assert_eq!(thing, "snow");

        let c = "a is a char.";
        let char: char = parse!(c, "{} is a char.");
        assert_eq!(char, 'a');
        let l = "a is a char.";
        let string: String = parse!(l, "{} is a char.");
        assert_eq!(string, "a");
    }

    #[test]
    fn errors() {
        let l = "I love the following: bananas, apples, oranges.";
        let case: Result<&str, _> = try_parse!(l, "I love the followin: {}.");
        assert_eq!(
            case,
            Err(ParseError::Literal {
                expected: String::from("I love the followin: "),
                found: String::from(l)
            })
        );
        let case: Result<u32, _> = try_parse!(l, "I love the following: {}.");
        assert!(case.is_err());

        let case: Result<Vec<u32>, _> = try_parse!(l, "I love the following: {:, :}.");
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
}
