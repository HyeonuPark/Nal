use ast::literal::Value;

#[derive(Debug)]
pub enum Control {
  Return(Value),
  Break,
  Continue,
  RuntimeError(String),
}

pub type Result<T> = ::std::result::Result<T, Control>;
