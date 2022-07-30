// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

/// Represents a position in source code (a branch of source files in a source
/// map).
///
/// The source code manager assigns each file a contiguous interval [start, end)
/// that does not intersect with each other. The `usize` number in `Pos` is the
/// [`start_pos`] of that file adding the offset (aka. the index of that byte
/// in source file). With this `Pos`, you can find all information you need in
/// the [`SourceMap`], the file name, the source character, the line and column
/// numbers...
///
/// Here is a simple example:
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
/// You can also get a sub-string of the source code by using [`Interval`],
/// a range of `Pos`.
///
/// TODO: Improve this document, we need a more understandable explanation.
///
/// [`start_pos`]: crate::source_file::SourceFile::start_pos
/// [`SourceMap`]: crate::source_map::SourceMap
/// [`Interval`]: crate::interval::Interval
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

impl AddAssign<usize> for Pos {
    fn add_assign(&mut self, offset: usize) {
        self.index += offset;
    }
}

impl Sub<usize> for Pos {
    type Output = Pos;

    #[inline(always)]
    fn sub(self, offset: usize) -> Pos {
        Pos::from_usize(self.index - offset)
    }
}

impl SubAssign<usize> for Pos {
    fn sub_assign(&mut self, offset: usize) {
        self.index -= offset;
    }
}
