use ast::common::Ast;
use ast::stmt::{Stmt, StmtBlock};

use super::common::{Input, sp, nl, nl_f, noop};
use super::pattern::parse_pattern;
use super::expr::parse_expr;
use super::function::parse_function;

named!(parse_if_stmt(Input) -> Stmt, alt_complete!(
    map!(
        tuple!(
            word!("if"),
            delimited!(sp, parse_expr, sp),
            parse_stmt_block,
            tuple!(nl, word!("else"), sp),
            parse_stmt_block
        ),
        |(_, cond, pos, _, neg)| Stmt::If(cond, pos, Some(neg))
    ) |
    map!(
        tuple!(
            word!("if"),
            delimited!(sp, parse_expr, sp),
            parse_stmt_block
        ),
        |(_, cond, pos)| Stmt::If(cond, pos, None)
    )
));

named!(parse_while_stmt(Input) -> Stmt, map!(
    tuple!(
        word!("while"), sp,
        parse_expr, sp,
        parse_stmt_block
    ),
    |(_, _, cond, _, block)| Stmt::While(cond, block)
));

named!(parse_for_in_stmt(Input) -> Stmt, map!(
    tuple!(
        word!("for"), sp,
        parse_pattern, sp,
        word!("in"), sp,
        parse_expr, sp,
        parse_stmt_block
    ),
    |(_, _, pat, _, _, _, expr, _, block)| Stmt::ForIn(pat, expr, block)
));

named!(parse_function_stmt(Input) -> Stmt, map!(
    tuple!(
        opt!(tuple!(word!("static"), sp)),
        parse_function
    ),
    |(is_static, func)| Stmt::Function(is_static.is_some(), func)
));

named!(parse_let_stmt(Input) -> Stmt, map!(
    tuple!(
        word!("let"), sp,
        parse_pattern, sp,
        tag!("="), sp,
        parse_expr
    ),
    |(_, _, pattern, _, _, _, expr)| Stmt::Let(pattern, expr)
));

named!(parse_assign_stmt(Input) -> Stmt, map!(
    tuple!(
        parse_pattern, sp,
        tag!("="), sp,
        parse_expr
    ),
    |(pattern, _, _, _, expr)| Stmt::Assign(pattern, expr)
));

named!(pub parse_stmt(Input) -> Ast<Stmt>, ast!(alt_complete!(
    parse_if_stmt |
    parse_while_stmt |
    parse_for_in_stmt |
    parse_function_stmt |
    parse_let_stmt |
    parse_assign_stmt |
    map!(
        parse_expr,
        Stmt::Expr
    )
)));

named!(pub parse_stmt_sep(Input) -> (), alt_complete!(
    map!(tuple!(nl, tag!(";"), nl), noop) |
    map!(nl_f, noop)
));

named!(pub parse_stmt_block(Input) -> Ast<StmtBlock>, ast!(delimited!(
    tuple!(tag!("{"), nl),
    separated_list_complete!(
        parse_stmt_sep,
        parse_stmt
    ),
    tuple!(nl, tag!("}"))
)));
