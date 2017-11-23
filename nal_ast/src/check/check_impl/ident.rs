use std::collections::HashSet;

use ast::common::{Ast, Ident};
use check::{Check, Ctx, Error as E};

lazy_static! {
    static ref RESERVED: HashSet<&'static str> = [
        "let", "mut", "as",
        "if", "else", "while", "for",
        "in", "break", "continue",
        "static", "fn", "return",
        "import", "export",
        "type", "enum", "const",
        "trait", "private",
        "try", "do", "yield",
        "match", "and", "or",
        "true", "false",
    ].into_iter().map(|&e| e).collect();
}

impl Check for Ast<Ident> {
    fn check(&self, ctx: &mut Ctx) {
        if RESERVED.contains(self as &str) {
            ctx.report(E::IdentIsKeyword(self.span));
        }
    }
}
