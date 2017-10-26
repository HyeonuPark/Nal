use std::rc::Rc;

use nal_ast::ast::prelude::{Function, FunctionBody as FB};

use common::{Eval, Env, Ast, Value, Control, Result};
use super::pattern::decl_pattern;

impl Eval for Ast<Function> {
    type Output = Value;

    fn eval(&self, env: &mut Env) -> Result<Value> {
        let env: Env = env.clone();
        Ok(Value::Func(self.clone(), Rc::new(env)))
    }
}

pub fn eval_call(callee: Value, args: Vec<Value>) -> Result<Value> {
    match callee {
        Value::Func(func, env) => {
            if func.params.len() != args.len() {
                Err(format!("Param mismatch - this function requires {} \
                parameters, but {} provided", func.params.len(), args.len()))?;
            }

            let env = &mut env.child();

            func.params.iter()
                .zip(args)
                .map(|(param, arg)| decl_pattern(env, param, arg))
                .collect::<Result<Vec<_>>>()?;

            match func.body {
                FB::Stmt(_) => {
                    setup_mapto!(mapto_vec[], func, env);

                    let res = mapto_vec!(Function {
                        name: _, params: _,
                        body: FB::Stmt(ref t)
                    } => t);

                    match res {
                        Ok(_) => Ok(Value::Unit),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
                FB::Expr(_) => {
                    setup_mapto!(mapto, func, env);

                    let res = mapto!(Function {
                        name: _, params: _,
                        body: FB::Expr(ref t)
                    } => t);

                    match res {
                        Ok(v) => Ok(v),
                        Err(Control::Return(v)) => Ok(v),
                        Err(Control::Error(e)) => Err(Control::Error(e)),
                        _ => unreachable!(),
                    }
                }
            }
        }

        Value::Native(func) => func(args).map_err(|e| Control::Error(e)),

        _ => {
            Err("You can only call functions")?;
            unreachable!()
        }
    }
}
