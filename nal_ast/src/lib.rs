//! AST and parser for Nal language.
//!
//! `ast` module contains types of AST nodes itself.
//!
//! Some specifications of AST nodes are not enforced by Rust's type system
//! to simplify parser implementation. But they're validated right after
//! AST contruction so `SourceBuffer` will not contains invalid AST.
#![deny(warnings)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate serde_yaml;

pub mod ast;

mod parse;
mod check;

mod buffer;
pub use buffer::SourceBuffer;

mod report;
pub use report::Report;
