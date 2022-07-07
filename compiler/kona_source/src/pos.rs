// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{ops::{Add, Sub}, fmt};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    index: usize,
}

impl Pos {
    #[inline(always)]
    pub fn from_usize(index: usize) -> Pos {
        Pos { index }
    }

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
//     add, sub: Fn (Pos, usize) -> Pos   // ✓
//     add, sub: Fn (Pos, Pos) -> Pos     // ✗

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
