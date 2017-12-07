use parse_tree::*;
use super::common::*;

use super::expr::parse_expr;
use super::pattern::parse_pattern;
use super::function::parse_function;

named!(parse_if_stmt(Input) -> Node<Stmt>, node!(map!(
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

named!(parse_while_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(
        word!("while"), sp, parse_expr,
        sp, parse_stmt_block
    ),
    |(_, _, condition, _, body)| Stmt::While { condition, body }
)));

named!(parse_function_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(
        optional!(tuple!(word!("static"), sp)),
        parse_function
    ),
    |(is_static, func)| Stmt::Function {
        is_static: is_static.is_some(),
        func
    }
)));

named!(parse_let_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(word!("let"), sp, parse_pattern, sp, tag!("="), sp, parse_expr),
    |(_, _, pat, _, _, _, expr)| Stmt::Let(pat, expr)
)));

named!(parse_assign_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(parse_pattern, sp, tag!("="), sp, parse_expr),
    |(pat, _, _, _, expr)| Stmt::Assign(pat, expr)
)));

named!(pub parse_stmt(Input) -> Node<Stmt>, alt_complete!(
    parse_if_stmt
    | parse_while_stmt
    | parse_function_stmt
    | parse_let_stmt
    | parse_assign_stmt
    | node!(map!(parse_expr, Stmt::Expr))
));

named!(pub parse_stmt_block(Input) -> Block<Stmt>, block!(
    "{", ";", "}",
    parse_stmt
));
