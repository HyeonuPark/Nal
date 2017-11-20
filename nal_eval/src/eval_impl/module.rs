use nal_ast::ast::prelude::{Module, ModuleStmt};

use common::prelude::*;

impl Eval for Module {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        for stmt in &self.body {
            stmt.eval(env)?;
        }

        Ok(())
    }
}

impl Eval for ModuleStmt {
    type Output = ();

    fn eval(&self, env: &mut Env) -> Result<()> {
        use self::ModuleStmt as M;

        match *self {
            M::Stmt(ref stmt) => stmt.eval(env)?,
        }

        Ok(())
    }
}
