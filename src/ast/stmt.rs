use ast::{Ast, Ident, Expr};

pub type StmtBlock<'a> = Vec<Ast<'a, Stmt<'a>>>;

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    Expr(Ast<'a, Expr<'a>>),
    Let(Ident, bool, Ast<'a, Expr<'a>>),
    Assign(Ident, Ast<'a, Expr<'a>>),
    If(Ast<'a, Expr<'a>>, StmtBlock<'a>, Option<StmtBlock<'a>>),
}
