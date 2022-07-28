// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Ident,
    Op,
    Lit(LitKind),
    Keyword(KeywordKind),

    DArrow,
    Eq,

    Semi,
    LParen,
    RParen,

    Trivia(TriviaKind),
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordKind {
    Else,
    Fn,
    If,
    In,
    Infix,
    Let,
    Then,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LitKind {
    Int,
    Float,
    String { terminated: bool },
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriviaKind {
    Whitespace,
    Eol,
    SingleLineComment,
    MultiLineComment { terminated: bool },
}
