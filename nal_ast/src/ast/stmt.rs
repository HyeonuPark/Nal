use super::{Node, Block, Ident, Expr, NamedFunction};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Node<Expr>),
    If(IfStmt),
    While(Node<Expr>, Block<Stmt>),
    Function {
        is_static: bool,
        func: Node<NamedFunction>,
    },
    Let(Node<Decl>, Node<Expr>),
    Assign(Node<Pattern>, Node<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IfStmt(pub Node<Expr>, pub Block<Stmt>, pub IfFalse);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IfFalse {
    None,
    Base(Block<Stmt>),
    Chain(Node<IfStmt>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Decl {
    Ident {
        is_mut: bool,
        ident: Node<Ident>,
    },
    Tuple(Block<TupleElemDecl>),
    Obj(Block<ObjPropDecl>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElemDecl {
    Atom(Node<Decl>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPropDecl {
    Named(Node<Ident>, Node<Decl>),
    Short(Node<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(Node<Ident>),
    Tuple(Block<TupleElemPattern>),
    Obj(Block<ObjPropPattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElemPattern {
    Atom(Node<Pattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPropPattern {
    Named(Node<Ident>, Node<Pattern>),
    Short(Node<Ident>),
}
