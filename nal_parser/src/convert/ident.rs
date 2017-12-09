use std::collections::HashSet;

use codebuf::Node;
use nal_ast::ast;
use parse_tree as pt;
use super::{Convert, Ctx, Error as E};

impl Convert<Node<ast::Ident>> for Node<pt::Ident> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Node<ast::Ident>, ()> {
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
        };

        let code = ctx.buf.span(self);

        if RESERVED.contains(code) {
            ctx.errors.push(E::UseReservedKeyword(self.span));
            Err(())
        } else {
            Ok(Node::new(self.span, ast::Ident(code.into())))
        }
    }
}
