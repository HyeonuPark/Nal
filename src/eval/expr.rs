use ast::{Expr, Literal, BinaryOp, UnaryOp};
use eval::{Result, Value, Control, Env, Eval};

impl<'a> Eval for Expr<'a> {
  fn eval(&self, env: &mut Env) -> Result<Value> {
    use self::Expr::*;
    use self::BinaryOp::*;
    use self::UnaryOp::*;
    use self::Literal as L;
    use self::Value as V;

    match *self {
      Literal(ref value) => Ok(match *value.as_ref() {
        L::Number(value) => V::Number(value),
        L::Bool(value) => V::Bool(value),
      }),
      Binary(op, ref left, ref right) => match (left.eval(env)?, right.eval(env)?) {
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
      Identifier(ref id) => env.get(id),
      Unary(op, ref expr) => Ok(match (op, expr.eval(env)?) {
        (Neg, V::Number(value)) => V::Number(-value),
        (Not, V::Bool(value)) => V::Bool(!value),
        _ => return Err(Control::RuntimeError("TypeError".into())),
      }),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use eval::Value::*;
  use ast::Span;
  use parser::parse_expr;

  fn eval(src: &str) -> Value {
    parse_expr(Span::new(src)).unwrap().1.eval(&mut Env::default()).unwrap()
  }

  #[test]
  fn test_eval_binary_expr() {
    assert_eq!(
      eval("3+7*5-6"),
      Number(32.0)
    );
    assert_eq!(
      eval("5 + 2 * 4 * 5 * 9"),
      Number(365.0)
    );
    assert_eq!(
      eval("7 > 2 + 4"),
      Bool(true)
    );
  }

  #[test]
  fn test_eval_unary_expr() {
      assert_eq!(
        eval("-99.8p"),
        Number(-99.8)
      )
  }
}
