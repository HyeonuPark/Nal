use std::collections::HashMap;

use ast::common::Ast;
use ast::expr::{Expr, Literal, ObjProp};
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
    fn check(&self, ctx: &mut Ctx) {
        use self::Literal as L;

        match **self {
            L::Num(_) => {}
            L::Bool(_) => {}
            L::Str(_) => {}

            L::Obj(ref items) => check_obj(items, ctx),
        }
    }
}

fn check_obj(items: &[Ast<ObjProp>], ctx: &mut Ctx) {
    use self::ObjProp as P;

    let mut decl_map: HashMap<String, _> = HashMap::new();

    let mut check_dup = |name: &Ast<::ast::common::Ident>, ctx: &mut Ctx| {
        if decl_map.contains_key(name as &str) {
            ctx.report(E::DupedPropName(
                decl_map[name as &str],
                name.span,
            ));
        } else {
            decl_map.insert(name.to_string(), name.span);
        }
    };

    for item in items {
        match **item {
            P::Named(ref name, ref value) => {
                name.check(ctx);
                value.check(ctx);

                check_dup(name, ctx);
            }
            P::Short(ref name) => {
                let name2 = name.clone();
                let expr = name.clone().map(|_| Ident(name2));
                expr.check(ctx);

                check_dup(name, ctx);
            }
            P::Method(ref func) => {
                func.check(ctx);

                if let Some(ref name) = func.name {
                    check_dup(name, ctx);
                } else {
                    ctx.report(E::FuncNotNamed(item.span));
                }
            }
        }
    }
}
