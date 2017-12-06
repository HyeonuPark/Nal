use ast::*;
use super::common::*;

use super::expr::parse_expr;

named!(parse_if_stmt(Input) -> Ast<Stmt>, ast!(map!(
    tuple!(
        separated_list!(
            tuple!(nl, word!("else"), sp),
            map!(
                tuple!(
                    word!("if"), sp, parse_expr,
                    sp, parse_stmt_block
                ),
                |(_, _, cond, _, body)| (cond, body)
            )
        ),
        optional!(preceded!(
            tuple!(nl, word!("else"), sp),
            parse_stmt_block
        ))
    ),
    |(conditional, otherwise)| Stmt::If { conditional, otherwise }
)));

named!(parse_while_stmt(Input) -> Ast<Stmt>, ast!(map!(
    tuple!(
        word!("while"), sp, parse_expr,
        sp, parse_stmt_block
    ),
    |(_, _, condition, _, body)| Stmt::While { condition, body }
)));

named!(pub parse_stmt(Input) -> Ast<Stmt>, alt_complete!(
    parse_if_stmt
    | parse_while_stmt
    | ast!(map!(parse_expr, Stmt::Expr))
));

named!(pub parse_stmt_block(Input) -> Block<Stmt>, block!(
    "{", ";", "}",
    parse_stmt
));
