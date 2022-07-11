// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::str::Chars;

/// Peekable iterator with position information over a source string.
///
/// This iterator that provides three additional mechanisms on [`str::Chars`]:
///
/// - Peek at the next or more character without advancing the iterator;
/// - Get the cosumed byte count since the last call to [`reset_consumed_len`];
/// - Some useful helper functions for lexing;
///
/// Unlike [`Peekable`], which stores the peeked character in a buffer, this
/// implementation simply iterates forward. The string iteration overhead is
/// small, and the lexer does not need to look forward too many times (usually
/// two is enough).
///
/// [`str::Chars`]: std::str::Chars
/// [`reset_consumed_len`]: crate::source_iter::SourceIter::reset_consumed_len
/// [`Peekable`]: std::iter::Peekable
pub(crate) struct SourceIter<'s> {
    /// The initial number of bytes left in the iterator.
    ///
    /// This length is the number of bytes, not [`char`]s. You can get the
    /// byte count of current token by subtracting the remaining length from
    /// this, i.e. `initial_len - chars.as_str().len()`.
    ///
    /// ```text
    ///         |-len-|                         initial_len - remain_len
    /// let val number = 42 in number * 2 end
    ///         |     |------remain_len------|  chars.as_str().len()
    ///         |--------initial_len---------|  initial_len
    /// ```
    initial_len: usize,

    /// Iterator over source characters.
    chars: Chars<'s>,
}

impl<'s> Iterator for SourceIter<'s> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}

const EOF_CHAR: char = '\0';

impl<'s> SourceIter<'s> {
    pub(crate) fn new(source: &'s str) -> SourceIter {
        SourceIter {
            initial_len: source.len(),
            chars: source.chars(),
        }
    }

    pub(crate) fn peek_fst(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn peek_snd(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn eat(&mut self) -> char {
        self.chars.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_fst()) && !self.is_eof() {
            self.eat();
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn reset_consumed_len(&mut self) {
        self.initial_len = self.chars.as_str().len();
    }

    pub(crate) fn consumed_len(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }
}

