#[macro_use]
mod ast_macro;

pub mod common;
pub mod ident;
pub mod literal;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod module;

pub use nom::IError as ParseError;

use ast::module::Module;

pub fn parse(src: &str) -> Result<Module, ParseError> {
    module::parse_module(common::Input::new(src)).to_full_result()
}
