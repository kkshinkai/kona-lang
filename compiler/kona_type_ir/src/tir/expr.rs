// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_ast::ident::Ident;
use kona_source::span::Span;

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    // Zero-arity tuple, remove this when we have tuples.
    Unit,
    Call(Vec<Box<Expr>>, Box<Expr>),
    InfixCall(Ident, Box<Expr>, Box<Expr>),
    Lit(/* Lit */),
    Let(Ident, Box<Expr>, Box<Expr>),
    Lambda(Vec<Ident>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
}
