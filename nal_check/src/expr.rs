use nal_ast::ast::common::Ast;
use nal_ast::ast::expr::Expr;

use common::{Check, Scope, Acc};

impl Check for Ast<Expr> {
    fn check(&self, scope: Scope, acc: Acc) {
        use self::Expr::*;

        match **self {
            Binary(_, ref left, ref right) => {
                left.check(scope, acc);
                right.check(scope, acc);
            }
            Unary(_, ref expr) => {
                expr.check(scope, acc);
            }
            Ident(ref ident) => {
                ident.check(scope, acc);

                if let Err(err) = scope.exist(&***ident) {
                    acc.push((err, self.span));
                }
            }
            Literal(_) => {}
        };
    }
}
