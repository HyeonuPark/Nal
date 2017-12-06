use super::{Span, Block, Ident, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Span<Ident>),
    Literal(Span<Literal>),
    Tagged(Span<Ident>, Option<Span<Expr>>),
    Binary(BinaryOp, Span<Expr>, Span<Expr>),
    Unary(UnaryOp, Span<Expr>),
    Call {
        callee: Span<Expr>,
        args: Block<TupleElem>,
    },
    Prop(Span<Expr>, Span<Ident>),
    Return(Option<Span<Expr>>),
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
    Function(Span<Function>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElem {
    Atom(Span<Expr>),
    Spread(Span<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjProp {
    Named(Span<Ident>, Span<Expr>),
    Short(Span<Ident>),
    Method(Span<Function>),
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