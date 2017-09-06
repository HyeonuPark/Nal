use std::ops::Deref;

use ast::{Expr, Literal, BinaryOp};
use eval::{Result, Value, Control, Env};

impl<'a> Expr<'a> {
  pub fn evaluate(&self, env: &mut Env) -> Result<Value> {
    use self::Expr::*;
    use self::BinaryOp::*;
    use self::Literal as L;
    use self::Value as V;

    match *self {
      Literal(ref value) => Ok(match *value.deref() {
        L::Number(value) => V::Number(value),
        L::Bool(value) => V::Bool(value),
      }),
      Binary(op, ref left, ref right) => match (left.evaluate(env)?, right.evaluate(env)?) {
        (V::Number(left), V::Number(right)) => Ok(match op {
          Add => V::Number(left + right),
          Sub => V::Number(left - right),
          Mul => V::Number(left * right),
          Div => V::Number(left / right),
          Eq  => V::Bool(left == right),
          Neq => V::Bool(left != right),
          Gt  => V::Bool(left > right),
          Gte => V::Bool(left >= right),
          Lt  => V::Bool(left < right),
          Lte => V::Bool(left <= right),
          _ => return Err(Control::RuntimeError("TypeError".into())),
        }),
        (V::Bool(left), V::Bool(right)) => Ok(match op {
          Eq  => V::Bool(left == right),
          Neq => V::Bool(left != right),
          And => V::Bool(left && right),
          Or  => V::Bool(left || right),
          _ => return Err(Control::RuntimeError("TypeError".into())),
        }),
        _ => Err(Control::RuntimeError("Cannot calculate bool or None".into())),
      },
      Identifier(id) => env.get(id)
    }
  }
}

#[cfg(test)]
mod test {
  use eval::Value::*;
  use ast::Span;
  use parser::parse_expr;

  fn d<D: Default>() -> D {
    Default::default()
  }

  #[test]
  fn test_evaluate_binary_expr() {
    assert_eq!(
      parse_expr(Span::new("3+7*5-6")).unwrap().1.evaluate(&mut d()).unwrap(),
      Number(32.0)
    );
    assert_eq!(
      parse_expr(Span::new("5 + 2 * 4 * 5 * 9")).unwrap().1.evaluate(&mut d()).unwrap(),
      Number(365.0)
    );
    assert_eq!(
      parse_expr(Span::new("7 > 2 + 4")).unwrap().1.evaluate(&mut d()).unwrap(),
      Bool(true)
    );
  }
}
