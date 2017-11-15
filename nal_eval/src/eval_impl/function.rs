
use nal_ast::ast::prelude::{Function, FunctionBody as FB};

use common::prelude::*;
use super::pattern::decl_pattern;

impl Eval for Ast<Function> {
    type Output = ValueRef;

    fn eval(&self, env: &mut Env) -> Result<ValueRef> {
        Ok(Value::Func(self.clone(), env.deep_clone().into()).into())
    }
}

pub fn eval_call<'a>(callee: ValueRef, args: Vec<ValueRef>) -> Result<Value> {
    match *callee {
        Value::Func(ref func, ref env) => {
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
                FB::Stmt(_) => {
                    setup!(eval[], func, &mut env.child(), *);

                    let res = eval!(Function {
                        name: _, params: _,
                        body: FB::Stmt(ref t),
                    } => t);

                    match res {
                        Ok(_) => Ok(Value::Unit),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
                FB::Expr(_) => {
                    setup!(eval, func, &mut env.child());

                    let res = eval!(Function {
                        name: _, params: _,
                        body: FB::Expr(ref t),
                    } => t);

                    match res {
                        Ok(v) => Ok(v.clone()),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
            }
        }
        Value::Native(ref func) => {
            func(args.into_iter().map(|v| v.clone()).collect())
                .map_err(Control::Error)
        }
        _ => {
            Err(format!("Invalid type - Callee should be function type, \
            but found {:#?}", *callee))?
        }
    }
}
