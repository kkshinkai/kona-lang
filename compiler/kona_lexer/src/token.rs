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
    Ident(IdentKind),
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
    Int,
    Float,
    String,
    Bool,
}

pub enum IdentKind {
    Alphanumeric,
    Symbolic,
}
