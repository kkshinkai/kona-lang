// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::ops::Range;
use kona_source::pos::Pos;

pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub span: Range<Pos>,
}

pub enum Level {
    Error,
    Warning,
    Info,
}
