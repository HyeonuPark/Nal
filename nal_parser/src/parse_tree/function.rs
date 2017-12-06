use super::{Span, Block, Ident, Pattern, Stmt, Expr};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<Span<Ident>>,
    pub params: Option<Span<Pattern>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Stmt(Block<Stmt>),
    Expr(Span<Expr>),
}
