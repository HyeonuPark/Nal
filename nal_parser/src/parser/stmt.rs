use parse_tree::*;
use super::common::*;

use super::expr::parse_expr;
use super::pattern::{parse_decl, parse_pattern};
use super::function::parse_named_function;

named!(parse_if_stmt(Input) -> IfStmt, map!(
    tuple!(
        word!("if"), sp, parse_expr, sp,
        parse_stmt_block,
        alt_complete!(
            map!(
                tuple!(word!("else"), sp, parse_stmt_block),
                |(_, _, body)| IfFalse::Base(body)
            ) |
            map!(node!(parse_if_stmt), IfFalse::Chain) |
            value!(IfFalse::None, tag!(""))
        )
    ),
    |(_, _, cond, _, body, if_false)| IfStmt(cond, body, if_false)
));

named!(parse_while_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(
        word!("while"), sp, parse_expr,
        sp, parse_stmt_block
    ),
    |(_, _, cond, _, body)| Stmt::While(cond, body)
)));

named!(parse_function_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(
        optional!(tuple!(word!("static"), sp)),
        parse_named_function
    ),
    |(is_static, func)| Stmt::Function {
        is_static: is_static.is_some(),
        func
    }
)));

named!(parse_let_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(word!("let"), sp, parse_decl, sp, tag!("="), sp, parse_expr),
    |(_, _, decl, _, _, _, expr)| Stmt::Let(decl, expr)
)));

named!(parse_assign_stmt(Input) -> Node<Stmt>, node!(map!(
    tuple!(parse_pattern, sp, tag!("="), sp, parse_expr),
    |(pat, _, _, _, expr)| Stmt::Assign(pat, expr)
)));

named!(pub parse_stmt(Input) -> Node<Stmt>, alt_complete!(
    node!(map!(parse_if_stmt, Stmt::If))
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
