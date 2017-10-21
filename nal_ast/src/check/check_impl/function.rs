use ast::common::Ast;
use ast::function::{Function, FunctionBody as FB};

use check::{Check, Ctx};
use super::pattern::check_pattern_decl;

impl Check for Ast<Function> {
    fn check(&self, ctx: &mut Ctx) {
        self.name.check(ctx);

        ctx.subscope(|ctx| {
            for param in &self.params {
                check_pattern_decl(param, ctx);
            }

            ctx.with_fn(|ctx| {
                match self.body {
                    FB::Stmt(ref stmt) => {
                        stmt.check(ctx);
                    }
                    FB::Expr(ref expr) => {
                        expr.check(ctx);
                    }
                }
            });
        });
    }
}
