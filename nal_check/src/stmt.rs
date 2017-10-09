use nal_ast::ast::common::Ast;
use nal_ast::ast::stmt::{Stmt, Pattern, StmtBlock};

use scope::DeclInfo;
use common::{Check, Scope, Acc};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    MutInAssignStmt,
}

fn insert_pattern(pat: &Ast<Pattern>, scope: Scope, acc: Acc) {
    match **pat {
        Pattern::Ident(ref ident, is_mut) => {
            ident.check(scope, acc);
            scope.insert(
                &**ident,
                DeclInfo::new(pat.span).set_mut(is_mut)
            );
        }
    }
}

fn check_pattern(pat: &Ast<Pattern>, scope: Scope, acc: Acc) {
    match **pat {
        Pattern::Ident(ref ident, is_mut) => {
            if is_mut {
                acc.push((Error::MutInAssignStmt.into(), pat.span));
            }
            ident.check(scope, acc);

            if let Err(err) = scope.exist_mut(&***ident) {
                acc.push((err, pat.span));
            }
        }
    }
}

impl Check for Ast<Stmt> {
    fn check(&self, scope: Scope, acc: Acc) {
        use self::Stmt::*;

        match **self {
            Let(ref pat, ref expr) => {
                expr.check(scope, acc);
                insert_pattern(pat, scope, acc);
            }
            Assign(ref pat, ref expr) => {
                expr.check(scope, acc);
                check_pattern(pat, scope, acc);
            }
            If(ref cond, ref on_true, ref opt_false) => {
                cond.check(scope, acc);
                on_true.check(scope, acc);

                if let &Some(ref on_false) = opt_false {
                    on_false.check(scope, acc);
                }
            }
            While(ref cond, ref body) => {
                cond.check(scope, acc);
                body.check(scope, acc);
            }
            ForIn(ref each, ref seq, ref body) => {
                seq.check(scope, acc);

                let scope = &mut scope.clone();
                insert_pattern(each, scope, acc);
                body.check(scope, acc);
            }
            Expr(ref expr) => {
                expr.check(scope, acc);
            }
        }
    }
}

impl Check for StmtBlock {
    fn check(&self, scope: Scope, acc: Acc) {
        let scope = &mut scope.clone();

        for stmt in self.iter() {
            stmt.check(scope, acc);
        }
    }
}
