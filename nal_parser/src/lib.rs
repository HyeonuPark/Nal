#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;
#[macro_use]
extern crate lazy_static;

extern crate codebuf;
extern crate nal_ast;

mod parse_tree;
mod parser;
mod convert;
mod error;

pub use error::Error;

use codebuf::CodeBuf;
use nal_ast::ast;

pub fn parse(buf: &CodeBuf) -> Result<ast::Module, (Option<ast::Module>, Error)> {
    match parser::parse(buf.code()) {
        Err(e) => Err((None, e.into())),
        Ok(ref module) => match convert::convert(buf, module) {
            Ok(module) => Ok(module),
            Err((module, e)) => Err((module, e.into()))
        }
    }
}
