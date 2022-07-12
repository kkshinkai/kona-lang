// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::HashMap;

use crate::source_iter::SourceIter;
use crate::char_spec::*;
use crate::token::{IdentKind, LitKind, TriviaKind};
use crate::token::{Token, TokenKind::{self, *}};

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut iter = SourceIter::new(input);
    std::iter::from_fn(move || {
        if iter.is_eof() {
            None
        } else {
            iter.reset_consumed_len();
            Some(iter.lex_token())
        }
    })
}

lazy_static! {
    static ref ALPHA_KEYWORD_TABLE: HashMap<&'static str, TokenKind> = [
        ("else", Else), ("end", End), ("fn", Fn), ("if", If), ("in", In),
        ("let", Let), ("op", Op), ("then", Then), ("val", Val),
        ("true", Lit(LitKind::Bool)), ("false", Lit(LitKind::Bool)),
    ].into_iter().collect::<HashMap<_, _>>();

    static ref MAX_ALPHA_KEYWORD_LEN: usize =
        ALPHA_KEYWORD_TABLE.keys().map(|s| s.len()).max().unwrap();

    static ref SYMBOL_KEYWORD_TABLE: HashMap<&'static str, TokenKind> = [
        ("=>", DArrow), ("=", Eq),
    ].into_iter().collect::<HashMap<_, _>>();

    static ref MAX_SYMBOL_KEYWORD_LEN: usize =
        SYMBOL_KEYWORD_TABLE.keys().map(|s| s.len()).max().unwrap();
}

impl SourceIter<'_> {
    fn lex_token(&mut self) -> Token {
        let kind = match self.peek_fst() {
            // Block comment or symbolic identifier start with '/'.
            '/' if self.peek_snd() == '-' => self.lex_block_comment(),

            // Line comment or symbolic identifier start with '--'.
            '-' if self.peek_snd() == '-' => {
                let next = self.peek_trd();
                if is_inline_space(next) || is_linebreak(next) {
                    self.lex_line_comment()
                } else {
                    self.lex_sym_ident()
                }
            }

            c if is_sym_ident(c) => self.lex_sym_ident(),

            // Whitespace sequence.
            c if is_inline_space(c) => self.lex_inline_spaces(),

            // End of line.
            '\n' | '\r' => self.lex_end_of_line(),

            // Alphanumeric identifier or keyword.
            c if is_alpha_ident_head(c) => self.lex_alpha_ident(),

            // Symbolic identifier or keyword.
            c if is_sym_ident(c) => self.lex_sym_ident(),

            // Integer literal and float literal.
            '0'..='9' => self.lex_number(),

            // String literal.
            '"' => self.lex_string(),

            ';' => { self.eat(); Semi }
            '(' => { self.eat(); LParen }
            ')' => { self.eat(); RParen }

            _ => { self.eat(); Invalid }
        };
        Token::new(kind, self.consumed_len())
    }

    fn lex_line_comment(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '-' && self.eat() == '-');
        self.eat_while(|c| c != '\n');
        Trivia(TriviaKind::SingleLineComment)
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
                        return Trivia(TriviaKind::MultiLineComment {
                            terminated: true,
                        });
                    }
                }
                _ => (),
            }
        }

        Trivia(TriviaKind::MultiLineComment { terminated: false })
    }

    fn lex_inline_spaces(&mut self) -> TokenKind {
        debug_assert!(is_inline_space(self.eat()));

        self.eat_while(is_inline_space);
        Trivia(TriviaKind::Whitespace)
    }

    fn lex_end_of_line(&mut self) -> TokenKind {
        debug_assert!(is_linebreak(self.peek_fst()));

        if self.eat() == '\r' && self.peek_fst() == '\n' {
            self.eat(); // Consume '\n' in CRLF.
        }

        Trivia(TriviaKind::Eol)
    }

    fn lex_alpha_ident(&mut self) -> TokenKind {
        debug_assert!(is_alpha_ident_head(self.peek_fst()));

        let mut ident = String::new();
        while is_alpha_ident_part(self.peek_fst()) {
            ident.push(self.eat());
        }

        ALPHA_KEYWORD_TABLE
            .get(ident.as_str())
            .cloned()
            .unwrap_or(Ident(IdentKind::Alphanumeric))
    }

    fn lex_sym_ident(&mut self) -> TokenKind {
        debug_assert!(is_sym_ident(self.peek_fst()));

        let mut ident = String::new();
        while is_sym_ident(self.peek_fst()) {
            ident.push(self.eat());
        }

        SYMBOL_KEYWORD_TABLE
            .get(ident.as_str())
            .cloned()
            .unwrap_or(Ident(IdentKind::Symbolic))
    }

    fn lex_number(&mut self) -> TokenKind {
        debug_assert!(matches!(self.eat(), '0'..='9'));

        self.eat_while(is_digit);

        if self.peek_fst() == '.' && is_digit(self.peek_snd()) {
            self.eat(); // Eat '.'.
            self.eat_while(is_digit);
            Lit(LitKind::Float)
        } else {
            Lit(LitKind::Int)
        }
    }

    fn lex_string(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '"');

        while let Some(c) = self.next() {
            match c {
                '"' => return Lit(LitKind::String { terminated: true }),
                '\\' if self.peek_fst() == '\\' && self.peek_fst() == '"' => {
                    self.eat();
                }
                _ => (),
            }
        }

        Lit(LitKind::String { terminated: false })
    }
}