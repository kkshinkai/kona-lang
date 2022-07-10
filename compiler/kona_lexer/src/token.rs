// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

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
    Keyword(KeywordKind),

    Semi,
    LParen,
    RParen,

    Whitespace,
    LineComment,
    BlockComment,

    Invalid,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Ident(_) => "ident",
            Self::Lit(lit_kind) => match lit_kind {
                LitKind::Int => "int",
                LitKind::Float => "float",
                LitKind::String => "string",
                LitKind::Bool => "bool",
            }
            Self::Keyword(kw_kind) => match kw_kind {
                KeywordKind::DArrow => "->",
                KeywordKind::Eq => "=",
                KeywordKind::Else => "else",
                KeywordKind::End => "end",
                KeywordKind::Fn => "fn",
                KeywordKind::If => "if",
                KeywordKind::In => "in",
                KeywordKind::Let => "let",
                KeywordKind::Op => "op",
                KeywordKind::Then => "then",
                KeywordKind::Val => "val",
            }
            Self::Semi => "semi",
            Self::LParen => "l_paren",
            Self::RParen => "r_paren",
            Self::Whitespace => "whitespace",
            Self::LineComment => "line_comment",
            Self::BlockComment => "block_comment",
            Self::Invalid => "invalid",
        })
    }
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

pub enum KeywordKind {
    DArrow,
    Eq,
    Else,
    End,
    Fn,
    If,
    In,
    Let,
    Op,
    Then,
    Val,
}
