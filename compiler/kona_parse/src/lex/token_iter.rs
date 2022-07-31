// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_source::pos::Pos;

use super::{token::{Token, TokenKind}, source_iter::SourceIter, tokenize};

/// Peekable iterator of token stream.
pub struct TokenIter<'src> {
    iter: SourceIter<'src>,
    peeked_token: Option<Token>,
}

impl<'src> TokenIter<'src> {
    /// Creates a new token iterator from the input string.
    pub fn new(input: &str, start_pos: Pos) -> TokenIter {
        let start_pos = start_pos.into();
        let iter = SourceIter::new(input, start_pos);
        TokenIter { iter, peeked_token: None }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if let Some(next) = &self.peeked_token {
            return Some(next.clone());
        }

        loop {
            if self.iter.is_eof() {
                return None;
            }

            let next = self.iter.lex_token();
            if !matches!(next.kind, TokenKind::Trivia(_)) {
                self.peeked_token = Some(next.clone());
                return Some(next)
            }
        };
    }

    pub fn eat(&mut self) -> Option<Token> {
        self.peek();
        return std::mem::replace(&mut self.peeked_token, None);
    }
}
