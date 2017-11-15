use owning_ref::RcRef;

use nal_ast::SourceBuffer;
use nal_ast::ast::prelude::ModuleStmt;

use common::prelude::*;

impl Eval for RcRef<SourceBuffer> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        for i in 0..self.module.body.len() {
            self.clone().map(|sb| &sb.module.body[i]).eval(env)?;
        }

        Ok(())
    }
}

impl Eval for Ast<ModuleStmt> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        use self::ModuleStmt as M;

        setup!(eval, self, env);

        match ***self {
            M::Stmt(_) => eval!(M::Stmt(ref t) => t)?,
        }

        Ok(())
    }
}
