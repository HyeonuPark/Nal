use super::*;

#[derive(Debug)]
pub enum Ty {
    Ref(Ast<Ident>, Vec<Ast<Ty>>),
    Add(Ast<Ty>, Ast<Ty>),
    Tuple(Vec<Ast<TupleTypeElem>>),
    Record {
        name: Option<Ast<Ident>>,
        body: Vec<Ast<TypeElem>>,
    },
    Enum {
        name: Option<Ast<Ident>>,
        body: Vec<Ast<TypeElem>>,
    },
}

#[derive(Debug)]
pub enum TyDecl {
    Alias {
        name: Ast<Ident>,
        params: Vec<Ast<Ident>>,
        body: Ast<Ty>,
    },
    Record {
        name: Ast<Ident>,
        params: Vec<Ast<Ident>>,
        body: Vec<Ast<TypeElem>>,
    },
    Enum {
        name: Ast<Ident>,
        params: Vec<Ast<Ident>>,
        body: Vec<Ast<TypeElem>>,
    }
}

#[derive(Debug)]
pub enum TypeElem {
    Simple(Ast<Ident>, Ast<Ty>),
    Spread(Ast<Ty>),
}

#[derive(Debug)]
pub enum TupleTypeElem {
    Simple(Ast<Ty>),
    Spread(Ast<Ty>),
}
