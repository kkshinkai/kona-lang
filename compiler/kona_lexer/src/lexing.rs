// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use crate::source_iter::SourceIter;
use crate::char_spec::*;
use crate::token::{IdentKind, LitKind};
use crate::token::{Token, TokenKind::{self, *}};

impl SourceIter<'_> {
    fn lex_token(&mut self) -> Token {
        let kind = match self.peek_fst() {
            // Line comment, block comment, or symbolic identifier start
            // with '/'.
            '/' => match self.peek_snd() {
                '/' => self.lex_line_comment(),
                '-' => self.lex_block_comment(),
                _ => self.lex_sym_ident(),
            },

            // Whitespace sequence.
            c if is_whitespace(c) => self.lex_whitespace(),

            // Alphanumeric identifier or keyword.
            c if is_alpha_ident_head(c) => self.lex_alpha_ident(),

            // Symbolic identifier or keyword.
            c if is_sym_ident(c) => self.lex_sym_ident(),

            // Integer literal and float literal.
            '0'..='9' => self.lex_number(),

            // String literal.
            '"' => self.lex_string(),

            ';' => Semi,
            '(' => LParen,
            ')' => RParen,

            _ => unimplemented!(),
        };
        unimplemented!()
    }

    fn lex_line_comment(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '/' && self.eat() == '/');
        self.eat_while(|c| c != '\n');
        LineComment
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
                '*' if self.peek_fst() == '-' => {
                    self.eat();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }

        BlockComment
    }

    fn lex_whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.eat()));

        self.eat_while(is_whitespace);
        Whitespace
    }

    fn lex_alpha_ident(&mut self) -> TokenKind {
        debug_assert!(is_alpha_ident_head(self.eat()));
        self.eat_while(is_alpha_ident_body);
        Ident(IdentKind::Alphanumeric)
    }

    fn lex_sym_ident(&mut self) -> TokenKind {
        debug_assert!(is_sym_ident(self.eat()));
        self.eat_while(is_sym_ident);
        Ident(IdentKind::Symbolic)
    }

    fn lex_number(&mut self) -> TokenKind {
        debug_assert!(matches!(self.eat(), '0'..='9'));

        self.eat_while(is_digit);

        if self.peek_fst() == '.' && is_digit(self.peek_snd()) {
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
                '"' => break,
                '\\' if self.peek_fst() == '\\' && self.peek_fst() == '"' => {
                    self.eat();
                }
                _ => (),
            }
        }

        Lit(LitKind::String)
    }
}
