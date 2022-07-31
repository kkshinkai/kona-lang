// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::rc::Rc;

use kona_source::source_map::SourceMap;

pub struct DiagnosticsEngine {
    source_map: Rc<SourceMap>,
}
