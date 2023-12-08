mod common {
    use prse::{parse, Parse};

    #[test]
    fn empty_literal() {
        let input = "";
        parse!(input, "");
        let input = "Test";
        parse!(input, "Test")
    }

    #[derive(Parse, Eq, PartialEq, Debug)]
    enum SimpleAlphabet {
        #[prse = "A"]
        A,
        #[prse = "B"]
        B,
    }

    #[derive(Parse, Eq, PartialEq, Debug)]
    #[prse = "{:-:2}"]
    struct Alphabets([SimpleAlphabet; 2]);

    #[test]
    fn parse_single_chars() {
        assert_eq!(SimpleAlphabet::A, parse!("A", "{}"));
        assert_eq!(
            Alphabets([SimpleAlphabet::A, SimpleAlphabet::B]),
            parse!("A-B", "{}")
        )
    }

    #[derive(Parse, Debug, PartialEq, Eq)]
    enum Capture<'c> {
        #[prse = "{}"]
        Single(&'c str),
    }

    #[derive(Parse, Debug, Eq, PartialEq)]
    #[prse = "{b} {c:-:2}"]
    struct Lifetimes<'a, 'b> {
        b: Capture<'a>,
        c: [&'b str; 2],
    }

    #[test]
    fn parse_lifetime_derive() {
        assert_eq!(
            Lifetimes {
                b: Capture::Single("yummy"),
                c: ["gummy", "bear"]
            },
            parse!("yummy gummy-bear", "{}")
        );
    }

    #[test]
    fn parse_trim() {
        assert_eq!(
            (false, 2_u8, 3_i32, 5_f64),
            parse!(" false , 2 , 3 , 5.0 ", "{},{},{},{}")
        )
    }

    #[derive(Parse, Debug, Eq, PartialEq)]
    enum Char {
        #[prse = "a:{}"]
        #[prse = "A:{}"]
        A(u32),
        #[prse = "b"]
        B,
    }

    #[test]
    fn parse_multi_enum() {
        let l = "a:4 A:3 b";
        assert_eq!((Char::A(4), Char::A(3), Char::B), parse!(l, "{} {} {}"));
    }

    #[derive(Parse, Eq, PartialEq, Debug)]
    #[prse = "{arr:::!2}"]
    struct MultiSep {
        arr: [u32; 2],
    }

    #[test]
    fn parse_multi_sep() {
        let arr: [u8; 3] = parse!("1-2---3", "{:-:!3}");
        assert_eq!([1, 2, 3], arr);
        assert_eq!(MultiSep { arr: [23, 1] }, parse!(":::23::::1", "{}"))
    }
}
