// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::str::Chars;

use kona_source::{pos::Pos, span::Span};

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
    /// Start position of the current token, not the start position of this
    /// file, this value will be updated every time [`consume_span`] is called.
    start_pos: Pos,

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
    ///
    /// This value should only be accessed by [`consume_span`].
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
    pub(crate) fn new(source: &'s str, start_pos: Pos) -> SourceIter {
        SourceIter {
            start_pos,
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

    /// Get and consume the span of the current token after all characters of
    /// this token have been eaten.
    pub (crate) fn consume_span(&mut self) -> Span {
        let consumed_len = self.initial_len - self.chars.as_str().len();
        let start_pos = self.start_pos;

        // Set `initial_len` and `start_pos` to the current position.
        self.initial_len = self.chars.as_str().len();
        self.start_pos += consumed_len;

        Span {
            start: start_pos,
            end: start_pos + consumed_len,
        }
    }
}

