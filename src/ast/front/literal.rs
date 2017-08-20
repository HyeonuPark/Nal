use super::{Ast, Ident, Expr, Ty, Pattern, Stmt};

#[derive(Debug)]
pub enum Literal {
    Str(String),
    FormatStr(String, Vec<(Ast<Expr>, String)>),
    Bool(bool),
    Int(i32),
    Num(f64),
    Tuple(Vec<Ast<TupleElem>>),
    Func {
        name: Option<Ast<Ident>>,
        qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
        params: Vec<(Ast<Pattern>, Option<Ast<Ty>>)>,
        rest_param: Option<Ast<FuncRestParam>>,
        return_ty: Option<Ast<Ty>>,
        body: Ast<FuncBody>,
    },
    Obj {
        name: Option<Ast<Ident>>,
        body: Vec<Ast<ObjElem>>,
    },
    Trait {
        name: Option<Ast<Ident>>,
        qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
        input: Option<Ast<Ty>>,
        body: Vec<Ast<TraitElem>>,
    },
}

#[derive(Debug)]
pub enum NamedLiteral {
    Func {
        name: Ast<Ident>,
        qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
        params: Vec<(Ast<Pattern>, Option<Ast<Ty>>)>,
        rest_param: Option<Ast<FuncRestParam>>,
        return_ty: Option<Ast<Ty>>,
        body: Ast<FuncBody>,
    },
    Obj {
        name: Ast<Ident>,
        body: Vec<Ast<ObjElem>>,
    },
    Trait {
        name: Ast<Ident>,
        qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
        input: Option<Ast<Ty>>,
        body: Vec<Ast<TraitElem>>,
    },
}

#[derive(Debug)]
pub struct FuncRestParam {
    name: Ast<Ident>,
    is_mut: bool,
    ty: Ast<Ty>,
}

#[derive(Debug)]
pub enum FuncBody {
    Stmt(Vec<Ast<Stmt>>),
    Expr(Ast<Expr>),
}

#[derive(Debug)]
pub enum ObjElem {
    Spread(Ast<Expr>),
    Prop {
        name: Ast<Ident>,
        value: Option<Ast<Expr>>,
        is_override: bool,
    },
    Named {
        literal: Ast<NamedLiteral>,
        is_override: bool,
    },
}

#[derive(Debug)]
pub enum TraitElem {
    Spread(Ast<Expr>),
    Method  {
        name: Ast<Ident>,
        is_override: bool,
        qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
        params: Vec<(Ast<Pattern>, Option<Ast<Ty>>)>,
        rest_param: Option<Ast<FuncRestParam>>,
        return_ty: Option<Ast<Ty>>,
        body: Ast<FuncBody>,
    },
}

#[derive(Debug)]
pub enum TupleElem {
    Simple(Ast<Expr>),
    Spread(Ast<Expr>),
}
