use std::usize;

/// Span is identical to `(start_offset, end_offset)`
/// but guaranteed to be `start_offset <= end_offset`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span(usize, usize);

impl Span {
    /// Create new `Span` from start/end offset
    ///
    /// # Panics
    ///
    /// Panics if `start` is greater then `end`
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end,
                "Span end MUST be greater or equal then span start");
        Span(start, end)
    }

    pub fn dummy() -> Self {
        Span(usize::MAX, usize::MAX)
    }

    pub fn is_dummy(&self) -> bool {
        self.0 == usize::MAX && self.1 == usize::MAX
    }

    pub fn pair(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    pub fn start(&self) -> usize {
        self.0
    }

    pub fn end(&self) -> usize {
        self.1
    }
}

impl From<(usize, usize)> for Span {
    /// # Panics
    ///
    /// Panics if `start` is greater then `end`
    fn from(s: (usize, usize)) -> Self {
        Self::new(s.0, s.1)
    }
}

impl AsRef<Span> for Span {
    fn as_ref(&self) -> &Span {
        self
    }
}

impl Default for Span {
    fn default() -> Self {
        Span::dummy()
    }
}

impl ::std::ops::Add for Span {
    type Output = Self;

    /// Create union of given spans
    fn add(self, right: Span) -> Self {
        Self::new(self.0.min(right.0), self.1.max(right.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(left: usize, right: usize) -> Span {
        Span::new(left, right)
    }

    #[test]
    fn test_new_span() {
        s(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_fail_new_span() {
        s(2, 1);
    }

    #[test]
    fn test_getter() {
        let span = s(1, 2);
        assert_eq!(span.start(), 1);
        assert_eq!(span.end(), 2);
        assert_eq!(span.pair(), (1, 2));
    }

    #[test]
    fn test_from_tuple() {
        assert_eq!(s(1, 2), (1, 2).into());
    }

    #[test]
    #[should_panic]
    fn test_fail_from_tuple() {
        Span::from((2, 1));
    }

    #[test]
    fn test_add_span() {
        assert_eq!(s(1, 2) + s(3, 4), s(1, 4));
        assert_eq!(s(1, 3) + s(2, 4), s(1, 4));
        assert_eq!(s(1, 4) + s(2, 3), s(1, 4));
    }
}
