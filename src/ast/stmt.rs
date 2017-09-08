use ast::{Ast, Ident, Expr};

#[derive(Debug, PartialEq)]
pub struct StmtBlock<'a>(Vec<Ast<'a, Stmt<'a>>>);

impl<'a> From<Vec<Ast<'a, Stmt<'a>>>> for StmtBlock<'a> {
    fn from(arg: Vec<Ast<'a, Stmt<'a>>>) -> Self {
        StmtBlock(arg)
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    Expr(Ast<'a, Expr<'a>>),
    Let(Ident, bool, Ast<'a, Expr<'a>>),
    Assign(Ident, Ast<'a, Expr<'a>>),
}
