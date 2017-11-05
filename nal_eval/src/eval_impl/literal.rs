use std::collections::HashMap;

use nal_ast::ast::prelude::{Literal as L, ObjProp};

use common::{Eval, Env, Ast, Value as V, Result};

impl Eval for Ast<L> {
    type Output = V;

    fn eval(&self, env: &mut Env) -> Result<V> {
        setup!(eval_obj[HashMap<_, _>], self, env, *);

        Ok(match ***self {
            L::Num(v) => V::Num(v),
            L::Bool(v) => V::Bool(v),
            L::Str(ref v) => V::Str((v as &str).into()),
            L::Obj(_) => V::Obj(eval_obj!(L::Obj(ref t) => t)?),
        })
    }
}

impl Eval for Ast<ObjProp> {
    type Output = (String, V);

    fn eval(&self, env: &mut Env) -> Result<(String, V)> {
        use self::ObjProp as P;
        setup!(eval, self, env);

        Ok(match ***self {
            P::Named(ref name, _) => (
                name.to_string(),
                eval!(P::Named(_, ref t) => t)?,
            ),
            P::Short(ref name) => (
                name.to_string(),
                env.get(name as &str)?,
            ),
            P::Method(ref func) => (
                func.name.as_ref().unwrap().to_string(),
                eval!(P::Method(ref t) => t)?,
            ),
        })
    }
}
