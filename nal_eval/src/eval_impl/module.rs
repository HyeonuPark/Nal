use owning_ref::RcRef;

use nal_ast::SourceBuffer;
use nal_ast::ast::prelude::ModuleStmt;

use common::{Eval, Env, Ast, Result};

impl Eval for RcRef<SourceBuffer> {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        for i in 0..self.body.len() {
            self.clone().map(|srcbuf| &(*srcbuf).body[i]).eval(env)?
        }

        Ok(())
    }
}

impl Eval for Ast<ModuleStmt> {
    type Output = ();

    #[allow(unreachable_patterns)]
    fn eval(&self, env: &mut Env) -> Result<()> {
        setup_mapto!(mapto, self, env);
        match ***self {
            ModuleStmt::Stmt(_) => mapto!(ModuleStmt::Stmt(ref t) => t),
        }
    }
}
