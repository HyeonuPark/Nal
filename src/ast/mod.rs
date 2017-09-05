use std::ops::Deref;

use nom_locate::LocatedSpan;

#[macro_use]
mod ast_macro {
  macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
      map!($i,
        tuple!(
          position!(),
          $submac!($($args)*),
          position!()
        ),
        (|(left, body, right)| $crate::ast::Ast::with_merge(body, left, right))
      )
    );
    ($i:expr, $f:ident) => (
      ast!($i, call!($f))
    );
  }
}

pub mod parser;
pub mod interpreter;

mod literal;
pub use self::literal::*;

mod expr;
pub use self::expr::*;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
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

  pub fn inner(ast: Ast<'a, T>) -> T {
    *ast.inner_value
  }
}

impl<'a, T> Deref for Ast<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.inner_value.deref()
  }
}
