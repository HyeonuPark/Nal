use ast::common::Ast;
use ast::stmt::{Stmt, StmtBlock};

use super::{Check, Ctx};
use super::pattern::{check_pattern_decl, check_pattern_assign};

impl Check for Ast<Stmt> {
    fn check(&self, ctx: &mut Ctx) {
        use ast::stmt::Stmt::*;

        match **self {
            If(ref cond, ref pos, ref neg) => {
                cond.check(ctx);
                pos.check(ctx);
                neg.check(ctx);
            }
            While(ref cond, ref body) => {
                cond.check(ctx);
                body.check(ctx);
            }
            ForIn(ref each, ref seq, ref body) => {
                seq.check(ctx);

                ctx.subscope(|ctx| {
                    check_pattern_decl(each, ctx);
                    body.check(ctx);
                });
            }
            Function(is_static, ref func) => {
                if !is_static {
                    func.check(ctx);
                }
            }
            Let(ref pat, ref expr) => {
                expr.check(ctx);
                check_pattern_decl(pat, ctx);
            }
            Assign(ref pat, ref expr) => {
                expr.check(ctx);
                check_pattern_assign(pat, ctx);
            }
            Expr(ref expr) => {
                expr.check(ctx);
            }
        }
    }
}

impl Check for Ast<StmtBlock> {
    fn check(&self, ctx: &mut Ctx) {
        ctx.subscope(|ctx| {
            for stmt in self.iter() {
                match **stmt {
                    Stmt::Function(is_static, ref func) if is_static => {
                        func.check(ctx);
                    }
                    _ => {}
                }
            }

            for stmt in self.iter() {
                stmt.check(ctx);
            }
        });
    }
}
