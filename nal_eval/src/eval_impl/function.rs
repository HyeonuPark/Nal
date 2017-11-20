
use nal_ast::ast::prelude::{Function, FunctionBody as FB};

use common::prelude::*;
use super::pattern::decl_pattern;

impl Eval for Function {
    type Output = Value;

    fn eval(&self, env: &mut Env) -> Result<Value> {
        Ok(env.get_fn(self))
    }
}

pub fn eval_call(callee: ValueRef, args: Vec<ValueRef>) -> Result<Value> {
    match *callee {
        V::Func(ref func, ref env) => {
            if func.params.len() != args.len() {
                Err(format!("Param mismatch - this function requires {} \
                parameters, but {} provided", func.params.len(), args.len()))?;
            }

            let env = &mut env.child();

            func.params.iter()
                .zip(args)
                .map(|(param, arg)| decl_pattern(env, param, arg.clone()))
                .collect::<Result<Vec<_>>>()?;

            match func.body {
                FB::Stmt(ref stmt) => {
                    match stmt.eval(env) {
                        Ok(_) => Ok(Value::Unit),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
                FB::Expr(ref expr) => {
                    match expr.eval(env) {
                        Ok(v) => Ok(v.clone()),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
            }
        }
        V::Native(ref func) => {
            func(args.into_iter().map(|v| v.into()).collect())
                .map_err(Control::Error)
        }
        _ => {
            Err(format!("Invalid type - Callee should be function type, \
            but found {:#?}", *callee))?
        }
    }
}
