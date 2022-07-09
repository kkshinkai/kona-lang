// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

pub enum TokenKind {
    Keyword(Keyword),
    Ident(Ident),
    Lit(LitKind),

    Semi,
    LParen,
    RParen,

    Whitespace,
    LineComment,
    BlockComment,

    Invalid,
}

pub enum LitKind {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

pub enum Keyword {
    Else,
    End,
    Fn,
    If,
    In,
    Let,
    Op,
    Then,
    Val,
    Eq,
    DArrow,
}

pub struct Ident {
    name: String,
    kind: IdentKind,
}

pub enum IdentKind {
    Alphanumeric,
    Symbolic,
}
