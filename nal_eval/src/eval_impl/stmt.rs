use nal_ast::ast::prelude::Stmt;

use common::{Eval, Env, Ast, Value, Result};
use super::pattern::{decl_pattern, assign_pattern};

use self::Stmt::*;

impl Eval for Ast<Stmt> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        setup!(eval, self, env);
        setup!(eval_block[], self, &mut env.clone(), *);

        match ***self {
            If(_, _, ref else_case) => match eval!(If(ref t, _, _) => t)? {
                Value::Bool(true) => {
                    eval_block!(If(_, ref t, _) => t)?;
                }
                Value::Bool(false) => {
                    if else_case.is_some() {
                        eval_block!(If(_, _, ref t) => t.as_ref().unwrap())?;
                    }
                }
                _ => {
                    Err("TypeError - If condition should be bool type")?;
                }
            }
            While(_, _) => {
                while match eval!(While(ref t, _) => t)? {
                    Value::Bool(b) => b,
                    _ => {
                        Err("TypeError - While condition should be bool type")?;
                        unreachable!()
                    }
                } {
                    eval_block!(While(_, ref t) => t)?;
                }
            }
            ForIn(_, _, _) => {
                Err("ForIn stmt will not be supported until complex types are implemented")?;
            }
            Function(is_static, ref func) => {
                if !is_static {
                    let v = eval!(Function(_, ref t) => t)?;
                    env.decl(func.name.as_ref().unwrap(), v);
                }
            }
            Let(ref pat, _) => {
                let v = eval!(Let(_, ref t) => t)?;
                decl_pattern(env, pat, v)?;
            }
            Assign(ref pat, _) => {
                let v = eval!(Assign(_, ref t) => t)?;
                assign_pattern(env, pat, v)?;
            }
            Expr(_) => {
                eval!(Expr(ref t) => t).map(|_| ())?;
            }
        }

        Ok(())
    }
}
