use nal_ast::ast::prelude::Stmt;

use common::prelude::*;
use super::pattern::{decl_pattern, assign_pattern};

impl Eval for Ast<Stmt> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        use self::Stmt as S;

        setup!(eval, self, env);
        setup!(eval_block[], self, &mut env.child(), *);

        match ***self {
            S::If(_, _, ref on_else) => match *eval!(S::If(ref t, _, _) => t)? {
                V::Bool(true) => {
                    eval_block!(S::If(_, ref t, _) => t)?;
                }
                V::Bool(false) => {
                    if on_else.is_some() {
                        eval_block!(S::If(_, _, ref t) => t.as_ref().unwrap())?;
                    }
                }
                _ => Err("Invalid type - If condition should be bool type")?,
            }
            S::While(_, _) => match *eval!(S::While(ref t, _) => t)? {
                V::Bool(true) => {
                    eval_block!(S::While(_, ref t) => t)?;
                }
                V::Bool(false) => (),
                _ => Err("Invalid type - While condition should be bool type")?,
            }
            S::ForIn(_, _, _) => {
                Err("ForIn stmt is not supported yet")?
            }
            S::Function(is_static, ref func) => {
                if !is_static {
                    let v = eval!(S::Function(_, ref t) => t)?;
                    env.decl(func.name.as_ref().unwrap().name(), v.clone());
                }
            }
            S::Let(ref pat, _) => {
                let v = eval!(S::Let(_, ref t) => t)?;
                decl_pattern(env, pat, v.clone())?;
            }
            S::Assign(ref pat, _) => {
                let v = eval!(S::Assign(_, ref t) => t)?;
                assign_pattern(env, pat, v.clone())?;
            }
            S::Expr(_) => {
                eval!(S::Expr(ref t) => t)?;
            }
        }

        Ok(())
    }
}
