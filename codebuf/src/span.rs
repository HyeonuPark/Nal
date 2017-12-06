use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Clone)]
pub struct Span<T> {
    inner: Box<T>,
    pub span: SpanInfo,
}

impl<T> Span<T> {
    pub fn new(span: SpanInfo, inner: T) -> Self {
        Span {
            inner: inner.into(),
            span,
        }
    }

    pub fn dummy(inner: T) -> Self {
        Span {
            inner: inner.into(),
            span: SpanInfo::dummy(),
        }
    }

    pub fn into_inner(spanned: Self) -> T {
        *spanned.inner
    }

    pub fn map<U, F: FnOnce(T) -> U>(spanned: Self, f: F) -> Span<U> {
        Span {
            inner: f(*spanned.inner).into(),
            span: spanned.span,
        }
    }
}

impl<T> ::std::ops::Deref for Span<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: PartialEq> PartialEq for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Serialize> Serialize for Span<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(s)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Span<T> {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        Ok(Span {
            inner: T::deserialize(d)?.into(),
            span: SpanInfo::dummy(),
        })
    }
}

mod dbg {
    use super::Span;
    use std::fmt::{Debug, Formatter, Error};

    impl<T: Debug> Debug for Span<T> {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            self.inner.fmt(f)
        }
    }
}

/// This struct is identical to `(start_offset, end_offset)`
/// but guaranteed to be `start_offset <= end_offset`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanInfo(usize, usize);

impl SpanInfo {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end,
                "Span end MUST be greater or equal then span start");
        SpanInfo(start, end)
    }

    pub fn dummy() -> Self {
        SpanInfo(0, 0)
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

impl From<(usize, usize)> for SpanInfo {
    fn from(s: (usize, usize)) -> Self {
        Self::new(s.0, s.1)
    }
}

impl ::std::ops::Add for SpanInfo {
    type Output = Self;

    fn add(self, right: SpanInfo) -> Self {
        Self::new(self.0.min(right.0), self.1.max(right.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(left: usize, right: usize) -> SpanInfo {
        SpanInfo::new(left, right)
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
        SpanInfo::from((2, 1));
    }

    #[test]
    fn test_add_span() {
        assert_eq!(s(1, 2) + s(3, 4), s(1, 4));
        assert_eq!(s(1, 3) + s(2, 4), s(1, 4));
        assert_eq!(s(1, 4) + s(2, 3), s(1, 4));
    }
}
