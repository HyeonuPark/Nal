use super::{Node, Block, Ident, Pattern, Stmt, Expr};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<Node<Ident>>,
    pub params: Option<Node<Pattern>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Stmt(Block<Stmt>),
    Expr(Node<Expr>),
}
