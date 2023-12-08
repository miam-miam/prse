use crate::{Parse, ParseError, __private};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::str::CharIndices;
use memchr::memmem::FindIter;

/// An iterator that takes a string and parses all items between each separator.
///
/// It is produced from [`parse!`](crate::parse) and [`try_parse!`](crate::try_parse)'s Iterator
/// repetition when given a separator, otherwise [`ParseChars`] is used instead.
///
/// ```
/// # use prse::{Parse, ParseIter, parse};
/// #[derive(Parse)]
/// #[prse = "Game {count}: {results:-:0}"]
/// struct Game<'a> {
///     count: u32,
///     results: ParseIter<'a, u32>
/// }
///
/// let mut game: Game = parse!("Game 2: 2-3", "{}");
/// assert_eq!(game.count, 2);
/// assert_eq!(game.results.next(), Some(Ok(2)));
/// assert_eq!(game.results.next(), Some(Ok(3)));
/// assert_eq!(game.results.next(), None);
/// ```
#[derive(Debug)]
pub struct ParseIter<'a, T: Parse<'a>> {
    finder: FindIter<'a, 'a>,
    is_multi: bool,
    separator_size: usize,
    string: &'a str,
    last_match_idx: usize,
    phantom: PhantomData<T>,
}

impl<'a, T: Parse<'a>> ParseIter<'a, T> {
    #[doc(hidden)]
    /// Not part of public api, used to create the iterator.
    pub fn new(string: &'a str, separator: &'a str, is_multi: bool) -> Self {
        Self {
            finder: memchr::memmem::find_iter(string.as_bytes(), separator.as_bytes()),
            separator_size: separator.as_bytes().len(),
            is_multi,
            string,
            last_match_idx: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Parse<'a>> Iterator for ParseIter<'a, T> {
    type Item = Result<T, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.finder.by_ref() {
            if let Some(slice) = self.string.get(self.last_match_idx..idx) {
                self.last_match_idx = idx + self.separator_size;
                if !self.is_multi || !slice.is_empty() {
                    return Some(__private::add_err_multi_context(
                        T::from_str(slice),
                        self.string,
                        slice,
                    ));
                }
            }
        }
        if let Some(slice) = self.string.get(self.last_match_idx..) {
            self.last_match_idx = self.string.len() + 1;
            if !slice.is_empty() {
                return Some(__private::add_err_multi_context(
                    T::from_str(slice),
                    self.string,
                    slice,
                ));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match (
            self.string.len().checked_sub(self.last_match_idx),
            self.is_multi,
        ) {
            (None | Some(0), _) => (0, Some(0)),
            (Some(haystack_len), false) => (0, Some(1 + haystack_len / self.separator_size)),
            (Some(haystack_len), true) => (0, Some(1 + haystack_len / (self.separator_size + 1))),
        }
    }
}

impl<'a, T: Parse<'a>> FusedIterator for ParseIter<'a, T> {}

/// An iterator that takes a string and parses all chars individually.
///
/// It is produced from [`parse!`](crate::parse) and [`try_parse!`](crate::try_parse)'s Iterator
/// repetition when not given a separator, otherwise [`ParseIter`] is used instead.
///
/// ```
/// # use prse::{Parse, ParseChars, parse};
/// #[derive(Parse, Debug, Eq, PartialEq)]
/// enum SimpleAlphabet {
///     #[prse = "a"]
///     #[prse = "A"]
///     A,
///     #[prse = "🛫"]
///     Airplane,
///     #[prse = "{}"]
///     AnythingElse(char),
/// }
///
/// let mut word: ParseChars<SimpleAlphabet> = parse!("a🛫Ab", "{::0}");
/// assert_eq!(word.next(), Some(Ok(SimpleAlphabet::A)));
/// assert_eq!(word.next(), Some(Ok(SimpleAlphabet::Airplane)));
/// assert_eq!(word.next(), Some(Ok(SimpleAlphabet::A)));
/// assert_eq!(word.next(), Some(Ok(SimpleAlphabet::AnythingElse('b'))));
/// assert_eq!(word.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct ParseChars<'a, T: Parse<'a>> {
    chars: CharIndices<'a>,
    string: &'a str,
    phantom: PhantomData<T>,
}

impl<'a, T: Parse<'a>> ParseChars<'a, T> {
    #[doc(hidden)]
    /// Not part of public api, used to create the iterator.
    pub fn new(string: &'a str) -> Self {
        Self {
            chars: string.char_indices(),
            string,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Parse<'a>> Iterator for ParseChars<'a, T> {
    type Item = Result<T, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(start, c)| {
            let slice = self.string.get(start..(start + c.len_utf8())).unwrap();
            __private::add_err_multi_context(T::from_str(slice), self.string, slice)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chars.size_hint()
    }

    fn count(self) -> usize {
        self.chars.count()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T: Parse<'a>> DoubleEndedIterator for ParseChars<'a, T> {
    fn next_back(&mut self) -> Option<Result<T, ParseError>> {
        self.chars.next_back().map(|(start, c)| {
            let slice = self.string.get(start..(start + c.len_utf8())).unwrap();
            __private::add_err_multi_context(T::from_str(slice), self.string, slice)
        })
    }
}

impl<'a, T: Parse<'a>> FusedIterator for ParseChars<'a, T> {}
