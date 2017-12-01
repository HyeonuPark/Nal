use nom::{digit, alphanumeric};

use ast::*;
use super::common::*;

use super::compound::parse_compound;

named!(parse_bool(Input) -> Literal, value!(
    Literal::Bool,
    alt_complete!(word!("true") | word!("false"))
));

named!(parse_num(Input) -> Literal, value!(
    Literal::Num,
    alt_complete!(
        tuple!(digit, tag!("."), digit) => {noop}
        | digit => {noop}
    )
));

named!(pub parse_literal(Input) -> Literal, alt_complete!(
    parse_bool
    | parse_num
    | parse_compound
));
