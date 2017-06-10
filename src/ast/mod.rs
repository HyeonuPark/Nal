use std::ops::Deref;

mod expr;
mod literal;
mod stmt;
mod pattern;
mod ty;

pub use self::expr::*;
pub use self::literal::*;
pub use self::stmt::*;
pub use self::pattern::*;
pub use self::ty::*;

 #[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ast<T> {
    content: Box<T>,
}

impl<T> Ast<T> {
    pub fn new(content: T) -> Self {
        Ast {
            content: Box::new(content),
        }
    }
}

impl<T> Deref for Ast<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.content.deref()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ident(String);
