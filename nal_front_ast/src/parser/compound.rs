use ast::*;
use super::common::*;

use super::expr::parse_expr;

named!(parse_tuple(Input) -> Literal, map!(
    block!(
        "(", ",", ")",
        parse_expr
    ),
    Literal::Tuple
));

named!(pub parse_compound(Input) -> Literal, alt_complete!(
    parse_tuple
));
