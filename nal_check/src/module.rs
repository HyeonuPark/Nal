use nal_ast::ast::module::{Module, ModuleStmt};

use common::{Check, Scope, Acc};

impl Check for Module {
    fn check(&self, scope: Scope, acc: Acc) {
        for stmt in self.body.iter() {
            stmt.check(scope, acc);
        }
    }
}

impl Check for ModuleStmt {
    fn check(&self, scope: Scope, acc: Acc) {
        use self::ModuleStmt::*;

        match *self {
            Stmt(ref stmt) => stmt.check(scope, acc),
        };
    }
}
