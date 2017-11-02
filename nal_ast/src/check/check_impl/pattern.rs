use ast::common::Ast;
use ast::stmt::Pattern;

use check::{Check, Ctx, DeclInfo, Error as E};

impl Check for Ast<Pattern> {
    fn check(&self, ctx: &mut Ctx) {
        use self::Pattern as P;

        match **self {
            P::Ident(ref ident, is_mut) => {
                ident.check(ctx);

                if is_mut {
                    ctx.report(E::AssignMutPattern(self.span));
                }

                ctx.exist_mut(ident);
            }
        }
    }
}

#[derive(Debug)]
pub struct Decl<'a>(pub &'a Ast<Pattern>);

impl<'a> Check for Decl<'a> {
    fn check(&self, ctx: &mut Ctx) {
        use self::Pattern as P;

        match **self.0 {
            P::Ident(ref ident, is_mut) => {
                ident.check(ctx);

                ctx.insert(ident, DeclInfo::new(ident.span).set_mut(is_mut));
            }
        }
    }
}
