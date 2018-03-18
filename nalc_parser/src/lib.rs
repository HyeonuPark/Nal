#[macro_use]
extern crate nom;

extern crate nal_ast;

#[macro_use]
mod macro_def;
pub mod common;

mod ident;
mod literal;
mod expr;
mod stmt;
mod func;
mod module;

mod prelude {
    pub use nom::types::CompleteStr as Src;
    pub use nal_ast::*;
    pub use super::common::*;

    pub fn noop<T>(_: T) {}
}

use nom::types::CompleteStr;
use nal_ast::Module;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(pub String);

impl<T: Into<String>> From<T> for Error {
    fn from(v: T) -> Self {
        Error(v.into())
    }
}

pub fn parse(src: &str) -> Result<Module, Error> {
    match self::module::parse_module(CompleteStr(src)) {
        Ok((_, module)) => Ok(module),
        Err(e) => Err(format!("{:?}", e).into()),
    }
}
