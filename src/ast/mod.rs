use std::ops::Deref;

use nom_locate::LocatedSpan;

mod expr;
pub use self::expr::*;

mod stmt;
pub use self::stmt::*;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Ast<'a, T> {
  inner_value: Box<T>,
  pub span: Span<'a>,
}

impl<'a, T> Ast<'a, T> {
  pub fn new(span: Span<'a>, value: T) -> Self {
    Ast {
      inner_value: value.into(),
      span,
    }
  }

  pub fn with_merge(value: T, left: Span<'a>, right: Span<'a>) -> Self {
    let (left, right) = match (left, right) {
      (l, r) if l.offset + l.fragment.len() < r.offset => (l, r),
      (l, r) if r.offset + r.fragment.len() < l.offset => (r, l),
      _ => panic!("Two spans must not overlap"),
    };

    Ast {
      inner_value: value.into(),
      span: Span {
        offset: left.offset,
        line: left.line,
        fragment: unsafe { left.fragment.slice_unchecked(0, right.offset - left.offset + right.fragment.len()) },
      }
    }
  }

  pub fn unwrap(ast: Ast<'a, T>) -> T {
    *ast.inner_value
  }

  pub fn dummy(value: T) -> Self {
    Ast {
      inner_value: value.into(),
      span: Span::new(""),
    }
  }
}

impl<'a, T: PartialEq> PartialEq for Ast<'a, T> {
  fn eq(&self,other: &Self) -> bool {
    self.inner_value == other.inner_value
  }
}

impl<'a, T> Deref for Ast<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.inner_value
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Ident(pub String);
