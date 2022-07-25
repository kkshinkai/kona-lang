// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use kona_memory::intern::symbol::Symbol;
use kona_source::span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    /// Creates a new identifier with the given symbol and span.
    #[inline]
    pub fn new(name: Symbol, span: Span) -> Ident {
        Ident { name, span }
    }

    /// Creates a new identifier with dummy span.
    #[inline]
    pub fn with_dummy_span(name: Symbol) -> Ident {
        Ident::new(name, Span::dummy())
    }

    /// Creates a new identifier with the given string and span, the string will
    /// be interned.
    pub fn from_str(name: &str, span: Span) -> Ident {
        Ident::new(Symbol::intern(name), span)
    }

    /// Creates a new identifier with the given string and dummy span, the
    /// string will be interned.
    pub fn from_str_with_dummy_span(name: &str) -> Ident {
        Ident::with_dummy_span(Symbol::intern(name))
    }
}
