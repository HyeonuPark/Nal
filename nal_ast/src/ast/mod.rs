use internship::InternStr;

pub use super::{Node, Block};

mod expr;
pub use self::expr::*;

mod stmt;
pub use self::stmt::*;

mod func;
pub use self::func::*;

mod module;
pub use self::module::*;

#[derive(Debug)]
pub struct Ident(pub InternStr);
