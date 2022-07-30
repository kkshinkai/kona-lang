// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_memory::intern::symbol::Symbol;
use kona_source::span::Span;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
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

    Ident(Symbol),
    Op(Symbol),
    Lit(LitKind),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LitKind {
    Int,
    Float,
    Bool,
    String,
}
