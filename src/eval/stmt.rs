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
