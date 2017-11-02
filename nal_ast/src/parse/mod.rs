#[macro_use]
mod ast_macro;

pub mod common;
pub mod ident;
pub mod literal;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod module;
pub mod function;
pub mod string;

pub use nom::Err as ParseError;

use ast::module::Module;

pub fn parse(src: &str) -> Result<Module, ParseError> {
    module::parse_module(common::Input::new(src)).to_result()
}

#[cfg(test)]
mod tests;
