
/// Something that holds span information
pub trait Spanned {
    fn span(&self) -> Span;
}

/// This struct is identical to `(start_offset, end_offset)`
/// but guaranteed to be `start_offset < end_offset`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span(usize, usize);

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end, "Span end MUST be greater then span start");
        Span(start, end)
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
    fn from(s: (usize, usize)) -> Self {
        Span::new(s.0, s.1)
    }
}

impl ::std::ops::Add for Span {
    type Output = Self;

    fn add(self, right: Span) -> Span {
        Span::new(self.0, right.1)
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

    #[test]
    #[should_panic]
    fn test_fail_add_span() {
        s(3, 4) + s(1, 2);
    }
}
