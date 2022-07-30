// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_memory::intern::symbol::Symbol;
use kona_source::span::Span;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Trivia {
    pub kind: TriviaKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TriviaKind {
    SingleLineComment,
    MultiLineComment,
    Whitespace,
    Eol,
}
