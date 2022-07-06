// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

/// TODO
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new span from the given start and end byte offsets.
    ///
    /// The interval is left-closed and right-open, aka. [start, end).
    ///
    /// # Examples
    ///
    /// ```
    /// # use kona_span::span::Span;
    /// let span = Span::new(0, 5);
    /// assert_eq!(span.start(), 0);
    /// assert_eq!(span.end(), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `start` is no less than `end`. Specially, the case
    /// `end == start` is used to represent a dummy span. You should use
    /// [`Span::dummy`] to create it instead of [`Span::new`].
    pub fn new(start: usize, end: usize) -> Span {
        assert!(start < end,
            "start index must be less than end index in a span, {}",
            if start == end {
                format!("the case `start == end` is used to represent a dummy \
                    span, please create it with `Span::dummy` instead of \
                    `Span::new`")
            } else {
                format!("but got start={start} and end={end}")
            },
        );
        Span { start, end }
    }

    /// Creates a dummy span.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kona_span::span::Span;
    /// let dummy_span = Span::dummy();
    /// assert!(dummy_span.is_dummy());
    /// ```
    pub fn dummy() -> Span {
        Span { start: 0, end: 0 }
    }

    /// Checks if the span is dummy.
    pub fn is_dummy(self) -> bool {
        self.start >= self.end
    }

    /// Returns the start byte offset of the span.
    pub fn start(self) -> usize {
        self.start
    }

    /// Returns the end byte offset of the span.
    pub fn end(self) -> usize {
        self.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Span({}..{})", self.start, self.end)
    }
}
