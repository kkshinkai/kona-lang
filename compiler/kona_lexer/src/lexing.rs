// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use crate::source_iter::SourceIter;
use crate::char_spec::*;
use crate::token::{IdentKind, LitKind, KeywordKind, TriviaKind, CommentKind};
use crate::token::{Token, TokenKind::{self, *}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexMode {
    #[deprecated]
    TokenOnly, // This mode is broken, remove it
    TokenAndTrivia,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str, lex_mode: LexMode) -> impl Iterator<Item = Token> + '_ {
    let mut iter = SourceIter::new(input);
    std::iter::from_fn(move || {
        while !iter.is_eof() {
            iter.reset_consumed_len();
            let next_token = iter.lex_token();

            if lex_mode == LexMode::TokenOnly
                && matches!(next_token.kind, TokenKind::Trivia(_))
            {
                continue;
            } else {
                return Some(next_token);
            }
        }

        None
    })
}

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

            ';' => { self.eat(); Semi }
            '(' => { self.eat(); LParen }
            ')' => { self.eat(); RParen }

            _ => { self.eat(); Invalid }
        };
        Token::new(kind, self.consumed_len())
    }

    fn lex_line_comment(&mut self) -> TokenKind {
        debug_assert!(self.eat() == '/' && self.eat() == '/');
        self.eat_while(|c| c != '\n');
        Trivia(TriviaKind::Comment(CommentKind::Line))
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

        Trivia(TriviaKind::Comment(CommentKind::Block))
    }

    fn lex_whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.eat()));

        self.eat_while(is_whitespace);
        Trivia(TriviaKind::Whitespace)
    }

    fn lex_alpha_ident(&mut self) -> TokenKind {
        debug_assert!(is_alpha_ident_head(self.peek_fst()));

        // Lex keywords, I'm not sure if this is a good implementation, but for
        // now this is all I can do.
        if self.eat_if_is('e') {
            if self.eat_if_is('l') {
                if self.eat_if_is('s') {
                    if self.eat_if_is('e') {
                        if !is_alpha_ident_body(self.peek_fst()) {
                            return Keyword(KeywordKind::Else);
                        }
                    }
                }
            } else if self.eat_if_is('n') {
                if self.eat_if_is('d') {
                    if !is_alpha_ident_body(self.peek_fst()) {
                        return Keyword(KeywordKind::End);
                    }
                }
            }
        } else if self.eat_if_is('f') {
            if self.eat_if_is('n') {
                if !is_alpha_ident_body(self.peek_fst()) {
                    return Keyword(KeywordKind::Fn);
                }
            }
        } else if self.eat_if_is('i') {
            if self.eat_if_is('f') {
                if !is_alpha_ident_body(self.peek_fst()) {
                    return Keyword(KeywordKind::If);
                }
            }
        } else if self.eat_if_is('l') {
            if self.eat_if_is('e') {
                if self.eat_if_is('t') {
                    if !is_alpha_ident_body(self.peek_fst()) {
                        return Keyword(KeywordKind::Let);
                    }
                }
            }
        } else if self.eat_if_is('o') {
            if self.eat_if_is('p') {
                if !is_alpha_ident_body(self.peek_fst()) {
                    return Keyword(KeywordKind::Op);
                }
            }
        } else if self.eat_if_is('t') {
            if self.eat_if_is('h') {
                if self.eat_if_is('e') {
                    if !is_alpha_ident_body(self.peek_fst()) {
                        return Keyword(KeywordKind::Then);
                    }
                }
            }
        } else if self.eat_if_is('v') {
            if self.eat_if_is('a') {
                if self.eat_if_is('l') {
                    if !is_alpha_ident_body(self.peek_fst()) {
                        return Keyword(KeywordKind::Val);
                    }
                }
            }
        }


        self.eat_while(is_alpha_ident_body);
        Ident(IdentKind::Alphanumeric)
    }

    fn lex_sym_ident(&mut self) -> TokenKind {
        debug_assert!(is_sym_ident(self.peek_fst()));

        if self.eat_if_is('=') {
            if self.eat_if_is('>') {
                if !is_sym_ident(self.peek_fst()) {
                    return Keyword(KeywordKind::DArrow);
                }
            } else {
                if !is_sym_ident(self.peek_fst()) {
                    return Keyword(KeywordKind::Eq);
                }
            }
        }

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
