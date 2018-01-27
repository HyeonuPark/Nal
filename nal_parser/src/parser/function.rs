use parse_tree::*;
use super::common::*;

use super::ident::parse_ident;
use super::pattern::parse_tuple_decl;
use super::stmt::parse_stmt_block;
use super::expr::parse_expr;

named!(parse_function_body(Input) -> FunctionBody, alt_complete!(
    map!(
        preceded!(sp, parse_stmt_block),
        FunctionBody::Stmt
    )
    | map!(
        tuple!(nl, tag!("="), sp, parse_expr),
        |(_, _, _, expr)| FunctionBody::Expr(expr)
    )
));

named!(pub parse_function(Input) -> Node<Function>, node!(map!(
    tuple!(
        word!("fn"),
        optional!(preceded!(sp, parse_ident)),
        optional!(preceded!(sp, parse_tuple_decl)),
        parse_function_body
    ),
    |(_, name, params, body)| Function { name, params, body }
)));

named!(pub parse_named_function(Input) -> Node<NamedFunction>, node!(map!(
    tuple!(
        word!("fn"),
        preceded!(sp, parse_ident),
        optional!(preceded!(sp, parse_tuple_decl)),
        parse_function_body
    ),
    |(_, name, params, body)| NamedFunction { name, params, body }
)));
