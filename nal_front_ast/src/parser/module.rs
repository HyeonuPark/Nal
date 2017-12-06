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

named!(pub parse_module_stmt(Input) -> Ast<ModuleStmt>, ast!(alt_complete!(
    parse_stmt => {ModuleStmt::Stmt}
)));
