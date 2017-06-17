use super::*;

#[derive(Debug)]
pub struct IdentPattern {
    name: Ast<Ident>,
    ty: Option<Ast<Ty>>,
    is_mut: bool,
}

#[derive(Debug)]
pub enum Pattern {
    Void(Option<Ast<Ty>>),
    Ident(IdentPattern),
    Record(Vec<(Ast<Ident>, Option<Ast<Pattern>>)>),
    /// (a, ...b, c)
    Tuple(Vec<Ast<Pattern>>, Option<(Ast<IdentPattern>, Vec<Ast<Pattern>>)>),
}

#[derive(Debug)]
pub enum CondPattern {
    ValueEq(Ast<Expr>),
    EnumMatch(Ast<Ident>, Ast<CondPattern>),
    Void(Option<Ast<Ty>>),
    Ident(IdentPattern),
    Record(Vec<(Ast<Ident>, Option<Ast<CondPattern>>)>),
    /// (a, ...b, c)
    Tuple(Vec<Ast<CondPattern>>, Option<(Ast<IdentPattern>, Vec<Ast<CondPattern>>)>),
}
