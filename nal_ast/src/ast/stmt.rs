use super::common::{Ast, Ident};
use super::expr::Expr;
use super::function::Function;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    If(Ast<Expr>, StmtBlock, Option<StmtBlock>),
    While(Ast<Expr>, StmtBlock),
    ForIn(Ast<Pattern>, Ast<Expr>, StmtBlock),
    Function(bool, Ast<Function>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    Expr(Ast<Expr>),
}

pub type StmtBlock = Vec<Ast<Stmt>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(Ast<Ident>, bool),
}