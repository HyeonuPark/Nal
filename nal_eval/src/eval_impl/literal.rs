use std::collections::HashMap;

use nal_ast::ast::prelude::{Literal, ObjProp as P};

use common::prelude::*;

use self::Literal as L;

impl Eval for Literal {
    type Output = Value;

    fn eval(&self, env: &mut Env) -> Result<Value> {
        Ok(match *self {
            L::Num(v) => V::Num(v),
            L::Bool(v) => V::Bool(v),
            L::Str(ref v) => V::Str(v.to_string()),

            L::Obj(ref props) => {
                let table = props.iter()
                    .map(|prop| Ok(match **prop {
                        P::Named(ref ident, ref expr) => (
                            ident.name(),
                            expr.eval(env)?.into(),
                        ),
                        P::Short(ref ident) => (
                            ident.name(),
                            env.get(ident)?.clone(),
                        ),
                        P::Method(ref func) => (
                            func.name.as_ref().unwrap().name(),
                            func.eval(env)?,
                        ),
                    }))
                    .collect::<Result<HashMap<_, _>>>()?;

                V::Obj(table)
            }
        })
    }
}
