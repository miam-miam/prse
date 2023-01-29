#[cfg(test)]
mod tests {
    use prse::{parse, try_parse, ParseError};

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("ui/*.rs");
    }

    #[test]
    fn yum() {
        let input = "Person 5: Hello Bob!";
        let mut name: &str = "";
        // let five: u32 = parse!(input, "Person {}: Hello {name}!");
        let five = {
            use ::prse::{ExtParseStr, LendingFromStr};
            fn __prse_func<'a, T0: LendingFromStr<'a>, T1: LendingFromStr<'a>>(
                mut __prse_input: &'a str,
            ) -> Result<(T0, T1), ParseError> {
                let mut __prse_parse;
                (__prse_parse, __prse_input) =
                    __prse_input.split_once("Person ").ok_or_else(|| {
                        ::prse::ParseError::Literal {
                            expected: ("Person ").into(),
                            found: __prse_input.into(),
                        }
                    })?;
                (__prse_parse, __prse_input) =
                    __prse_input.split_once(": Hello ").ok_or_else(|| {
                        ::prse::ParseError::Literal {
                            expected: (": Hello ").into(),
                            found: __prse_input.into(),
                        }
                    })?;
                let __prse_1 = __prse_parse.lending_parse()?;
                (__prse_parse, __prse_input) =
                    __prse_input
                        .split_once('!')
                        .ok_or_else(|| ::prse::ParseError::Literal {
                            expected: ('!').into(),
                            found: __prse_input.into(),
                        })?;
                let __prse2 = __prse_parse.lending_parse()?;
                if !__prse_input.is_empty() {
                    return Err(::prse::ParseError::Literal {
                        expected: "".into(),
                        found: __prse_input.into(),
                    });
                }
                Ok::<_, ::prse::ParseError>((__prse_1, __prse2))
            }
            #[allow(clippy::needless_borrow)]
            // use this for parse but create an unwrap function that boils down to panic!("Unable to parse {input:?}: {e:?}"),
            // let __prse_input: &str = &input;
            // let (__prse1, __prse2) = __prse_func(__prse_input).unwrap();
            // name = __prse2;
            // __prse1
            // use this for try_parse
            match __prse_func(&input) {
                Ok((__prse1, __prse2)) => {
                    name = __prse2;
                    Ok(__prse1)
                }
                // Err(e) => panic!("Unable to parse {input:?}: {e:?}"),
                Err(e) => Err(e),
            }
        };
        // match five {
        //     5_u64 => {
        //         println!("yay")
        //     }
        //     _ => {}
        // }
        assert_eq!(five, Ok(5_u64));
        assert_eq!(name, "Bob");
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
}
