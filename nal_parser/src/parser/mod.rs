use nom::Err;
use super::parse_tree::Module;

#[macro_use]
mod macros;

mod common;
mod ident;
mod literal;
mod compound;
mod expr;
mod stmt;
mod pattern;
mod function;
mod module;

pub fn parse(src: &str) -> Result<Module, Err> {
    let src = common::Input::new(src);
    module::parse_module(src).to_result()
}
