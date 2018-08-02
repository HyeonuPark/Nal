
use std::sync::Arc;
use std::collections::HashMap;

use obj::{Obj, Ref, RefMut};

#[derive(Debug)]
pub enum ObjKind {
    Empty,
    Value(Value),
    Table(HashMap<Ident, Obj>),
    Tuple(Vec<Obj>),
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Num(f64),
    Ref(Ref),
    RefMut(RefMut),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Ident(Arc<str>);

pub type IResult<T> = Result<T, IError>;

#[derive(Debug)]
pub enum IError {
    PopRootScope,
    ScopeEnded,
    StackOverflow,
    VarNotInit,
    RefOverflow,
    ObjBorrowed,
    RefMutBorrowed,
}
