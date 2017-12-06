use ast::*;
use super::common::*;

use super::expr::parse_expr;
use super::ident::parse_ident;

named!(parse_tuple_elem(Input) -> Ast<TupleElem>, ast!(alt_complete!(
    map!(parse_expr, TupleElem::Atom)
    | map!(
        tuple!(tag!("..."), sp, parse_expr),
        |(_, _, expr)| TupleElem::Spread(expr)
    )
)));

named!(pub parse_tuple(Input) -> Block<TupleElem>, block!(
    "(", ",", ")",
    parse_tuple_elem
));

named!(parse_obj_prop(Input) -> Ast<ObjProp>, ast!(alt_complete!(
    map!(
        tuple!(parse_ident, tuple!(nl, tag!("="), sp), parse_expr),
        |(name, _, expr)| ObjProp::Named(name, expr)
    )
    | map!(
        parse_ident,
        |name| ObjProp::Short(name)
    )
)));

named!(parse_obj(Input) -> Block<ObjProp>, block!(
    "{", ",", "}",
    parse_obj_prop
));

named!(pub parse_compound(Input) -> Ast<Literal>, ast!(alt_complete!(
    map!(parse_tuple, Literal::Tuple)
    | map!(parse_obj, Literal::Obj)
)));
