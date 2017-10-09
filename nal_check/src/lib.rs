#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nal_ast;

pub mod scope;
pub mod common;

mod module;
mod ident;
mod stmt;
mod expr;

use nal_ast::SourceBuffer;
use nal_ast::ast::common::Span;
use common::{Check, Error};

pub fn check(src: &SourceBuffer) -> Result<(), Vec<(Error, Span)>> {
    let mut acc = Vec::new();
    src.check(&mut scope::Scope::new(), &mut acc);

    if acc.len() > 0 {
        Err(acc)
    } else {
        Ok(())
    }
}
