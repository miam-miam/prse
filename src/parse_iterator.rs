use crate::{Parse, ParseError};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use memchr::memmem::FindIter;

#[derive(Debug)]
pub struct ParseIter<'a, T: Parse<'a>> {
    finder: FindIter<'a, 'a>,
    is_multi: bool,
    needle_size: usize,
    haystack: &'a str,
    phantom: PhantomData<T>,
    last_match: usize,
}

impl<'a, T: Parse<'a>> ParseIter<'a, T> {
    pub fn new(haystack: &'a str, needle: &'a str, is_multi: bool) -> Self {
        Self {
            finder: memchr::memmem::find_iter(haystack.as_bytes(), needle.as_bytes()),
            needle_size: needle.as_bytes().len(),
            is_multi,
            haystack,
            phantom: PhantomData,
            last_match: 0,
        }
    }
}

impl<'a, T: Parse<'a>> Iterator for ParseIter<'a, T> {
    type Item = Result<T, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.finder.by_ref() {
            if let Some(slice) = self.haystack.get(self.last_match..idx) {
                self.last_match = idx + self.needle_size;
                if !self.is_multi || !slice.is_empty() {
                    return Some(T::from_str(slice));
                }
            }
        }
        if let Some(slice) = self.haystack.get(self.last_match..) {
            self.last_match = self.haystack.len() + 1;
            if !slice.is_empty() {
                return Some(T::from_str(slice));
            }
        }
        None
    }
}

impl<'a, T: Parse<'a>> FusedIterator for ParseIter<'a, T> {}
