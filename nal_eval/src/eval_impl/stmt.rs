use nal_ast::ast::prelude::{Stmt};

use common::{Eval, Env, Ast, Value, Result};
use super::pattern::{decl_pattern, assign_pattern};

use self::Stmt::*;

impl Eval for Ast<Stmt> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        setup_mapto!(mapto, self, env);
        setup_mapto!(mapto_vec[], self, &mut env.child());

        match ***self {
            If(_, _, ref else_block) => match mapto!(If(ref t, _, _) => t)? {
                Value::Bool(true) => {
                    mapto_vec!(If(_, ref t, _) => t)?;
                }
                Value::Bool(false) => {
                    if else_block.is_some() {
                        mapto_vec!(If(_, _, ref t) => t.as_ref().unwrap())?;
                    }
                }
                _ => {
                    Err("TypeError - If condition should be bool type")?;
                }
            }
            While(_, _) => {
                while match mapto!(While(ref t, _) => t)? {
                    Value::Bool(b) => b,
                    _ => {
                        Err("TypeError - While condition should be bool type")?;
                        unreachable!()
                    }
                } {
                    mapto_vec!(While(_, ref t) => t)?;
                }
            }
            ForIn(_, _, _) => {
                Err("ForIn stmt will not be supported until complex types are implemented")?;
            }
            Function(is_static, ref func) => {
                if !is_static {
                    let v = mapto!(Function(_, ref t) => t)?;
                    env.decl(func.name.as_ref().unwrap(), v);
                }
            }
            Let(ref pat, _) => {
                let v = mapto!(Let(_, ref t) => t)?;
                decl_pattern(env, pat, v)?;
            }
            Assign(ref pat, _) => {
                let v = mapto!(Assign(_, ref t) => t)?;
                assign_pattern(env, pat, v)?;
            }
            Expr(_) => {
                mapto!(Expr(ref t) => t).map(|_| ())?;
            }
        }

        Ok(())
    }
}
