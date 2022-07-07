// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;
use std::rc::Rc;

use crate::source_file::{SourceFile, FilePath};

/// Represents a set of human-readable position information, including the line,
/// column, file name, some other metadata of the source file.
///
/// You can get the [`PosInfo`] with a [`Pos`] in the source manager
/// [`SourceMgr`].
#[derive(Clone, PartialEq, Eq)]
pub struct PosInfo {
    /// Information about the original source.
    file: Rc<SourceFile>, // TBD: we don't need all these information, why not
                          // just `FilePath`?

    /// The 1-based line number.
    line: usize,

    /// The 0-based column offset.
    col: usize,

    /// The 0-based column offset when displayed.
    col_display: usize,
}

impl fmt::Debug for PosInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: This may be wrong, test it later.
        let file_name = match self.file.path() {
            FilePath::File(path) => format!("file:{path:?}"),
            FilePath::Repl(path) => format!("repl:{path:?}"),
            FilePath::Virtual(name) => name.clone(),
        };

        f.debug_struct("PosInfo")
            .field("file", &file_name)
            .field("line", &self.line)
            .field("column", &self.col)
            .finish()
    }
}
