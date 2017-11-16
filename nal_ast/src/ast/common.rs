use std::ops::{Deref, Add};
use std::rc::Rc;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

/// Ast node with it's span information
///
/// Under the hood, Ast<T> is made with (Box<T>, Span).
/// Box part makes it easy to construct recursive structure.
/// And Span is (usize, usize) which contains start/end offset
/// of this AST node, half inclusive so you can use it
/// as simple range like (span.0..span.1).
#[derive(Clone)]
pub struct Ast<T> {
    inner_value: Box<T>,
    pub span: Span,
}

impl<T> Ast<T> {
    pub fn new(value: T, span: Span) -> Self {
        Ast {
            inner_value: value.into(),
            span
        }
    }

    pub fn into_inner(self) -> T {
        *self.inner_value
    }

    /// Create dummy AST node without span information.
    /// This constructor makes testing parser simple,
    /// because Ast<T> considered equal when their inner T is equal
    /// regardless their span value.
    pub fn dummy(value: T) -> Self {
        Ast {
            inner_value: value.into(),
            span: Span(0, 0),
        }
    }

    /// Replace AST's inner value with mapper function.
    /// This method doesn't change associated span info.
    pub fn map<U, F>(self, mapper: F) -> Ast<U> where F: FnOnce(T) -> U {
        let Ast{ inner_value, span } = self;

        Ast {
            inner_value: mapper(*inner_value).into(),
            span,
        }
    }
}

mod dbg {
    use super::Ast;
    use std::fmt::{Debug, Formatter, Error};

    impl<T: Debug> Debug for Ast<T> {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_str("Ast:")?;
            self.inner_value.fmt(f)
        }
    }
}


impl<T: PartialEq> PartialEq for Ast<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner_value.eq(&other.inner_value)
    }
}

impl<T> Deref for Ast<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.inner_value
    }
}

impl<T: Serialize> Serialize for Ast<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.inner_value.serialize(s)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Ast<T> {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        T::deserialize(d).map(Ast::dummy)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Span(pub usize, pub usize);

impl Add for Span {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert!(self.0 < other.1);
        Span(self.0, other.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub Rc<str>);

impl Ident {
    pub fn name(&self) -> Rc<str> {
        Rc::clone(&self.0)
    }
}

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Serialize for Ident {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'d> Deserialize<'d> for Ident {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).map(|s| Ident(s.into()))
    }
}
