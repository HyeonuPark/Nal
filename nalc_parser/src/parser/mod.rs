
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
    pub use nal_ast::prelude::*;
    pub use super::common::*;

    pub fn noop<T>(_: T) {}
}

use nom;
use nom::types::CompleteStr;
use nal_ast::prelude::Module;

pub type ParseError<'a> = nom::Err<CompleteStr<'a>>;

pub fn parse(src: &str) -> Result<Module, ParseError> {
    self::module::parse_module(CompleteStr(src)).map(|(_, module)| module)
}
