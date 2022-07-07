// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{path::PathBuf, rc::Rc};

use crate::pos::Pos;

/// Represents a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    /// The path to the source file.
    path: FilePath,

    /// The source code in the file.
    src: Rc<String>,

    /// The start position of this source in the file.
    ///
    /// Each file is assigned a unique index range, see [`Pos`] for details.
    start_pos: Pos,

    /// The end position of this source in the file.
    end_pos: Pos,

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
