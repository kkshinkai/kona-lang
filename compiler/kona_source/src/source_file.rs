// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{path::PathBuf, rc::Rc, ops::Range};

use crate::{pos::Pos, source_analyzer};

/// Represents a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    /// The path to the source file.
    path: FilePath,

    /// The source code in the file.
    pub src: Rc<String>,

    /// The start position of this source in the file.
    ///
    /// Each file is assigned a unique index range, see [`Pos`] for details.
    pub start_pos: Pos,

    /// The end position of this source in the file.
    pub end_pos: Pos,

    /// Caches the start of each line in the source file.
    ///
    /// Line breaks include carriage return (CR, `\r`), line feed (LF, `\n`), and
    /// carriage return followed by line feed (CRLF, `\r\n`). These three kinds
    /// of line breaks can be mixed in the same file (although this is a bad
    /// practice).
    lines: Vec<Pos>,

    /// Caches the position of all multi-byte characters in the source file.
    ///
    /// TODO: More explanation for UTF-8 encoding.
    multi_byte_chars: Vec<MultiByteChar>,

    /// Caches the position of characters that are not narrow in the source
    /// file.
    ///
    /// This property may be used when printing source code and error messages
    /// in the terminal. See also Unicode Standard Annex #11 [East Asian Width].
    ///
    /// [East Asian Width]: https://www.unicode.org/reports/tr11/
    non_narrow_chars: Vec<NonNarrowChar>,
}

impl SourceFile {
    pub fn path(&self) -> &FilePath {
        &self.path
    }
}

impl SourceFile {
    /// Creates a new source file from the given path and source code.
    pub fn new(path: FilePath, src: Rc<String>, start_pos: Pos) -> SourceFile {
        let end_pos = start_pos + src.len();
        let (lines, multi_byte_chars, non_narrow_chars) =
            source_analyzer::analyze(&src, start_pos);
        SourceFile {
            src, path, start_pos, end_pos, lines,
            multi_byte_chars, non_narrow_chars,
        }
    }

    /// Finds the line containing the given position.
    ///
    /// The return value is the index into the `lines` array of this
    /// `SourceFile`, not the 1-based line number. If the source file is empty
    /// or the position is located before the first line, `None` is returned.
    pub fn lookup_line(&self, pos: Pos) -> Option<usize> {
        match self.lines.binary_search(&pos) {
            Ok(index) => Some(index),
            Err(0) => None,
            Err(index) => Some(index - 1),
        }
    }

    pub fn lookup_line_bounds(&self, line_index: usize) -> Range<Pos> {
        if self.is_empty() {
            return self.start_pos..self.end_pos;
        }

        assert!(line_index < self.lines.len());
        if line_index == (self.lines.len() - 1) {
            self.lines[line_index]..self.end_pos
        } else {
            self.lines[line_index]..self.lines[line_index + 1]
        }
    }


    /// Looks up the file's 1-based line number and 0-based column offset, for a
    /// given [`Pos`].
    pub fn lookup_line_and_col(&self, pos: Pos) -> (usize, usize) {
        if let Some(line) = self.lookup_line(pos) {
            let line_start = self.lines[line];
            let col = {
                let linebpos = self.lines[line];
                let start_idx = self.multi_byte_chars
                    .binary_search_by_key(&linebpos, |x| x.pos())
                    .unwrap_or_else(|x| x);
                let extra_byte = self
                    .multi_byte_chars[start_idx..]
                    .iter()
                    .take_while(|x| x.pos() < pos)
                    .map(|x| x.len() as usize - 1)
                    .sum::<usize>();
                pos.to_usize() - line_start.to_usize() - extra_byte
            };
            (line + 1, col)
        } else {
            (0, 0)
        }
    }

    pub fn lookup_line_col_and_col_display(
        &self, pos: Pos
    ) -> (usize, usize, usize) {
        let (line, col) = self.lookup_line_and_col(pos);
        let col_display = {
            let linebpos = self.lines[line - 1];
            let start_idx = self
                .non_narrow_chars
                .binary_search_by_key(&linebpos, |x| x.pos())
                .unwrap_or_else(|x| x);
            let non_narrow = self
                .non_narrow_chars[start_idx..]
                .iter()
                .take_while(|x| x.pos() < pos);
            let width = non_narrow.clone()
                .map(|x| x.width())
                .sum::<usize>();
            let count = non_narrow.count();
            col + width - count
        };
        (line, col, col_display)
    }

    #[inline]
    pub fn contains(&self, pos: Pos) -> bool {
        pos >= self.start_pos && pos <= self.end_pos
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start_pos == self.end_pos
    }
}


/// Represents a path to a source file.
///
/// The file may be virtual, or it may not exist. We don't check these when
/// creating a new [`FilePath`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FilePath {
    /// The path to a local file.
    File(PathBuf),

    /// Temporary file that holds the code in REPL.
    ///
    /// Kona does not have an interpretation mode, the REPL input will be placed
    /// in a temporary file and compiled like a common source file.
    Repl(PathBuf),

    /// A dummy file with given name, mostly for testing.
    Virtual(String),
}

/// Represents a multi-byte UTF-8 unicode scalar in the source code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiByteChar {
    pos: Pos,

    /// The number of bytes in the UTF-8 encoding of the character. It could
    /// only be 2, 3 or 4.
    len: u8,
}

impl MultiByteChar {
    /// Creates a new [`MultiByteChar`] from [`Pos`] and its length.
    #[inline]
    pub fn new(pos: Pos, len: u8) -> Self {
        MultiByteChar { pos, len }
    }

    /// Returns the UTF-8 length of this character.
    #[inline]
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Returns the [`Pos`] of this character.
    #[inline]
    pub fn pos(&self) -> Pos {
        self.pos
    }
}

/// Represents a non-narrow character in the source code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonNarrowChar {
    pos: Pos,
    kind: NonNarrowCharKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NonNarrowCharKind {
    /// A zero-width character.
    ZeroWidth,

    /// A full-width character.
    Wide,

    /// A tab, treated as four spaces.
    Tab,
}

impl NonNarrowChar {
    /// Creates a new [`NonNarrowChar`] from [`Pos`] and its east asian
    /// width.
    pub fn new(pos: Pos, width: usize) -> Self {
        let kind = match width {
            0 => NonNarrowCharKind::ZeroWidth,
            2 => NonNarrowCharKind::Wide,
            _ => NonNarrowCharKind::Tab,
        };
        NonNarrowChar { pos, kind }
    }

    /// Returns the position of this character.
    #[inline]
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns the width of this character.
    pub fn width(&self) -> usize {
        match self.kind {
            NonNarrowCharKind::ZeroWidth => 0,
            NonNarrowCharKind::Wide => 2,
            NonNarrowCharKind::Tab => 4,
        }
    }
}
