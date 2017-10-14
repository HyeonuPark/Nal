use ast::prelude::*;

use super::{Check, Ctx};

impl Check for Module {
    fn check(&self, ctx: &mut Ctx) {
        for stmt in self.body.iter() {
            stmt.check(ctx);
        }
    }
}

impl Check for Ast<ModuleStmt> {
    fn check(&self, ctx: &mut Ctx) {
        use self::ModuleStmt as M;

        match **self {
            M::Stmt(ref stmt) => stmt.check(ctx),
        }
    }
}
