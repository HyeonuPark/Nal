use ast::common::Ast;
use ast::expr::{Expr, Literal};
use self::Expr::*;

use check::{Check, Ctx, Error as E};

impl Check for Ast<Expr> {
    fn check(&self, ctx: &mut Ctx) {
        match **self {
            Literal(ref literal) => {
                literal.check(ctx);
            }
            Binary(_, ref left, ref right) => {
                left.check(ctx);
                right.check(ctx);
            }
            Unary(_, ref expr) => {
                expr.check(ctx);
            }
            Call(ref callee, ref args) => {
                callee.check(ctx);

                for expr in args {
                    expr.check(ctx);
                }
            }
            Return(ref expr) => {
                expr.check(ctx);

                if !ctx.is_fn() {
                    ctx.report(E::ContextNotFound(self.span));
                }
            }
            Break | Continue => {
                if !ctx.is_loop() {
                    ctx.report(E::ContextNotFound(self.span));
                }
            }
            Function(ref func) => {
                func.check(ctx);
            }
            Ident(ref ident) => {
                ident.check(ctx);

                ctx.exist(ident);
            }
        }
    }
}

impl Check for Ast<Literal> {
    fn check(&self, _ctx: &mut Ctx) {
        match **self {
            Literal::Num(_) => {}
            Literal::Bool(_) => {}
            Literal::Str(_) => {}
        }
    }
}
