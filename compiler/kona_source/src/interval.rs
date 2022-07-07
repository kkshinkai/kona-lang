// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

use crate::pos::Pos;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Interval {
    start: Pos,
    end: Pos,
}

impl Interval {
    /// Creates a new left-closed, right-open interval `[start, end)` with the
    /// given positions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kona_source::interval::Interval;
    /// let interval = Interval::new(
    ///     Pos::from_uszie(0),
    ///     Pos::from_uszie(5),
    /// );
    /// assert_eq!(interval.start(), Pos::from_uszie(0));
    /// assert_eq!(interval.end(), Pos::from_uszie(5));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `start` is no less than `end`. Specially, the case
    /// `end == start` is used to represent a dummy interval. You should use
    /// [`Interval::dummy`] to create it instead of [`Interval::new`].
    pub fn new(start: Pos, end: Pos) -> Interval {
        debug_assert!(start < end,
            "start index must be less than end index in a interval, {}",
            if start == end {
                format!("the case `start == end` is used to represent a dummy \
                    interval, please create it with `Interval::dummy` instead \
                    of `Interval::new`")
            } else {
                format!("but got start={} and end={}",
                    start.to_usize(), end.to_usize())
            },
        );
        Interval { start, end }
    }

    /// Creates a dummy interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kona_source::interval::Interval;
    /// let dummy_interval = Interval::dummy();
    /// assert!(dummy_interval.is_dummy());
    /// ```
    pub fn dummy() -> Interval {
        let zero = Pos::from_usize(0);
        Interval::new(zero, zero)
    }

    /// Checks if the interval is dummy.
    pub fn is_dummy(self) -> bool {
        self.start >= self.end
    }

    /// Returns the start position of the interval.
    pub fn start(self) -> Pos {
        self.start
    }

    /// Returns the end position of the interval.
    pub fn end(self) -> Pos {
        self.end
    }
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interval({}..{})",
            self.start.to_usize(), self.end.to_usize())
    }
}
