// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::rc::Rc;

use kona_source::{source_map::SourceMap, span::Span};

pub struct DiagnosticsEngine {
    source_map: Rc<SourceMap>,
}

impl DiagnosticsEngine {
    pub fn new(source_map: Rc<SourceMap>) -> DiagnosticsEngine {
        DiagnosticsEngine { source_map }
    }

    pub fn report_err(&self, span: Span, message: String) {
        let info = self.source_map.lookup_pos_info(span.start);
        println!("error:  {}", message);
        println!("   {}:{}:{}", info.file.file_name(), info.line, info.col);
    }
}
