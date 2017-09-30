#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;
#[macro_use]
mod ast_macro;

extern crate nal_ast as ast;

pub mod common;
pub mod ident;
pub mod literal;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod module;

pub fn parse(src: &str) -> Result<ast::Module, nom::IError> {
    module::parse_module(common::Input::new(src)).to_full_result()
}
