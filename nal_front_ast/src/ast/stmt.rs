use super::{Ast, Ident, Expr, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Ast<Expr>),
    If {
        conditional: Vec<(Ast<Expr>, Vec<Ast<Stmt>>)>,
        otherwise: Vec<Ast<Stmt>>,
    },
    While {
        condition: Ast<Expr>,
        body: Vec<Ast<Stmt>>,
    },
    Function(Ast<Function>),
    Let(Ast<Pattern>, Ast<Expr>),
    Assign(Ast<Pattern>, Ast<Expr>),
    AssignLval(Ast<Expr>, Ast<Expr>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(Ast<Ident>),
    Tuple(Vec<Ast<Pattern>>),
    Obj(Vec<ObjPatternProp>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjPatternProp {
    Named(Ast<Ident>, Ast<Pattern>),
    Short(Ast<Ident>),
}
