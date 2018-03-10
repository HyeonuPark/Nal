extern crate internship;

mod node;
mod ast;

pub mod prelude {
    pub use super::node::{Node, Block, Span};
    pub use super::ast::*;
}
