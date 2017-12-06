use std::ops::Deref;
use std::cmp::PartialEq;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use codebuf::{Span, Spanned};

#[derive(Clone)]
pub struct Ast<T> {
    inner: Box<T>,
    span: Span,
}

impl<T> Ast<T> {
    pub fn new(span: Span, inner: T) -> Self {
        Ast {
            inner: inner.into(),
            span,
        }
    }

    pub fn dummy(inner: T) -> Self {
        Ast {
            inner: inner.into(),
            span: Span::dummy(),
        }
    }

    pub fn into_inner(ast: Self) -> T {
        *ast.inner
    }

    pub fn map<U, F: FnOnce(T) -> U>(ast: Self, f: F) -> Ast<U> {
        Ast {
            inner: f(*ast.inner).into(),
            span: ast.span,
        }
    }
}

impl<T> Spanned for Ast<T> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Deref for Ast<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: PartialEq> PartialEq for Ast<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Serialize> Serialize for Ast<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(s)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Ast<T> {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        Ok(Ast {
            inner: T::deserialize(d)?.into(),
            span: Span::dummy(),
        })
    }
}

mod dbg {
    use super::Ast;
    use std::fmt::{Debug, Formatter, Error};

    impl<T: Debug> Debug for Ast<T> {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            self.inner.fmt(f)
        }
    }
}

/// This type represent sequence of elements
/// where parser fails are isolated to its containing line.
///
/// `Ok(Ast<T>)` represents parsed line and
/// `Err(Ast<()>)` represents parse failed
pub type Block<T> = Ast<Vec<Result<Ast<T>, Ast<()>>>>;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Ident;
