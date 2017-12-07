use parse_tree::*;
use super::common::*;

use super::expr::parse_expr;
use super::ident::parse_ident;
use super::function::parse_function;

named!(parse_tuple_elem(Input) -> Node<TupleElem>, node!(alt_complete!(
    map!(parse_expr, TupleElem::Atom)
    | map!(
        tuple!(tag!("..."), sp, parse_expr),
        |(_, _, expr)| TupleElem::Spread(expr)
    )
)));

named!(pub parse_tuple_literal(Input) -> Block<TupleElem>, block!(
    "(", ",", ")",
    parse_tuple_elem
));

named!(parse_obj_prop(Input) -> Node<ObjProp>, node!(alt_complete!(
    map!(
        parse_function,
        ObjProp::Method
    )
    | map!(
        tuple!(parse_ident, tuple!(nl, tag!("="), sp), parse_expr),
        |(name, _, expr)| ObjProp::Named(name, expr)
    )
    | map!(
        parse_ident,
        |name| ObjProp::Short(name)
    )
)));

named!(parse_obj_literal(Input) -> Block<ObjProp>, block!(
    "{", ",", "}",
    parse_obj_prop
));

named!(pub parse_compound(Input) -> Node<Literal>, node!(alt_complete!(
    map!(parse_tuple_literal, Literal::Tuple)
    | map!(parse_obj_literal, Literal::Obj)
    | map!(parse_function, Literal::Function)
)));
