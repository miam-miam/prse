#[cfg(test)]
mod tests {
    use prse::{parse, try_parse, Parse, ParseError};

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
    fn positional_tests() {
        let numbers = "6 1 5 3 4 2 0";
        let (zero, one, two, three, four, five, six): (i32, i32, i32, i32, i32, i32, i32) =
            parse!(numbers, "{6} {1} {5} {3} {4} {2} {0}");
        assert_eq!(
            (0..=6).collect::<Vec<i32>>(),
            vec![zero, one, two, three, four, five, six]
        );

        let numbers = "test: 5 - 9";
        let var: &str;
        let (rhs, lhs): (u32, u32) = parse!(numbers, "{var} {1} - {0}");
        assert_eq!(lhs, 5);
        assert_eq!(rhs, 9);
        assert_eq!(var, "test:")
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

    #[test]
    #[should_panic]
    fn check_parse_unwrap() {
        let input = String::from("There are 7 boos.");
        let num: u32;
        parse!(input, "There are {num} bos.");
        drop(input);
        assert_eq!(num, 7)
    }

    #[test]
    fn empty_literal() {
        let input = "";
        parse!(input, "");
        let input = "Test";
        parse!(input, "Test")
    }

    #[derive(Debug, Parse)]
    struct Position {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Parse)]
    #[prse = "({x}, {y})"]
    struct Position2 {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Parse, Eq, PartialEq)]
    enum Position3 {
        #[prse = "({x}, {y})"]
        Position { x: i32, y: i32 },
        #[prse = "({})"]
        SinglePositon(i32),
        #[prse = "()"]
        NoPosition,
    }

    impl std::str::FromStr for Position {
        type Err = ();

        fn from_str(mut s: &str) -> Result<Self, Self::Err> {
            s = s.strip_prefix('(').ok_or(())?;
            s = s.strip_suffix(')').ok_or(())?;
            let (x, y) = s.split_once(',').ok_or(())?;
            Ok(Position {
                x: x.parse().map_err(|_| ())?,
                y: y.trim().parse().map_err(|_| ())?,
            })
        }
    }

    #[derive(Debug, Parse, Eq, PartialEq)]
    #[prse = "({x:::}, {y:::3})"]
    struct Position4 {
        x: Vec<i32>,
        y: [i32; 3],
    }

    #[test]
    fn doc_test() {
        let pos: Position = parse!("This is a position: (1, 2)", "This is a position: {}");
        let pos2: Position2 = parse!("This is a position: (-4, 5)", "This is a position: {}");
        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 2);
        assert_eq!(pos2.x, -4);
        assert_eq!(pos2.y, 5);
        let pos0: Position3 = parse!("This is a position: (1, 2)", "This is a position: {}");
        let pos1: Position3 = parse!("This is a position: (3)", "This is a position: {}");
        let pos2: Position3 = parse!("This is a position: ()", "This is a position: {}");
        assert_eq!(pos0, Position3::Position { x: 1, y: 2 });
        assert_eq!(pos1, Position3::SinglePositon(3));
        assert_eq!(pos2, Position3::NoPosition);
        let pos4: Position4 = parse!("Position: (5:9:3, 9:8:7)", "Position: {}");
        assert_eq!(
            pos4,
            Position4 {
                x: vec![5, 9, 3],
                y: [9, 8, 7]
            }
        )
    }
}
