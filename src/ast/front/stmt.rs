use super::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(Ast<Expr>),
    NamedLiteral(Ast<Ident>, Vec<Ast<Expr>>, Ast<NamedLiteral>),
    TyDecl(Ast<TyDecl>),
    If(Ast<Expr>, Vec<Ast<Stmt>>),
    While(Ast<Expr>, Vec<Ast<Stmt>>),
    IfLet(Ast<CondPattern>, Ast<Expr>, Vec<Ast<Stmt>>),
    WhileLet(Ast<CondPattern>, Ast<Expr>, Vec<Ast<Stmt>>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    IndexAssign(Ast<Expr>, Ast<Expr>, Ast<Expr>),
    ForIn(Ast<Pattern>, Ast<Expr>, Vec<Ast<Stmt>>),
    Return(Ast<Expr>),
    Break,
    Continue,
}