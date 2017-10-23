use std::rc::Rc;

use owning_ref::RcRef;
use nal_ast::ast::prelude as ast;
use nal_ast::SourceBuffer;

pub use env::Env;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Num(f64),
    Bool(bool),
    Func(Ast<ast::Function>, Rc<Env<'static>>),
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
