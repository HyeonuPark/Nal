use super::{Ast, Block, Ident, Expr, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Ast<Expr>),
    If {
        conditional: Vec<(Ast<Expr>, Block<Stmt>)>,
        otherwise: Block<Stmt>,
    },
    While {
        condition: Ast<Expr>,
        body: Block<Stmt>,
    },
    Function(Ast<Function>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    AssignLval(Ast<Expr>, Ast<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(Ast<Ident>),
    Tuple(Block<Pattern>),
    Obj(Block<ObjPatternProp>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPatternProp {
    Named(Ast<Ident>, Ast<Pattern>),
    Short(Ast<Ident>),
}
