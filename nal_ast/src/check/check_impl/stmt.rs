use ast::common::Ast;
use ast::stmt::{Stmt, StmtBlock};

use check::{Check, Ctx, Error as E};
use super::pattern::Decl;

impl Check for Ast<Stmt> {
    fn check(&self, ctx: &mut Ctx) {
        use ast::stmt::Stmt::*;

        match **self {
            If(ref cases, ref otherwise) => {
                for &(ref cond, ref pos) in cases {
                    cond.check(ctx);
                    pos.check(ctx);
                }
                otherwise.check(ctx);
            }
            While(ref cond, ref body) => {
                cond.check(ctx);

                ctx.with_loop(|ctx| {
                    body.check(ctx);
                });
            }
            ForIn(ref each, ref seq, ref body) => {
                seq.check(ctx);

                Decl(each).check(ctx);

                ctx.with_loop(|ctx| {
                    body.check(ctx);
                });
            }
            Function(is_static, ref func) => {
                if func.name.is_none() {
                    ctx.report(E::FuncNotNamed(self.span));
                }

                if !is_static {
                    func.check(ctx);
                }
            }
            Let(ref pat, ref expr) => {
                expr.check(ctx);
                Decl(pat).check(ctx);
            }
            Assign(ref pat, ref expr) => {
                expr.check(ctx);
                pat.check(ctx);
            }
            Expr(ref expr) => {
                expr.check(ctx);
            }
        }
    }
}

impl Check for Ast<StmtBlock> {
    fn check(&self, ctx: &mut Ctx) {
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
    }
}
