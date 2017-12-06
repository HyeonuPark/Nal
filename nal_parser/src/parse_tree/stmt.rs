use super::{Span, Block, Ident, Expr, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Span<Expr>),
    If {
        conditional: Vec<(Span<Expr>, Block<Stmt>)>,
        otherwise: Option<Block<Stmt>>,
    },
    While {
        condition: Span<Expr>,
        body: Block<Stmt>,
    },
    Function {
        is_static: bool,
        func: Span<Function>,
    },
    Let(Span<Pattern>, Span<Expr>),
    Assign(Span<Pattern>, Span<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident {
        is_mut: bool,
        ident: Span<Ident>,
    },
    Tuple(Block<TupleElemPattern>),
    Obj(Block<ObjPropPattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElemPattern {
    Atom(Span<Pattern>),
    Spread(Span<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPropPattern {
    Named(Span<Ident>, Span<Pattern>),
    Short(Span<Ident>),
}
