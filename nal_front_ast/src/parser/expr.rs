use ast::*;
use super::common::*;

use super::literal::parse_literal;

named!(pub parse_expr(Input) -> Expr, alt_complete!(
    ast!(parse_literal) => {|l| Expr::Literal(l)}
));
