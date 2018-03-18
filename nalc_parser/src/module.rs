use super::prelude::*;
use super::stmt::parse_stmt;

named!{pub parse_module(Src) -> Module, map!(
    tuple!(
        block!("", "", nl, parse_module_stmt),
        eof!()
    ),
    |(stmts, _)| Module {
        body: stmts,
    }
)}

named!{parse_module_stmt(Src) -> Node<ModuleStmt>, node!(alt!(
    map!(parse_stmt, ModuleStmt::Stmt)
))}
