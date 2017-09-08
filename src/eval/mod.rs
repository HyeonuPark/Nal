
mod env;
mod expr;
mod stmt;

pub use self::env::Env;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  None,
  Number(f64),
  Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Control {
  Return(Value),
  Break,
  Continue,
  RuntimeError(String),
}

pub trait Eval {
  fn eval(&self, env: &mut Env) -> Result<Value>;
}

pub trait Exec {
  fn exec(&self, env: &mut Env) -> Result<()>;
}

pub type Result<T> = ::std::result::Result<T, Control>;
