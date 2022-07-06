// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use crate::{ast::token::{Token, TokenKind, Lit, Ident, Keyword}, span_legacy::span::Span};

use super::lookahead::{AheadLooker, Lookahead};

struct SourceReader {
    src: String,
    pos: usize,
}

impl SourceReader {
    fn new(src: String) -> SourceReader {
        SourceReader { src, pos: 0 }
    }
}

impl Iterator for SourceReader {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        (self.pos < self.src.bytes().len()).then(|| {
            let char = self.src[self.pos..].chars().next().unwrap();
            self.pos += char.len_utf8();
            char
        })
    }
}

pub struct Lexer {
    src_reader: AheadLooker<SourceReader>,
}

impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer { src_reader: SourceReader::new(src).ahead_looker() }
    }

    fn peek(&mut self) -> Option<char> {
        self.src_reader.peek().cloned()
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        self.src_reader.peek_nth(n).cloned()
    }

    fn eat(&mut self) -> char {
        assert!(self.peek().is_some());
        self.src_reader.next().unwrap()
    }

    fn pos(&self) -> usize {
        self.src_reader.iter.pos
    }
}

impl Lexer {
    fn lex_next(&mut self) -> Option<Token> {
        // Skip whitespaces.
        while matches!(self.peek(), Some(next) if next.is_whitespace()) {
            self.eat();
        }

        // Skip comments, '//' or '/-'
        while self.peek() == Some('/') {
            match self.peek_nth(1) {
                Some('/') => {
                    self.eat();
                    self.eat();
                    while self.peek() != Some('\n') {
                        self.eat();
                    }
                    self.eat(); // Eat '\n'.
                }
                Some('-') => {
                    self.eat();
                    self.eat();
                    while !(self.peek() == Some('-') &&
                            self.peek_nth(1) == Some('/'))
                    {
                        if self.peek().is_none() {
                            panic!("unterminated comment");
                        }
                        self.eat();
                    }
                }
                _ => break,
            }
        }


        self.peek().map(|next| {
            match next {
                '(' | ')' | ';' => {
                    let start = self.pos();
                    self.eat();
                    let end = self.pos();

                    let kind = match self.eat() {
                        '(' => TokenKind::LParen,
                        ')' => TokenKind::RParen,
                        ';' => TokenKind::Semi,
                        _ => unreachable!(),
                    };

                    Token { kind, span: Span::new(start, end) }
                }
                '0'..='9' => self.lex_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.lex_alpha_ident(),
                '!' | '%' | '&' | '$' | '+' | '-' | ':' | '<' | '=' | '>'
                | '?' | '/' | '~' | '^' | '|' | '*' => self.lex_sym_ident(),
                '"' => self.lex_string(),
                _ => panic!("unexpected char: {}", next),
            }
        })
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos();
        let mut text = String::new();

        while matches!(self.peek(), Some('0'..='9')) {
            text.push(self.eat());
        }

        if self.peek() != Some('.') {
            let kind = TokenKind::Lit(Lit::Int(text.parse().unwrap()));
            return Token { kind, span: Span::new(start, self.pos()) };
        }

        text.push(self.eat()); // Eat '.'.

        while matches!(self.peek(), Some('0'..='9')) {
            text.push(self.eat());
        }

        let kind = TokenKind::Lit(Lit::Float(text.parse().unwrap()));
        Token { kind, span: Span::new(start, self.pos()) }
    }

    fn lex_alpha_ident(&mut self) -> Token {
        let start = self.pos();
        let mut text = String::new();

        while matches!(self.peek(), Some('a'..='z' | 'A'..='Z' | '_')) {
            text.push(self.eat());
        }

        let kind = match &text[..] {
            "=" => TokenKind::Keyword(Keyword::Eq),
            "=>" => TokenKind::Keyword(Keyword::DArrow),
            _ => TokenKind::Ident(Ident::new(text)),
        };
        Token { kind, span: Span::new(start, self.pos()) }
    }

    fn lex_sym_ident(&mut self) -> Token {
        let start = self.pos();
        let mut text = String::new();

        while matches!(self.peek(),
                       Some('!' | '%' | '&' | '$' | '+' | '-' | ':' | '<' | '='
                           | '>' | '?' | '/' | '~' | '^' | '|' | '*'))
        {
            text.push(self.eat());
        }

        let kind = match &text[..] {
            "else" => TokenKind::Keyword(Keyword::Else),
            "end" => TokenKind::Keyword(Keyword::End),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "if" => TokenKind::Keyword(Keyword::If),
            "in" => TokenKind::Keyword(Keyword::In),
            "let" => TokenKind::Keyword(Keyword::Let),
            "op" => TokenKind::Keyword(Keyword::Op),
            "then" => TokenKind::Keyword(Keyword::Then),
            "val" => TokenKind::Keyword(Keyword::Val),
            "true" => TokenKind::Lit(Lit::Bool(true)),
            "false" => TokenKind::Lit(Lit::Bool(false)),
            _ => TokenKind::Ident(Ident::new(text)),
        };
        Token { kind, span: Span::new(start, self.pos()) }
    }

    fn lex_string(&mut self) -> Token {
        let start = self.pos();
        self.eat(); // Eat '"'.

        let mut text = String::new();
        while self.peek() != Some('"') {
            if self.peek().is_none() {
                panic!("unterminated string");
            }

            if matches!(self.peek(), Some('\r' | '\n')) {
                panic!("newline is not allowed in string literals");
            }

            if self.peek() == Some('\\') {
                self.eat(); // Eat '\'.
                match self.peek() {
                    Some('n') => text.push('\n'),
                    Some('r') => text.push('\r'),
                    Some('t') => text.push('\t'),
                    Some('\\') => text.push('\\'),
                    Some('"') => text.push('"'),
                    Some('\r' | '\n') => {
                        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
                            self.eat();
                        }
                        if self.peek() == Some('\\') {
                            self.eat();
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!("unexpected escape sequence"),
                }
            }
        }

        self.eat(); // Eat '"'.

        let kind = TokenKind::Lit(Lit::String(text));
        Token { kind, span: Span::new(start, self.pos()) }
    }
}
