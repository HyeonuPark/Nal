use super::{Ast, Ident, Expr, NamedLiteral, TyDecl, CondPattern, Pattern};

#[derive(Debug)]
pub enum Stmt {
    Expr(Ast<Expr>),
    NamedLiteral(Ast<Ident>, Vec<Ast<Expr>>, Ast<NamedLiteral>),
    TyDecl(Ast<TyDecl>),
    If(Ast<Expr>, Vec<Ast<Stmt>>),
    While(Ast<Expr>, Vec<Ast<Stmt>>),
    IfIs(Ast<Expr>, Ast<CondPattern>, Vec<Ast<Stmt>>),
    WhileIs(Ast<Expr>, Ast<CondPattern>, Vec<Ast<Stmt>>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    IndexAssign(Ast<Expr>, Ast<Expr>, Ast<Expr>),
    ForIn(Ast<Pattern>, Ast<Expr>, Vec<Ast<Stmt>>),
    Return(Ast<Expr>),
    Break,
    Continue,
}