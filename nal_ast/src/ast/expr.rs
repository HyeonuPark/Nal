use super::*;

#[derive(Debug)]
pub enum Expr {
    Variable(Node<Ident>),
    Literal(Node<Literal>),
    Tuple(Block<TupleElem>),
    Obj(Block<ObjElem>),
    Function(Node<Function>),

    Unary(UnaryOp, Node<Expr>),
    Binary(BinaryOp, Node<Expr>, Node<Expr>),
    Call {
        callee: Node<Expr>,
        argument: Node<Expr>,
    },
    ObjField {
        parent: Node<Expr>,
        field: Node<Ident>,
    },
    TupleField {
        parent: Node<Expr>,
        field: Node<usize>,
    },

    Return(Option<Node<Expr>>),
    Break,
    Continue,
}

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Num(f64),
    Str(String),
}

#[derive(Debug)]
pub enum TupleElem {
    Atom(Node<Expr>),
}

#[derive(Debug)]
pub enum ObjElem {
    Named(Node<Ident>, Node<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}
