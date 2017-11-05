use std::collections::HashSet;

use ast::common::{Ast, Ident};
use check::{Check, Ctx, Error as E};

lazy_static! {
    static ref RESERVED: HashSet<&'static str> = [
        "if", "else",
        "while", "for", "in",
        "static", "fn", "let", "mut",
        "num", "bool",
        "return", "break", "continue",
    ].into_iter().map(|&e| e).collect();
}

impl Check for Ast<Ident> {
    fn check(&self, ctx: &mut Ctx) {
        if RESERVED.contains(self as &str) {
            ctx.report(E::IdentIsKeyword(self.span));
        }
    }
}
