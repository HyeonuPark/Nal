use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::RcRef;
use nal_ast::ast::prelude as ast;
use nal_ast::SourceBuffer;

pub use env::Env;

#[derive(Clone)]
pub enum Value {
    Unit,
    Num(f64),
    Bool(bool),
    Str(Rc<str>),
    Func(Ast<ast::Function>, Rc<Env<'static>>),
    Native(Rc<Fn(Vec<Value>) -> ::std::result::Result<Value, Error>>),
    Obj(HashMap<String, Value>),
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
                    let mut keys = table.keys()
                        .map(|k| k.clone())
                        .collect::<Vec<_>>();
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

impl From<String> for Control {
    fn from(msg: String) -> Self {
        Control::Error(msg)
    }
}

impl<'a> From<&'a str> for Control {
    fn from(msg: &'a str) -> Self {
        Control::Error(msg.into())
    }
}

pub trait Eval {
    type Output;

    fn eval(&self, env: &mut Env) -> Result<Self::Output>;
}
