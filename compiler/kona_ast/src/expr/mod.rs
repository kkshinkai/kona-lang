// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_source::span::Span;

use self::{lit::Lit, operator::Operator, ident::Ident};

pub mod ident;
pub mod operator;
pub mod lit;

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    Lit(Lit),
    PendingSeq(Vec<Expr>),
    InfixOp(Operator, Box<Expr>, Box<Expr>),
    Fn(Ident, Box<Expr>),
    FnCall(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Let(Ident, Box<Expr>, Box<Expr>),
}
