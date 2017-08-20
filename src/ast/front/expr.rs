use super::{Ast, Ident, Literal, TupleElem, CondPattern};

#[derive(Debug)]
pub enum Expr {
    Ident(Ast<Ident>),
    Literal(Ast<Literal>),
    EnumVar(Ast<Ident>, Ast<Expr>),
    Binary(BinaryOp, Ast<Expr>, Ast<Expr>),
    Unary(UnaryOp, Ast<Expr>),
    Call(Ast<Expr>, Vec<Ast<TupleElem>>),
    IndexGet(Ast<Expr>, Vec<Ast<Expr>>),
    IndexSet(Ast<Expr>, Vec<(Ast<Expr>, Ast<Expr>)>),
    Match(Ast<Expr>, Vec<(Ast<CondPattern>, Ast<Expr>)>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    And, Or,
    Eq, Neq,
    In,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}
