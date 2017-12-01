use ast::*;
use super::common::*;

use super::stmt::parse_stmt;

named!(pub parse_module(Input) -> Module, map!(
    block!(
        "", ";", "",
        parse_module_stmt
    ),
    |body| Module {
        body
    }
));

named!(pub parse_module_stmt(Input) -> ModuleStmt, alt_complete!(
    ast!(parse_stmt) => {ModuleStmt::Stmt}
));
