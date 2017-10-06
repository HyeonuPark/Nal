use ast::common::Ast;
use ast::module::{Module, ModuleStmt};

use super::common::{Input, nl};
use super::stmt::{parse_stmt, parse_stmt_sep};

named!(parse_module_stmt(Input) -> ModuleStmt, alt_complete!(
    map!(parse_stmt, ModuleStmt::Stmt)
));

named!(parse_body(Input) -> Vec<Ast<ModuleStmt>>, separated_list_complete!(
    parse_stmt_sep,
    ast!(parse_module_stmt)
));

named!(pub parse_module(Input) -> Module, map!(
    tuple!(nl, parse_body, nl, eof!()),
    |(_, body, _, _)| Module { body }
));
