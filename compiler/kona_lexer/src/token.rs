// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

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
    Ident(IdentKind),
    Lit(LitKind),

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

    Semi,
    LParen,
    RParen,

    Trivia(TriviaKind),
    Invalid,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(self, TokenKind::DArrow | TokenKind::Eq | TokenKind::Else
            | TokenKind::End | TokenKind::Fn | TokenKind::If | TokenKind::In
            | TokenKind::Let | TokenKind::Op | TokenKind::Then | TokenKind::Val)
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, TokenKind::Ident(_))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, TokenKind::Lit(_))
    }

    pub fn is_string_lit(&self) -> bool {
        matches!(self, TokenKind::Lit(LitKind::String))
    }

    pub fn is_int_lit(&self) -> bool {
        matches!(self, TokenKind::Lit(LitKind::Int))
    }

    pub fn is_float_lit(&self) -> bool {
        matches!(self, TokenKind::Lit(LitKind::Float))
    }

    pub fn is_bool_lit(&self) -> bool {
        matches!(self, TokenKind::Lit(LitKind::Bool))
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, TokenKind::Trivia(_))
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::Trivia(TriviaKind::Whitespace))
    }

    pub fn is_eol(&self) -> bool {
        matches!(self, TokenKind::Trivia(TriviaKind::Eol))
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, TokenKind::Trivia(TriviaKind::Comment(_)))
    }

    pub fn is_punct(&self) -> bool {
        matches!(self, TokenKind::LParen | TokenKind::RParen | TokenKind::Semi)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TokenKind::Ident(_) => "ident",
            TokenKind::Lit(lit_kind) => match lit_kind {
                LitKind::Int => "int",
                LitKind::Float => "float",
                LitKind::String => "string",
                LitKind::Bool => "bool",
            }
            TokenKind::DArrow => "double_arrow",
            TokenKind::Eq => "equal",
            TokenKind::Else => "else",
            TokenKind::End => "end",
            TokenKind::Fn => "fn",
            TokenKind::If => "if",
            TokenKind::In => "in",
            TokenKind::Let => "let",
            TokenKind::Op => "op",
            TokenKind::Then => "then",
            TokenKind::Val => "val",
            TokenKind::Semi => "semi",
            TokenKind::LParen => "l_paren",
            TokenKind::RParen => "r_paren",
            TokenKind::Trivia(trivia_kind) => match trivia_kind {
                TriviaKind::Whitespace => "whitespace",
                TriviaKind::Eol => "eol",
                TriviaKind::Comment(comment_kind) => {
                    match comment_kind {
                        CommentKind::SingleLine => "line_comment",
                        CommentKind::MultiLine => "block_comment",
                    }
                }
            }
            Self::Invalid => "invalid",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LitKind {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentKind {
    Alphanumeric,
    Symbolic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriviaKind {
    Whitespace,
    Eol,
    Comment(CommentKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommentKind {
    SingleLine,
    MultiLine,
}
