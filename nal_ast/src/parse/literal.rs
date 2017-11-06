use nom::{digit};

use ast::common::Ast;
use ast::expr::{Literal, ObjProp};

use super::common::{Input, sp, nl};
use super::string::parse_string;
use super::ident::parse_ident;
use super::expr::{parse_expr, parse_expr_sep};
use super::function::parse_function;

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

named!(parse_obj_prop(Input) -> ObjProp, alt_complete!(
    parse_function => { ObjProp::Method } |
    tuple!(parse_ident, sp, tag!("="), sp, parse_expr) => {
        |(name, _, _, _, value)| ObjProp::Named(name, value)
    } |
    parse_ident => { ObjProp::Short }
));

named!(parse_obj(Input) -> Literal, map!(
    delimited!(
        tuple!(tag!("{"), nl),
        separated_list_complete!(
            parse_expr_sep,
            ast!(parse_obj_prop)
        ),
        tuple!(nl, tag!("}"))
    ),
    Literal::Obj
));

named!(pub parse_literal(Input) -> Ast<Literal>, ast!(alt_complete!(
    parse_number |
    parse_bool |
    parse_string |
    parse_obj
)));
