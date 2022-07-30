// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::HashMap;

use kona_source::pos::Pos;

use crate::source_iter::SourceIter;
use crate::char_spec::*;
use crate::token::{TokenKind, LitKind, Token, TriviaKind};

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str, start_pos: Pos) -> impl Iterator<Item = Token> + '_ {
    let mut iter = SourceIter::new(input, start_pos);
    std::iter::from_fn(move || {
        if iter.is_eof() {
            None
        } else {
            Some(iter.lex_token())
        }
    })
}

lazy_static! {
    static ref KEYWORD_TABLE: HashMap<&'static str, TokenKind> = [
        ("else", TokenKind::Else),
        ("fn", TokenKind::Fn),
        ("if", TokenKind::If),
        ("in", TokenKind::In),
        ("infix", TokenKind::Infix),
        ("let", TokenKind::Let),
        ("then", TokenKind::Then),
        ("true", TokenKind::Lit(LitKind::Bool)),
        ("false", TokenKind::Lit(LitKind::Bool)),
    ].into_iter().collect::<HashMap<_, _>>();

    static ref MAX_KEYWORD_LEN: usize =
        KEYWORD_TABLE.keys().map(|s| s.len()).max().unwrap();

    static ref OP_LIKE_PUNCT_TABLE: HashMap<&'static str, TokenKind> = [
        ("=>", TokenKind::DArrow), ("=", TokenKind::Eq),
    ].into_iter().collect::<HashMap<_, _>>();

    static ref OP_LIKE_PUNCT_LEN: usize =
        OP_LIKE_PUNCT_TABLE.keys().map(|s| s.len()).max().unwrap();
}

impl SourceIter<'_> {
    fn lex_token(&mut self) -> Token {
        let kind = match self.peek_fst() {
            // Comment or operator start with '/'.
            '/' => {
                match self.peek_snd() {
                    '-' => self.lex_block_comment(), // Multi-line comment.
                    '/' => self.lex_line_comment(),  // Single-line comment.
                    _ => self.lex_operator(),
                }
            }

            c if is_operator_part(c) => self.lex_operator(),

            // Whitespace sequence.
            c if is_inline_space(c) => self.lex_inline_spaces(),

            // End of line.
            c if is_line_break(c) => self.lex_end_of_line(),

            // Alphanumeric identifier or keyword.
            c if is_ident_head(c) => self.lex_ident(),

            // Symbolic identifier or keyword.
            c if is_operator_part(c) => self.lex_operator(),

            // Integer literal and float literal.
            '0'..='9' => self.lex_number(),

            // String literal.
            '"' => self.lex_string(),

            ';' => { self.eat(); TokenKind::Semi }
            '(' => { self.eat(); TokenKind::LParen }
            ')' => { self.eat(); TokenKind::RParen }

            _ => { self.eat(); TokenKind::Invalid }
        };
        Token::new(kind, self.consume_span())
    }

    fn lex_line_comment(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '/' && self.eat() == '/');
        self.eat_while(|c| c != '\n');
        TokenKind::Trivia(TriviaKind::SingleLineComment)
    }

    fn lex_block_comment(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '/' && self.eat() == '-');

        let mut depth = 1;
        while let Some(c) = self.next() {
            match c {
                '/' if self.peek_fst() == '-' => {
                    self.eat();
                    depth += 1;
                }
                '-' if self.peek_fst() == '/' => {
                    self.eat();
                    depth -= 1;
                    if depth == 0 {
                        return TokenKind::Trivia(TriviaKind::MultiLineComment);
                    }
                }
                _ => (),
            }
        }

        // Unterminated multi-line comment.
        TokenKind::Invalid
    }

    fn lex_inline_spaces(&mut self) -> TokenKind {
        debug_assert!(is_inline_space(self.eat()));

        self.eat_while(is_inline_space);
        TokenKind::Trivia(TriviaKind::Whitespace)
    }

    fn lex_end_of_line(&mut self) -> TokenKind {
        debug_assert!(is_line_break(self.peek_fst()));

        if self.eat() == '\r' && self.peek_fst() == '\n' {
            self.eat(); // Consume '\n' in CRLF.
        }

        TokenKind::Trivia(TriviaKind::Eol)
    }

    fn lex_ident(&mut self) -> TokenKind {
        debug_assert!(is_ident_head(self.peek_fst()));

        let mut ident = String::new();
        while is_ident_part(self.peek_fst()) {
            ident.push(self.eat());
        }

        KEYWORD_TABLE
            .get(ident.as_str())
            .cloned()
            .unwrap_or(TokenKind::Ident)
    }

    fn lex_operator(&mut self) -> TokenKind {
        debug_assert!(is_operator_part(self.peek_fst()));

        let mut ident = String::new();
        while is_operator_part(self.peek_fst()) {
            ident.push(self.eat());
        }

        OP_LIKE_PUNCT_TABLE
            .get(ident.as_str())
            .cloned()
            .unwrap_or(TokenKind::Op)
    }

    fn lex_number(&mut self) -> TokenKind {
        debug_assert!(matches!(self.eat(), '0'..='9'));

        self.eat_while(is_digit);

        if self.peek_fst() == '.' && is_digit(self.peek_snd()) {
            self.eat(); // Eat '.'.
            self.eat_while(is_digit);
            TokenKind::Lit(LitKind::Float)
        } else {
            TokenKind::Lit(LitKind::Int)
        }
    }

    fn lex_string(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '"');

        while let Some(c) = self.next() {
            match c {
                '"' => {
                    return TokenKind::Lit(LitKind::String)
                },
                '\\' if matches!(self.peek_fst(), '0' | '\\' | 't'
                                                | 'n' | 'r' | '"') => {
                    self.eat();
                },
                _ => (),
            }
        }

        // Unterminated string literal.
        TokenKind::Invalid
    }
}
