use super::{Ast, Block, Ident, Expr, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Ast<Expr>),
    If {
        conditional: Vec<(Ast<Expr>, Block<Stmt>)>,
        otherwise: Option<Block<Stmt>>,
    },
    While {
        condition: Ast<Expr>,
        body: Block<Stmt>,
    },
    Function {
        is_static: bool,
        func: Ast<Function>,
    },
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident {
        is_mut: bool,
        ident: Ast<Ident>,
    },
    Tuple(Block<TupleElemPattern>),
    Obj(Block<ObjPropPattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TupleElemPattern {
    Atom(Ast<Pattern>),
    Spread(Ast<Ident>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPropPattern {
    Named(Ast<Ident>, Ast<Pattern>),
    Short(Ast<Ident>),
}
