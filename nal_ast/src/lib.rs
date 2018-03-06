extern crate internship;

mod node;
mod ast;

pub mod prelude {
    pub use super::node::{Node, Block};
    pub use super::ast::*;
}
