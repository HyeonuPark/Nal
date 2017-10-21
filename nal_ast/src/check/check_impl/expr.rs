use ast::common::Ast;
use ast::expr::{Expr, Literal};
use self::Expr::*;

use check::{Check, Ctx};

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
        // Is there something needed to check?
    }
}
