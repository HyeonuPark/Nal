use super::common::{Ast, Ident};
use super::function::Function;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(Ast<Literal>),
    Binary(BinaryOp, Ast<Expr>, Ast<Expr>),
    Unary(UnaryOp, Ast<Expr>),
    Call(Ast<Expr>, Vec<Ast<Expr>>),
    Return(Option<Ast<Expr>>),
    Break,
    Continue,
    Function(Ast<Function>),
    Ident(Ast<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Num(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Not,
}
