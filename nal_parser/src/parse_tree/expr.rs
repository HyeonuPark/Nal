use super::{Node, Block, Ident, Literal, Function, NamedFunction};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(Node<Ident>),
    Literal(Node<Literal>),
    Tuple(Block<TupleElem>),
    Obj(Block<ObjProp>),
    Function(Node<Function>),
    Call(Node<Expr>, Block<TupleElem>),
    Prop(Node<Expr>, Node<Ident>),
    Unary(UnaryOp, Node<Expr>),
    Binary(BinaryOp, Node<Expr>, Node<Expr>),
    Tagged(Node<Ident>, Option<Node<Expr>>),
    Return(Option<Node<Expr>>),
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TupleElem {
    Atom(Node<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjProp {
    Named(Node<Ident>, Node<Expr>),
    Short(Node<Ident>),
    Method(Node<NamedFunction>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Not, Neg,
}
