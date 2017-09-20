use eval::{Exec, Eval, Env, Result, Value, Control};
use ast::{Ast, Stmt};

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
      If(ref cond, ref on_true, ref on_false) => {
        match cond.eval(env)? {
          Value::Bool(true) => on_true.exec(env)?,
          Value::Bool(false) => if let &Some(ref stmt) = on_false {
            stmt.exec(env)?;
          },
          _ => return Err(Control::RuntimeError("TypeError".into())),
        };
      }
    })
  }
}

impl<'a> Exec for Vec<Ast<'a, Stmt<'a>>> {
  fn exec(&self, env: &mut Env) -> Result<()> {
    let env = &mut env.clone();

    for stmt in self {
      stmt.exec(env)?
    }

    Ok(())
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
      stmt => {
        stmt.exec(env).unwrap();
        Value::None
      }
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

  #[test]
  fn test_eval_if_stmt() {
      assert_eq!(
        eval("{
          let mut foo = 3

          if true {
            foo = 5
          }

          foo
        }"),
        Number(5.0)
      );
      assert_eq!(
        eval("{
          let mut foo = 0

          if 3 > 4 {
            foo = 3
          } else {
            foo = 4
          }

          foo
        }"),
        Number(4.0)
      );
  }
}
