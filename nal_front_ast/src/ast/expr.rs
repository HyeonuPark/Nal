use super::{Ast, Block, Ident, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Ast<Ident>),
    Literal(Ast<Literal>),
    Tagged(Ast<Ident>, Option<Ast<Expr>>),
    Binary(BinaryOp, Ast<Expr>, Ast<Expr>),
    Unary(UnaryOp, Ast<Expr>),
    Call {
        callee: Ast<Expr>,
        args: Block<TupleElem>,
    },
    Prop(Ast<Expr>, Ast<Ident>),
    Return(Option<Ast<Expr>>),
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool,
    Num,
    Str,
    Tuple(Block<TupleElem>),
    Obj(Block<ObjProp>),
    Function(Ast<Function>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElem {
    Atom(Ast<Expr>),
    Spread(Ast<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjProp {
    Named(Ast<Ident>, Ast<Expr>),
    Short(Ast<Ident>),
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
    Not, Neg,
}
