use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct Ast<'src, T> {
    inner_value: Box<T>,
    pub span: Span<'src>,
}

impl<'src, T> Deref for Ast<'src, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.inner_value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span<'src> {
    offset: usize,
    input: &'src str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident<'src>(&'src str);
