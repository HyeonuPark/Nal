use ast::common::Ast;
use ast::function::{Function, FunctionBody};

use super::common::{Input, sp, nl};
use super::stmt::parse_stmt_block;
use super::expr::{parse_expr, parse_expr_sep};
use super::ident::parse_ident;
use super::pattern::parse_pattern;

named!(parse_function_body(Input) -> FunctionBody, alt_complete!(
    map!(
        preceded!(sp, parse_stmt_block),
        FunctionBody::Stmt
    ) |
    map!(
        tuple!(sp, tag!("="), sp, parse_expr),
        |(_, _, _, expr)| FunctionBody::Expr(expr)
    )
));

named!(pub parse_function(Input) -> Ast<Function>, ast!(map!(
    tuple!(
        word!("fn"),
        opt!(preceded!(sp, parse_ident)),
        opt!(delimited!(
            tuple!(sp, tag!("("), nl),
            separated_list_complete!(
                parse_expr_sep,
                parse_pattern
            ),
            tuple!(nl, tag!(")"))
        )),
        parse_function_body
    ),
    |(_, name, params, body)| {
        Function {
            name,
            params: params.unwrap_or_default(),
            body,
        }
    }
)));
