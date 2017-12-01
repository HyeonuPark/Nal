use super::{Ast, Block, Ident, Pattern, Stmt, Expr};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<Ast<Ident>>,
    pub params: Option<Block<Pattern>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Stmt(Block<Stmt>),
    Expr(Ast<Expr>),
}
