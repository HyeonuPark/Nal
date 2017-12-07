use super::{Node, Block, Ident, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Node<Ident>),
    Literal(Node<Literal>),
    Call(Node<Expr>, Block<TupleElem>),
    Prop(Node<Expr>, Node<Ident>),
    Unary(UnaryOp, Node<Expr>),
    Binary(BinaryOp, Node<Expr>, Node<Expr>),
    Tagged(Node<Ident>, Option<Node<Expr>>),
    Return(Option<Node<Expr>>),
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
    Function(Node<Function>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElem {
    Atom(Node<Expr>),
    Spread(Node<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjProp {
    Named(Node<Ident>, Node<Expr>),
    Short(Node<Ident>),
    Method(Node<Function>),
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
