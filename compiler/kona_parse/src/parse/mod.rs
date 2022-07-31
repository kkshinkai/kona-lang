// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::rc::Rc;

use kona_ast::expr::Expr;
use kona_diagnostic::DiagnosticsEngine;
use kona_source::{source_map::{self, SourceMap}, source_file::SourceFile};

use crate::lex::token_iter::TokenIter;

pub struct Parser<'src> {
    tokens: TokenIter<'src>,
    diag: DiagnosticsEngine,
}

impl<'src> Parser<'src> {
    pub fn new(source_file: &'src Rc<SourceFile>, source_map: Rc<SourceMap>) -> Parser<'src> {
        // FIXME: I really hate `&'a Rc<T>`, but I haven't figured out how to
        // fix it.
        let tokens = TokenIter::new(&source_file.src, source_file.start_pos);
        let diag = DiagnosticsEngine::new(source_map);
        Parser { tokens, diag }
    }

    fn parse_expr() -> Expr {
        todo!()
    }
}
