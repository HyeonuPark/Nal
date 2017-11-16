use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::RcRef;
use nal_ast::ast::prelude as ast;
use nal_ast::SourceBuffer;

use env::Env;

pub mod prelude {
    pub use env::Env;
    pub use value_ref::{ValueRef, ValueRefMut};
    pub use super::{Value, Control, Error, Result, Ast, Eval};
    pub use super::Value as V;
}

pub trait Eval {
    type Output;

    fn eval(&self, env: &mut Env) -> Result<Self::Output>;
}

#[derive(Debug)]
pub enum Control {
    Error(Error),
    Return(Value),
    Break,
    Continue,
}

pub type Error = String;
pub type Result<T> = ::std::result::Result<T, Control>;
pub type Ast<T> = RcRef<SourceBuffer, ast::Ast<T>>;

impl<T: Into<Error>> From<T> for Control {
    fn from(err: T) -> Self {
        Control::Error(err.into())
    }
}

#[derive(Clone)]
pub enum Value {
    Unit,
    Num(f64),
    Bool(bool),
    Str(String),
    Func(Ast<ast::Function>, Rc<Env<'static>>),
    Native(Rc<Fn(Vec<Value>) -> ::std::result::Result<Value, Error>>),
    Obj(HashMap<Rc<str>, Value>),
}

impl Value {
    pub fn set(&mut self, other: Value) {
        *self = other;
    }
}

mod dbg {
    use super::Value;
    use super::Value::*;
    use std::fmt::{Debug, Formatter, Error};

    impl Debug for Value {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            match *self {
                Unit => write!(f, "Unit"),
                Num(n) => write!(f, "Num({})", n),
                Bool(b) => write!(f, "Bool({})", b),
                Str(ref s) => write!(f, "Str({})", s),
                Func(ref func, _) => match func.name {
                    Some(ref name) => write!(f, "fn {}() {{ .. }}", name as &str),
                    None => write!(f, "fn() {{ .. }}"),
                }
                Native(_) => write!(f, "fn {{ native }}"),
                Obj(ref table) => {
                    let mut keys: Vec<_> = table.keys().cloned().collect();
                    keys.sort();

                    let mut f = f.debug_struct("Obj");

                    for key in &keys {
                        f.field(key, &table[key]);
                    }

                    f.finish()
                }
            }
        }
    }
}
