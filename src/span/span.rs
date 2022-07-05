// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new code span `[start, end)` from then start and end index.
    #[inline]
    pub fn new(start: usize, end: usize) -> Span {
        // TBD: For now, `start == end` is used to represent a dummy span, which
        // is now only used in tests. Maybe we should deprecate this behavior
        // later.
        assert!(start <= end);

        Span { start, end }
    }

    /// Create a dummy code span. For testing purposes only.
    #[inline]
    pub fn dummy() -> Span {
        Span::new(0, 0)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
