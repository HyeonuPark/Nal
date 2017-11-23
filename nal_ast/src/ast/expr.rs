use super::common::{Ast, Ident};
use super::function::Function;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(Ast<Literal>),
    Binary(BinaryOp, Ast<Expr>, Ast<Expr>),
    Unary(UnaryOp, Ast<Expr>),
    Call(Ast<Expr>, Vec<Ast<Expr>>),
    Prop(Ast<Expr>, Ast<Ident>),
    /// Return expr should be located inside of some function
    Return(Option<Ast<Expr>>),
    /// Break expr should be located inside of some loop
    Break,
    /// Continue expr should be located inside of some loop
    Continue,
    Function(Ast<Function>),
    Ident(Ast<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Num(f64),
    Bool(bool),
    Str(String),
    /// Obj literal should not have duplicated keys
    Obj(Vec<Ast<ObjProp>>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjProp {
    Named(Ast<Ident>, Ast<Expr>),
    Short(Ast<Ident>),
    /// Method function should have it's name
    Method(Ast<Function>),
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
