use super::{Ast, Ident, Expr};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    If(Ast<Expr>, StmtBlock, Option<StmtBlock>),
    Expr(Ast<Expr>),
}

pub type StmtBlock = Vec<Ast<Stmt>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(Ident, bool),
}
