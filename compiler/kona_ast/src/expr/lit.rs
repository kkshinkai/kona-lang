// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_source::span::Span;

pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

pub enum LitKind {
    Int,
    Float,
    String,
    Char,
    Bool,
}
