use nom::Err;
use super::ast::Module;

#[macro_use]
mod macros;

mod common;
mod ident;
mod literal;
mod compound;
mod expr;
mod stmt;
mod module;

pub fn parse<S: AsRef<str>>(src: S) -> Result<Module, Err> {
    let src = common::Input::new(src.as_ref());
    module::parse_module(src).to_result()
}
