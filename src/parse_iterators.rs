use crate::{Parse, ParseError};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::str::CharIndices;
use memchr::memmem::FindIter;

#[derive(Debug)]
pub struct ParseIter<'a, T: Parse<'a>> {
    finder: FindIter<'a, 'a>,
    is_multi: bool,
    needle_size: usize,
    haystack: &'a str,
    last_match_idx: usize,
    phantom: PhantomData<T>,
}

impl<'a, T: Parse<'a>> ParseIter<'a, T> {
    pub fn new(haystack: &'a str, needle: &'a str, is_multi: bool) -> Self {
        Self {
            finder: memchr::memmem::find_iter(haystack.as_bytes(), needle.as_bytes()),
            needle_size: needle.as_bytes().len(),
            is_multi,
            haystack,
            last_match_idx: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Parse<'a>> Iterator for ParseIter<'a, T> {
    type Item = Result<T, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.finder.by_ref() {
            if let Some(slice) = self.haystack.get(self.last_match_idx..idx) {
                self.last_match_idx = idx + self.needle_size;
                if !self.is_multi || !slice.is_empty() {
                    return Some(T::from_str(slice));
                }
            }
        }
        if let Some(slice) = self.haystack.get(self.last_match_idx..) {
            self.last_match_idx = self.haystack.len() + 1;
            if !slice.is_empty() {
                return Some(T::from_str(slice));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match (
            self.haystack.len().checked_sub(self.last_match_idx),
            self.is_multi,
        ) {
            (None | Some(0), _) => (0, Some(0)),
            (Some(haystack_len), false) => (0, Some(1 + haystack_len / self.needle_size)),
            (Some(haystack_len), true) => (0, Some(1 + haystack_len / (self.needle_size + 1))),
        }
    }
}

impl<'a, T: Parse<'a>> FusedIterator for ParseIter<'a, T> {}

#[derive(Debug, Clone)]
pub struct ParseChars<'a, T: Parse<'a>> {
    chars: CharIndices<'a>,
    haystack: &'a str,
    phantom: PhantomData<T>,
}

impl<'a, T: Parse<'a>> ParseChars<'a, T> {
    pub fn new(haystack: &'a str) -> Self {
        Self {
            chars: haystack.char_indices(),
            haystack,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Parse<'a>> Iterator for ParseChars<'a, T> {
    type Item = Result<T, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|(start, c)| {
            let slice = self.haystack.get(start..(start + c.len_utf8())).unwrap();
            T::from_str(slice)
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
            let slice = self.haystack.get(start..(start + c.len_utf8())).unwrap();
            T::from_str(slice)
        })
    }
}

impl<'a, T: Parse<'a>> FusedIterator for ParseChars<'a, T> {}
