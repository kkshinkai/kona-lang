// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;
use std::ops::{Add, Sub};

/// An index for random access in all source code (a branch of source files) in
/// a source map.
///
/// TODO: Update the description.
///
/// [`Pos`] use an [`usize`] integer to represent the position of a byte. Each
/// source file is given a unique interval by the source code manager
/// [`SourceMap`]. [`Pos`] is therefore unique and can be used to pinpoint a
/// byte in multiple source files.
///
/// The source map keeps the starting [`Pos`] of each source file. The distance
/// between a [`Pos`] and the start of the file it is in, is its index in the
/// current source string.
///
/// For example:
///
/// ```text
///  idx1                idx2                    idx3            idx4
///   |------ file1 ------|-----x-- file2 --------|---- file3 ----|
///                             ^ pos
/// ```
///
/// We can infer that `pos` is in file2 because `idx2 <= pos < idx3`, the byte
/// at `pos` is then `file2.bytes[pos - idx2]`.
///
/// TODO: Improve this document, we need a more understandable explanation.
///
/// [`SourceMap`]: crate::source_map::SourceMap
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    index: usize,
}

impl Pos {
    /// Creates a new `Pos` with the given unique index.
    #[inline(always)]
    pub fn from_usize(index: usize) -> Pos {
        // NB. I didn't use the name `new` because I thought the name
        // `from_size` might make it clearer to the user that this is an
        // unverified forced conversion.
        Pos { index }
    }

    /// Returns the inner `usize` of the `Pos`.
    #[inline(always)]
    pub fn to_usize(self) -> usize {
        self.index
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos({})", self.index)
    }
}

// NOTE: Never implement `Add` and `Sub` for `Pos`. Adding two `Pos` together
// is meaningless, just like adding two raw pointers. Implement `Add<usize>`
// and `Sub<usize>` instead, here `usize` is considered as offset.
//
//     add, sub: Fn(Pos, usize) -> Pos   // ✓
//     add, sub: Fn(Pos, Pos) -> Pos     // ✗

impl Add<usize> for Pos {
    type Output = Pos;

    #[inline(always)]
    fn add(self, offset: usize) -> Pos {
        Pos::from_usize(self.index + offset)
    }
}

impl Sub<usize> for Pos {
    type Output = Pos;

    #[inline(always)]
    fn sub(self, offset: usize) -> Pos {
        Pos::from_usize(self.index - offset)
    }
}
