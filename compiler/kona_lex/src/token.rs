// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_source::span::Span;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    LParen,
    RParen,
    Semi,
    Eq,
    DArrow,

    Else,
    Fn,
    If,
    In,
    Infix,
    Let,
    Then,

    Ident,
    Op,
    Lit(LitKind),

    Trivia(TriviaKind),
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LitKind {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TriviaKind {
    SingleLineComment,
    MultiLineComment,
    Whitespace,
    Eol,
}
