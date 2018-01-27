use super::{Node, Block, Ident, Expr, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Node<Expr>),
    If(IfStmt),
    While(Node<Expr>, Block<Stmt>),
    Function {
        is_static: bool,
        func: Node<Function>,
    },
    Let(Node<Pattern>, Node<Expr>),
    Assign(Node<Pattern>, Node<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IfStmt(pub Node<Expr>, pub Block<Stmt>, pub IfFalse);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IfFalse {
    None,
    Base(Block<Stmt>),
    Chain(Box<IfStmt>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident {
        is_mut: bool,
        ident: Node<Ident>,
    },
    Tuple(Block<TupleElemPattern>),
    Obj(Block<ObjPropPattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElemPattern {
    Atom(Node<Pattern>),
    Spread(Node<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPropPattern {
    Named(Node<Ident>, Node<Pattern>),
    Short(Node<Ident>),
}
