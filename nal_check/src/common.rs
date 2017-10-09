use nal_ast::ast::common::Span;

use scope::{Scope as ScopeType, Error as ScopeError};
use {ident, stmt};

pub type Scope<'a> = &'a mut ScopeType;
pub type Acc<'a> = &'a mut Vec<(Error, Span)>;

pub trait Check {
    fn check(&self, scope: Scope, acc: Acc);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    Scope(ScopeError),
    Ident(ident::Error),
    Stmt(stmt::Error),
}

impl From<ScopeError> for Error {
    fn from(err: ScopeError) -> Self {
        Error::Scope(err)
    }
}

impl From<ident::Error> for Error {
    fn from(err: ident::Error) -> Self {
        Error::Ident(err)
    }
}

impl From<stmt::Error> for Error {
    fn from(err: stmt::Error) -> Self {
        Error::Stmt(err)
    }
}
