use super::{Ast, Ident, AExpr};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'src> {
    Let(Ast<'src, Pattern<'src>>, AExpr<'src>),
    Assign(Ast<'src, Pattern<'src>>, AExpr<'src>),
    If(AExpr<'src>, StmtBlock<'src>, Option<StmtBlock<'src>>),
    Expr(AExpr<'src>),
}

pub type StmtBlock<'src> = Vec<Ast<'src, Stmt<'src>>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'src> {
    Ident(Ident<'src>, bool),
}
