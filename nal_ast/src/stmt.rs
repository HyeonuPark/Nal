use super::{Ast, Ident, Expr};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'src> {
    Let(Ast<'src, Pattern<'src>>, Ast<'src, Expr<'src>>),
    Assign(Ast<'src, Pattern<'src>>, Ast<'src, Expr<'src>>),
    Expr(Ast<'src, Expr<'src>>),
    If(Ast<'src, Expr<'src>>, StmtBlock<'src>, Option<StmtBlock<'src>>),
}

pub type StmtBlock<'src> = Vec<Ast<'src, Stmt<'src>>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'src> {
    Ident(Ast<'src, Ident<'src>>, bool),
}
