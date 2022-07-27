// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{rc::Rc, collections::HashMap, path::PathBuf, io, fs, ops::Range};

use crate::{source_file::{SourceFile, FilePath}, pos::Pos, pos_info::PosInfo};

/// Source map for a compilation unit, including a bunch of source files, source
/// code, and position information.
///
/// [`SourceMap`] is the top-level interface for source code management. It
/// manages a collection of [`SourceFile`]s and the source code within them.
/// [`SourceMap`] provides a position assignment mechanism that allocates a
/// unique position [`Pos`] for each byte in the source code. You can get a
/// human-readable information [`PosInfo`] with a [`Pos`], or read a span of
/// source code with a [`Range<Pos>`] in source map.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceMap {
    /// The used position index, for allocating individual position intervals to
    /// source files.
    used_pos_space: usize,

    // WARNING: Don't modify `used_pos_space` directly. Don't add new functions
    // that might modify or access it. `allocate_pos_space` should be the
    // only function that can increase `used_pos_space`.

    /// The source files.
    files: Vec<Rc<SourceFile>>,

    /// The source files hash map.
    files_map: HashMap<FilePath, Rc<SourceFile>>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            used_pos_space: 0,
            files: Vec::new(),
            files_map: HashMap::new(),
        }
    }

    fn allocate_pos_space(&mut self, size: usize) -> usize {
        let pos = self.used_pos_space;
        self.used_pos_space += size;
        pos
    }

    /// Loads source file from the given path.
    pub fn load_file(
        &mut self, path: PathBuf,
    ) -> io::Result<Rc<SourceFile>> {
        // Path must be absolute to uniquely identify the source file.
        let file_path = FilePath::LocalFile(fs::canonicalize(&path)?);
        if let Some(sf) = self.files_map.get(&file_path) {
            return Ok(sf.clone());
        }

        let src = fs::read_to_string(&path)?;
        let start_pos = Pos::from_usize(self.allocate_pos_space(src.len()));
        let file = Rc::new(
            SourceFile::new(file_path.clone(), Rc::new(src), start_pos)
        );
        self.files.push(file.clone());
        self.files_map.insert(file_path, file.clone());
        Ok(file)
    }

    /// Adds a virtual source file with the given name and source string.
    pub fn load_virtual_file(
        &mut self, name: String, src: String
    ) -> Rc<SourceFile> {
        let path = FilePath::Virtual(name);
        if let Some(sf) = self.files_map.get(&path) {
            return sf.clone();
        }

        let start_pos = Pos::from_usize(self.allocate_pos_space(src.len()));
        let file = Rc::new(SourceFile::new(
            path.clone(),
            Rc::new(src),
            start_pos,
        ));
        self.files.push(file.clone());
        self.files_map.insert(path, file.clone());
        file
    }

    pub fn lookup_pos_info(&self, pos: Pos) -> PosInfo {
        let sf = self.lookup_file(pos);
        let (line, col, col_display) = sf.lookup_line_col_and_col_display(pos);
        PosInfo::new(sf, line, col, col_display)
    }

    /// Finds the source file containing the given position.
    pub fn lookup_file(&self, pos: Pos) -> Rc<SourceFile> {
        let idx = self.files
            .binary_search_by_key(&pos, |file| file.start_pos)
            .unwrap_or_else(|p| p - 1);
        self.files[idx].clone()
    }

    /// Returns the source file at the given interval.
    pub fn lookup_source(&self, range: Range<Pos>) -> String {
        let file = self.lookup_file(range.start);

        let start = range.start.to_usize() - file.start_pos.to_usize();
        let end = range.end.to_usize() - file.start_pos.to_usize();

        file.src[start..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loc_single_file1() {
        let mut mgr = SourceMap::new();
        mgr.load_virtual_file(
            "example.scm".to_string(),
            "abcdefghi".to_string(),
        );
        let loc = mgr.lookup_pos_info(Pos::from_usize(5));

        assert_eq!(loc.line, 1);
        assert_eq!(loc.col, 5);
        assert_eq!(loc.col_display, 5);
    }

    #[test]
    fn test_loc_single_file2() {
        let mut mgr = SourceMap::new();
        mgr.load_virtual_file(
            "example".to_string(),
            "abc\ndef\nghi".to_string(),
        );
        let loc = mgr.lookup_pos_info(Pos::from_usize(5));

        assert_eq!(loc.line, 2);
        assert_eq!(loc.col, 1);
        assert_eq!(loc.col_display, 1);
    }

    #[test]
    fn test_loc_single_file3() {
        let mut mgr = SourceMap::new();
        mgr.load_virtual_file(
            "example".to_string(),
            "🌊🌊🌊\n🌊🌊🌊\n🌊🌊🌊".to_string(),
        );
        let loc = mgr.lookup_pos_info(Pos::from_usize(17));

        assert_eq!(loc.line, 2);
        assert_eq!(loc.col, 1);
        assert_eq!(loc.col_display, 2);
    }

    #[test]
    fn test_lookup_source() {
        let mut mgr = SourceMap::new();
        mgr.load_virtual_file(
            "example".to_string(),
            "abcdefghijklmn".to_string(),
        );

        let str = mgr.lookup_source(
            Pos::from_usize(3)..Pos::from_usize(7)
        );
        assert_eq!(str, "defg");
    }
}
