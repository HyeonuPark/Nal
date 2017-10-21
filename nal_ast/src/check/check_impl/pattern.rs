use ast::common::Ast;
use ast::stmt::Pattern;

use check::{Check, Ctx, DeclInfo, Error as E};

pub fn check_pattern_decl(pat: &Ast<Pattern>, ctx: &mut Ctx) {
    use self::Pattern as P;

    match **pat {
        P::Ident(ref ident, is_mut) => {
            ident.check(ctx);

            ctx.insert(ident, DeclInfo::new(ident.span).set_mut(is_mut));
        }
    }
}

pub fn check_pattern_assign(pat: &Ast<Pattern>, ctx: &mut Ctx) {
    use self::Pattern as P;

    match **pat {
        P::Ident(ref ident, is_mut) => {
            ident.check(ctx);

            if is_mut {
                ctx.report(E::AssignMutPattern(pat.span));
            }

            ctx.exist_mut(ident);
        }
    }
}
