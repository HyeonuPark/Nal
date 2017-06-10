use ast::*;

#[derive(Debug)]
pub enum Stmt {
    If(Ast<Expr>, Vec<Ast<Stmt>>),
    While(Ast<Expr>, Vec<Ast<Stmt>>),
    ForIn(Ast<Pattern>, Ast<Expr>, Vec<Ast<Stmt>>),
    TyDecl(Ast<TyDecl>),
    NamedLiteral(Ast<Ident>, Vec<Ast<Expr>>, Ast<NamedLiteral>),
    Expr(Ast<Expr>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    IndexAssign(Ast<Expr>, Ast<Expr>, Ast<Expr>),
    Return(Ast<Expr>),
    Break,
    Continue,
}