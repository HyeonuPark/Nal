use ast::*;
use super::common::*;

use super::expr::parse_expr;

named!(pub parse_stmt(Input) -> Stmt, alt_complete!(
    ast!(parse_expr) => {Stmt::Expr}
));
