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
        word!("fn"), sp,
        opt!(delimited!(sp, parse_ident, sp)),
        opt!(delimited!(
            tuple!(tag!("("), nl),
            separated_list_complete!(
                parse_expr_sep,
                parse_pattern
            ),
            tuple!(nl, tag!(")"))
        )),
        sp, parse_function_body
    ),
    |(_, _, name, params, _, body)| {
        Function {
            name,
            params: params.unwrap_or_default(),
            body,
        }
    }
)));
