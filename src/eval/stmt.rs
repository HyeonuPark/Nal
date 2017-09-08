use eval::{Exec, Eval, Env, Result};
use ast::Stmt;

impl<'a> Exec for Stmt<'a> {
  fn exec(&self, env: &mut Env) -> Result<()> {
    use self::Stmt::*;

    Ok(match *self {
      Expr(ref expr) => {
        expr.eval(env)?;
      },
      Let(ref name, is_mut, ref expr) => {
        if is_mut {
          let value = expr.eval(env)?;
          env.declare_mut(name.clone(), value);
        } else {
          let value = expr.eval(env)?;
          env.declare(name.clone(), value);
        }
      },
      Assign(ref name, ref expr) => {
        let value = expr.eval(env)?;
        env.set(name, value)?;
      },
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use eval::Value;
  use eval::Value::*;
  use ast::Span;
  use parser::parse_stmt_block;

  fn eval(src: &str) -> Value {
    let block = parse_stmt_block(Span::new(src)).unwrap().1;
    let env = &mut Env::new();

    assert!(block.len() > 0);

    for stmt in &block[..block.len() - 1] {
      stmt.exec(env).unwrap();
    }

    match block.last().unwrap().as_ref() {
      &Stmt::Expr(ref expr) => expr.eval(env).unwrap(),
      _ => Value::None,
    }
  }

  #[test]
  fn test_eval_stmt() {
    assert_eq!(
      eval("{
        let foo = 3
        let bar = 4
        foo + bar
      }"),
      Number(7.0)
    );

    assert_eq!(
      eval("{
        let foo = 3

        let foo = 4 < 5
        foo
      }"),
      Bool(true)
    );
  }
}
