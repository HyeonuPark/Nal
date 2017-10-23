use std::rc::Rc;

use nal_ast::ast::prelude::{Function};

use common::{Eval, Env, Ast, Value, Result};

impl Eval for Ast<Function> {
    type Output = Value;

    fn eval(&self, env: &mut Env) -> Result<Value> {
        let env: Env = env.clone();
        Ok(Value::Func(self.clone(), Rc::new(env)))
    }
}
