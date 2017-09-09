
use ast::{Ast, Ident};

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
  Literal(Ast<'a, Literal>),
  Binary(BinaryOp, Ast<'a, Expr<'a>>, Ast<'a, Expr<'a>>),
  Unary(UnaryOp, Ast<'a, Expr<'a>>),
  Identifier(Ident),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Number(f64),
  Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
  Neg,
  Not,
}
