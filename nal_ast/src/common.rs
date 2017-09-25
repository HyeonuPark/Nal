use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Ast<'src, T> {
    inner_value: Box<T>,
    pub span: Span<'src>,
}

impl<'src, T> Ast<'src, T> {
    pub fn new(value: T, span: Span<'src>) -> Self {
        Ast {
            inner_value: value.into(),
            span
        }
    }

    pub fn dummy(value: T) -> Self {
        Ast {
            inner_value: value.into(),
            span: Span {
                offset: 0,
                input: "",
            }
        }
    }

    pub fn into_inner(self) -> T {
        *self.inner_value
    }
}

impl<'src, T: PartialEq> PartialEq for Ast<'src, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner_value.eq(&other.inner_value)
    }
}

impl<'src, T> AsRef<T> for Ast<'src, T> {
    fn as_ref(&self) -> &T {
        &*self.inner_value
    }
}

impl<'src, T> Deref for Ast<'src, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.inner_value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span<'src> {
    pub offset: usize,
    pub input: &'src str,
}

impl<'src> Span<'src> {
    pub fn merge(&self, right: &Self) -> Self {
        assert!(self.offset < right.offset + right.input.len());
        let length = right.offset + right.input.len() - self.offset;

        Span {
            offset: self.offset,
            input: unsafe { self.input.slice_unchecked(0, length) }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident<'src>(pub &'src str);

impl<'src> Deref for Ident<'src> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
