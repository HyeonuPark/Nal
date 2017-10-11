use super::common::{Ast, Ident};
use super::stmt::{Pattern, StmtBlock};
use super::expr::Expr;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<Ast<Ident>>,
    pub params: Vec<Ast<Pattern>>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Stmt(StmtBlock),
    Expr(Ast<Expr>),
}
