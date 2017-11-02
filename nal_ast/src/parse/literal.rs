use nom::{digit};

use ast::common::Ast;
use ast::expr::Literal;

use super::common::Input;
use super::string::parse_string;

named!(parse_number(Input) -> Literal, map!(
    alt_complete!(
        recognize!(tuple!(digit, tag!("."), digit)) |
        digit
    ),
    |input| Literal::Num(input.fragment.parse().unwrap())
));

named!(parse_bool(Input) -> Literal, map!(
    alt_complete!(
        word!("true") |
        word!("false")
    ),
    |input| Literal::Bool(input.fragment.parse().unwrap())
));

named!(pub parse_literal(Input) -> Ast<Literal>, ast!(alt_complete!(
    parse_number |
    parse_bool |
    parse_string
)));
