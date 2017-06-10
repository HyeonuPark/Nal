use ast::*;

#[derive(Debug)]
pub enum Literal {}

#[derive(Debug)]
pub enum NamedLiteral {}

#[derive(Debug)]
pub struct Func {
    name: Option<Ast<Ident>>,
    qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
    params: Ast<TuplePattern>,
    return_ty: Option<Ast<Ty>>,
    body: Ast<FuncBody>,
}

#[derive(Debug)]
pub struct NamedFunc {
    name: Ast<Ident>,
    qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
    params: Ast<TuplePattern>,
    return_ty: Option<Ast<Ty>>,
    body: Ast<FuncBody>,
}

#[derive(Debug)]
pub enum FuncBody {
    Stmt(Vec<Ast<Stmt>>),
    Expr(Ast<Expr>),
}

#[derive(Debug)]
pub struct Obj {
    name: Option<Ast<Ident>>,
    qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
    body: Vec<(Ast<Ident>, Ast<Expr>)>,
}

#[derive(Debug)]
pub struct NamedObj {
    name: Ast<Ident>,
    qualifiers: Vec<(Ast<Ident>, Option<Ast<Ty>>)>,
    body: Vec<(Ast<Ident>, Ast<Expr>)>,
}

#[derive(Debug)]
pub struct Trait {
    name: Option<Ast<Ident>>,
    param: Ast<Ident>,
    param_ty: Option<Ast<Ty>>,
    body: Vec<Ast<TraitElem>>,
}

#[derive(Debug)]
pub struct NamedTrait {
    name: Ast<Ident>,
    param: Ast<Ident>,
    param_ty: Option<Ast<Ty>>,
    body: Vec<Ast<TraitElem>>,
}

#[derive(Debug)]
pub enum TraitElem {
    Extends(Ast<Ident>),
    Method(Ast<NamedFunc>),
    Override(Ast<NamedFunc>),
}
