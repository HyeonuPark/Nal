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

// This is the Rusty way to implement Higher-Kinded-Type, right?
// Modules below are copy-pasted from nal_ast::ast
// These code use types like `super::Block` which is vary from here to nal_ast::ast
// So underlying types are different even though they have exactly same code.
// See `build.rs` for how they're copied.
mod expr { include!(concat!(env!("OUT_DIR"), "/ast_expr.rs")); }
mod stmt { include!(concat!(env!("OUT_DIR"), "/ast_stmt.rs")); }
mod function { include!(concat!(env!("OUT_DIR"), "/ast_function.rs")); }
mod module { include!(concat!(env!("OUT_DIR"), "/ast_module.rs")); }

pub use self::expr::*;
pub use self::stmt::*;
pub use self::function::*;
pub use self::module::*;
