use nal_ast::ast::prelude::{Literal as L};

use common::{Eval, Env, Ast, Value as V, Result};

impl Eval for Ast<L> {
    type Output = V;

    fn eval(&self, _env: &mut Env) -> Result<V> {
        Ok(match ***self {
            L::Num(v) => V::Num(v),
            L::Bool(v) => V::Bool(v),
        })
    }
}
