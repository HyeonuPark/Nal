
use super::Ty;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Primitive {
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
}

pub fn wrap(primitive: Primitive) -> Ty {
    unimplemented!()
}
