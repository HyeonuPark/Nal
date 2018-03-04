use serde::{Serialize, Serializer, Deserialize, Deserializer};
use span::Span;

pub struct Node<T: ?Sized> {
    inner: Box<T>,
    pub span: Span,
}

impl<T: ?Sized> Node<T> {
    pub fn new<U: Into<Box<T>>>(span: Span, inner: U) -> Self {
        Node {
            inner: inner.into(),
            span,
        }
    }

    pub fn dummy<U: Into<Box<T>>>(inner: U) -> Self {
        Node {
            inner: inner.into(),
            span: Span::dummy(),
        }
    }
}

impl<T> Node<T> {
    pub fn into_inner(node: Self) -> T {
        *node.inner
    }

    pub fn map<U, F: FnOnce(T) -> U>(node: Self, f: F) -> Node<U> {
        Node {
            inner: f(*node.inner).into(),
            span: node.span,
        }
    }
}

impl<T: ?Sized> ::std::ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: ?Sized> AsRef<Span> for Node<T> {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl<T: PartialEq + ?Sized> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: ?Sized> Clone for Node<T> where Box<T>: Clone {
    fn clone(&self) -> Self {
        Node {
            inner: self.inner.clone(),
            span: self.span.clone(),
        }
    }
}

impl<T: Serialize + ?Sized> Serialize for Node<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(s)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Node<T> {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        Ok(Node {
            inner: T::deserialize(d)?.into(),
            span: Span::dummy(),
        })
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Node<[T]> {
    fn deserialize<D: Deserializer<'d>>(d: D) -> Result<Self, D::Error> {
        Ok(Node {
            inner: Vec::<T>::deserialize(d)?.into(),
            span: Span::dummy(),
        })
    }
}

mod dbg {
    use super::Node;
    use std::fmt::{Debug, Formatter, Error};

    impl<T: Debug + ?Sized> Debug for Node<T> {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            self.inner.fmt(f)
        }
    }
}
