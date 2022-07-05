// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

use crate::span::span::Span;

#[derive(PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.span)
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn test_token_debug() {
        let dummy_span = Span::new(0, 0);

        let token = Token::new(TokenKind::Keyword(Keyword::Let), dummy_span);
        assert_eq!(format!("{token:?}"), "let 0..0");

        let token = Token::new(
            TokenKind::Ident(Ident::new("foo".to_string())),
            dummy_span,
        );
        assert_eq!(format!("{token:?}"), "ident 'foo' 0..0");

        let token = Token::new(TokenKind::LParen, dummy_span);
        assert_eq!(format!("{token:?}"), "lparen 0..0");

        let token = Token::new(TokenKind::Lit(Lit::Float(3.14)), dummy_span);
        assert_eq!(format!("{token:?}"), "float 3.14 0..0");

        let token = Token::new(
            TokenKind::Lit(Lit::String("\"hello\n\"".to_string())),
            dummy_span,
        );
        assert_eq!(format!("{token:?}"), r#"string "\"hello\n\"" 0..0"#);
    }
}

#[derive(PartialEq, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Ident(Ident),
    Lit(Lit),
    Semi,
    LParen,
    RParen,
}

impl fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword(kw) => write!(f, "{:?}", kw),
            TokenKind::Ident(ident) => write!(f, "ident '{}'", ident.name),
            TokenKind::Lit(lit) => write!(f, "{:?}", lit),
            TokenKind::Semi => write!(f, "semi"),
            TokenKind::LParen => write!(f, "lparen"),
            TokenKind::RParen => write!(f, "rparen"),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Keyword {
    Else,
    End,
    Fn,
    If,
    In,
    Let,
    Then,
    Val,
    Eq,
    DArrow,
}

impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Keyword::Else => "else",
            Keyword::End => "end",
            Keyword::Fn => "fn",
            Keyword::If => "if",
            Keyword::In => "in",
            Keyword::Let => "let",
            Keyword::Then => "then",
            Keyword::Val => "val",
            Keyword::Eq => "eq",
            Keyword::DArrow => "darrow",
        })
    }
}

#[derive(PartialEq, Clone)]
pub enum Lit {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "int {}", i),
            Lit::Float(n) => write!(f, "float {}", n),
            Lit::String(str) => write!(f, "string {:?}", str),
            Lit::Bool(b) => write!(f, "bool {}", b),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Ident {
    pub name: String,
}

impl Ident {
    #[inline]
    pub fn new(name: String) -> Ident {
        Ident { name }
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
