
mod expr;

#[derive(Debug, PartialEq)]
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

pub type Result<T> = ::std::result::Result<T, Control>;