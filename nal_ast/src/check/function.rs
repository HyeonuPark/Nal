use ast::common::Ast;
use ast::function::{Function, FunctionBody as FB};

use super::{Check, Ctx};
use super::pattern::check_pattern_decl;

impl Check for Ast<Function> {
    fn check(&self, ctx: &mut Ctx) {
        self.name.check(ctx);

        for param in self.params.iter() {
            check_pattern_decl(param, ctx);
        }

        match self.body {
            FB::Stmt(ref stmt) => {
                stmt.check(ctx);
            }
            FB::Expr(ref expr) => {
                expr.check(ctx);
            }
        }
    }
}
