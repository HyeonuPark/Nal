extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate internship;

mod common;
mod opcode;
mod func;
mod module;

pub mod prelude {
    pub use super::common::*;
    pub use super::opcode::*;
    pub use super::func::*;
    pub use super::module::*;
}
