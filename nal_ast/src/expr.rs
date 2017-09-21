use super::{Ast, Ident};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'src> {
    Literal(Ast<'src, Literal>),
    Binary(BinaryOp, Ast<'src, Expr<'src>>, Ast<'src, Expr<'src>>),
    Unary(UnaryOp, Ast<'src, Expr<'src>>),
    Ident(Ast<'src, Ident<'src>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Num(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
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
