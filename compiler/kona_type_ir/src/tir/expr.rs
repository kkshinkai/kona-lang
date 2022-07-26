// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_ast::token::ident::Ident;
use kona_source::span::Span;

pub struct Expr<'tir> {
    pub kind: ExprKind<'tir>,
    pub span: Span,
}

pub enum ExprKind<'tir> {
    // Zero-arity tuple, remove this when we have tuples.
    Unit,
    Lit(/* Lit */),
    Lambda(Vec<Ident>, &'tir Expr<'tir>),

    Call(&'tir Expr<'tir>, &'tir Expr<'tir>),
    InfixCall(Ident, &'tir Expr<'tir>, &'tir Expr<'tir>),

    Let(Ident, &'tir Expr<'tir>, &'tir Expr<'tir>),
    If(&'tir Expr<'tir>, &'tir Expr<'tir>, &'tir Expr<'tir>),
    Block(&'tir Expr<'tir>),
}
