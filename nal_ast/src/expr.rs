use super::{Ast, Ident};

pub type AExpr<'src> = Ast<'src, Expr<'src>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'src> {
    Literal(Literal),
    Binary(BinaryOp, AExpr<'src>, AExpr<'src>),
    Unary(UnaryOp, AExpr<'src>),
    Ident(Ident<'src>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Num(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}
