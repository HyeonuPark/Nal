use std::collections::HashMap;
use std::rc::Rc;

use nal_ast::ast::prelude::{Literal as L, ObjProp as P};

use common::prelude::*;

impl Eval for Ast<L> {
    type Output = ValueRef;

    fn eval(&self, env: &mut Env) -> Result<Self::Output> {
        let res = match ***self {
            L::Num(v) => V::Num(v),
            L::Bool(v) => V::Bool(v),
            L::Str(ref v) => V::Str(v.to_string()),

            L::Obj(_) => {
                setup!(eval_obj[HashMap<_, _>], self, env, *);
                V::Obj(eval_obj!(L::Obj(ref t) => t)?)
            }
        };

        Ok(res.into())
    }
}

impl Eval for Ast<P> {
    type Output = (Rc<str>, Value);

    fn eval(&self, env: &mut Env) -> Result<Self::Output> {
        setup!(eval, self, env);

        Ok(match ***self {
            P::Named(ref ident, _) => (
                ident.name(),
                eval!(P::Named(_, ref t) => t)?.clone(),
            ),
            P::Short(ref ident) => (
                ident.name(),
                env.get(ident)?.clone(),
            ),
            P::Method(ref func) => (
                func.name.as_ref().unwrap().name(),
                eval!(P::Method(ref t) => t)?.clone(),
            ),
        })
    }
}
