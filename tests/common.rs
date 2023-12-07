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
}
