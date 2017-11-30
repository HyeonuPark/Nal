use super::{Ast, Ident, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Ast<Ident>),
    Literal(Ast<Literal>),
    Binary(BinaryOp, Ast<Expr>, Ast<Expr>),
    Unary(UnaryOp, Ast<Expr>),
    Call {
        callee: Ast<Expr>,
        args: Vec<Ast<Expr>>,
    },
    Prop {
        parent: Ast<Expr>,
        field: Ast<Ident>,
    },
    Return(Option<Ast<Expr>>),
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool,
    Num,
    Str,
    Tuple(Vec<Ast<Expr>>),
    Obj(Vec<ObjProp>),
    Function(Ast<Function>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjProp {
    Named(Ast<Ident>, Ast<Expr>),
    Short(Ast<Ident>),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOp {
    Not, Neg,
}
