use std::rc::Rc;

pub type Ty = ();

pub struct Value(usize);
pub struct Ident(Rc<str>);
pub struct Index(usize);

pub struct VarName(Ident, usize);
pub struct Constant(Value);
