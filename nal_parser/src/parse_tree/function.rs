use super::{Node, Block, Ident, Decl, Stmt, Expr};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Option<Node<Ident>>,
    pub params: Option<Node<Decl>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedFunction {
    pub name: Node<Ident>,
    pub params: Option<Node<Decl>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionBody {
    Stmt(Block<Stmt>),
    Expr(Node<Expr>),
}
