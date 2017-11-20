use common::prelude::*;
use super::pattern::{decl_pattern, assign_pattern};

use self::ast::{Ast, Stmt};
use self::Stmt as S;

impl Eval for Stmt {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        match *self {
            S::If(ref cases, ref otherwise) => {
                for &(ref cond, ref body) in cases {
                    match *cond.eval(env)? {
                        V::Bool(true) => {
                            body.eval(env)?;
                            return Ok(());
                        }
                        V::Bool(false) => {}
                        _ => Err("Invalie type - Condition should be Bool type")?,
                    }
                }

                if let Some(ref body) = *otherwise {
                    body.eval(env)?;
                }
            }
            S::While(ref cond, ref body) => loop {
                match *cond.eval(env)? {
                    V::Bool(true) => body.eval(env)?,
                    V::Bool(false) => break,
                    _ => Err("Invalid type - Condition should be Bool type")?,
                }
            }
            S::ForIn(_, _, _) => Err("ForIn stmt is not supported yet")?,
            S::Function(is_static, ref func) => {
                if !is_static {
                    let v = func.eval(env)?;
                    env.decl(func.name.as_ref().unwrap().name(), v);
                }
            }
            S::Let(ref pat, ref expr) => {
                let v = expr.eval(env)?;
                decl_pattern(env, pat, v.into())?;
            }
            S::Assign(ref pat, ref expr) => {
                let v= expr.eval(env)?;
                assign_pattern(env, pat, v.into())?;
            }
            S::Expr(ref expr) => {
                expr.eval(env)?;
            }
        }

        Ok(())
    }
}

impl Eval for [Ast<Stmt>] {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        for stmt in self {
            match **stmt {
                S::Function(is_static, ref func) if is_static => {
                    let name = func.name.as_ref().unwrap().name();
                    let v = func.eval(&mut env.child())?;
                    env.decl(name, v);
                }
                _ => {}
            }
        }

        for stmt in self {
            stmt.eval(env)?;
        }

        Ok(())
    }
}
