use ast::{Ast, Module, ModuleStmt};

use common::{Input, nl};
use stmt::{parse_stmt, parse_stmt_sep};

named!(parse_module_stmt(Input) -> ModuleStmt, alt_complete!(
    map!(parse_stmt, |stmt| ModuleStmt::Stmt(stmt))
));

named!(parse_body(Input) -> Vec<Ast<ModuleStmt>>, separated_list_complete!(
    parse_stmt_sep,
    ast!(parse_module_stmt)
));

named!(pub parse_module(Input) -> Module, map!(
    tuple!(nl, parse_body, nl, eof!()),
    |(_, body, _, _)| Module { body }
));
