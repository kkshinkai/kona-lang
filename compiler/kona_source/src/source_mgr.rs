// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{rc::Rc, collections::HashMap};

use crate::source_file::{SourceFile, FilePath};

#[derive(Debug, Clone, PartialEq)]
pub struct SourceMgr {
    /// The used position index, for allocating individual position intervals to
    /// source files.
    used_pos_space: usize,

    // WARNING: Don't modify `used_pos_space` directly. Don't add new functions
    // that might modify or access it. `allocate_new_interval` should be the
    // only function that can increase and get `used_pos_space`.

    /// The source files.
    files: Vec<Rc<SourceFile>>,

    /// The source files hash map.
    files_map: HashMap<FilePath, Rc<SourceFile>>,
}

impl SourceMgr {
    pub fn new() -> Self {
        SourceMgr {
            used_pos_space: 0,
            files: Vec::new(),
            files_map: HashMap::new(),
        }
    }

    fn allocate_new_interval(&mut self, size: usize) -> usize {
        let pos = self.used_pos_space;
        self.used_pos_space += size;
        pos
    }
}
