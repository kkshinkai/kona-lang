// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct AheadLooker<I: Iterator> {
    pub iter: I, // FIXME: This should be private, but we need to access it in
                 //        the lexer. We need a better abstraction.
    peeked_queue: VecDeque<I::Item>,
}

impl<I: Iterator> AheadLooker<I> {
    pub fn new(iter: I) -> AheadLooker<I> {
        AheadLooker { iter, peeked_queue: VecDeque::new() }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        let need_count =
            // NOTE: Don't use `usize::max(n - self.peeked_queue.len() + 1, 0)`
            // here, these operands are unsigned, subtraction may cause
            // underflow. It is an obvious mistake to compare an unsigned
            // number with zero.
            if n >= self.peeked_queue.len() {
                n - self.peeked_queue.len() + 1
            } else {
                0
            };

        for _ in 0..need_count {
            match self.iter.next() {
                Some(item) => self.peeked_queue.push_back(item),
                None => break,
            }
        }

        self.peeked_queue.iter().nth(n)
    }
}

impl<I: Iterator> Iterator for AheadLooker<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.peeked_queue.pop_front().or_else(|| self.iter.next())
    }

    #[inline]
    fn count(self) -> usize {
        self.peeked_queue.len() + self.iter.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        let peeked_count = self.peeked_queue.len();
        if n < peeked_count {
            self.peeked_queue.drain(0..=n).last()
        } else {
            self.peeked_queue.clear();
            self.iter.nth(n - peeked_count)
        }
    }

    #[inline]
    fn last(mut self) -> Option<I::Item> {
        // Consume all the `self.iter` first anyway. If `self.iter` is empty,
        // go to `self.peeked_queue` and find if there is any peeked element
        // left.
        let last = self.iter.last().or_else(|| self.peeked_queue.pop_back());

        // Clear the `self.peeked_queue`, the iterator has been completely
        // consumed.
        self.peeked_queue.clear();

        last
    }
}

pub trait Lookahead {
    /// Get the [`AheadLooker`] that can lookahead any number of elements
    /// without consuming the iterator.
    fn ahead_looker(self) -> AheadLooker<Self> where Self: Sized + Iterator {
        AheadLooker::new(self)
    }
}

impl<I: Iterator> Lookahead for I {}

#[cfg(test)]
mod test_ahead_looker {
    use super::Lookahead;

    #[test]
    fn test_next() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter().ahead_looker();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_peek() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter().ahead_looker();
        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.next(), Some(&1));

        assert_eq!(iter.peek(), Some(&&2));
        assert_eq!(iter.peek(), Some(&&2));
        assert_eq!(iter.next(), Some(&2));


        assert_eq!(iter.peek(), Some(&&3));
        assert_eq!(iter.peek(), Some(&&3));
        assert_eq!(iter.next(), Some(&3));

        assert_eq!(iter.peek(), None);
        assert_eq!(iter.peek(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_peek_nth() {
        let vec = vec![1, 2, 3];

        let mut iter = vec.iter().ahead_looker();
        assert_eq!(iter.peek_nth(0), Some(&&1));
        assert_eq!(iter.peek_nth(1), Some(&&2));
        assert_eq!(iter.peek_nth(2), Some(&&3));
        assert_eq!(iter.peek_nth(3), None);

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.peek_nth(0), Some(&&2));
        assert_eq!(iter.peek_nth(1), Some(&&3));
        assert_eq!(iter.peek_nth(2), None);

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.peek_nth(0), Some(&&3));
        assert_eq!(iter.peek_nth(1), None);

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.peek_nth(0), None);

        let mut iter = vec.iter().ahead_looker();
        assert_eq!(iter.peek_nth(5), None);

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.peek_nth(0), Some(&&2));
        assert_eq!(iter.peek_nth(1), Some(&&3));
        assert_eq!(iter.peek_nth(2), None);

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.peek_nth(0), Some(&&3));
        assert_eq!(iter.peek_nth(1), None);

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.peek_nth(0), None);
    }
}
