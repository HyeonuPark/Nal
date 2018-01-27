pub use codebuf::Node;

/// This type represent sequence of elements
/// where parser fails are isolated to its containing line.
///
/// `Ok(Node<T>)` represents parsed line and
/// `Err(Node<()>)` represents parse failed
pub type Block<T> = Node<[Result<Node<T>, Node<()>>]>;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Ident;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool,
    Num,
    Str,
}

mod expr;
mod stmt;
mod function;
mod module;

pub use self::expr::*;
pub use self::stmt::*;
pub use self::function::*;
pub use self::module::*;
