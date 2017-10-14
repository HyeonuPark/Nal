use std::collections::HashSet;

use ast::common::{Ast, Ident};
use super::{Check, Ctx, Error as E};

lazy_static! {
    static ref RESERVED: HashSet<&'static str> = [
        "if", "else",
        "while", "for", "in",
        "fn", "let",
        "num", "bool",
    ].into_iter().map(|&e| e).collect();
}

impl Check for Ast<Ident> {
    fn check(&self, ctx: &mut Ctx) {
        if RESERVED.contains(&***self) {
            ctx.report(E::IdentIsKeyword(self.span));
        }
    }
}
