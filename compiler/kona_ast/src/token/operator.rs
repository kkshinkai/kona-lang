// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_memory::intern::symbol::Symbol;
use kona_source::span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Operator {
    pub name: Symbol,
    pub span: Span,
}

impl Operator {
    #[inline]
    pub fn new(name: Symbol, span: Span) -> Operator {
        Operator { name, span }
    }

    #[inline]
    pub fn with_dummy_span(name: Symbol) -> Operator {
        Operator::new(name, Span::dummy())
    }

    pub fn from_str(name: &str, span: Span) -> Operator {
        Operator::new(Symbol::intern(name), span)
    }

    pub fn from_str_with_dummy_span(name: &str) -> Operator {
        Operator::with_dummy_span(Symbol::intern(name))
    }
}
