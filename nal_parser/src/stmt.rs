use ast::{Ast, Stmt, StmtBlock};

use common::{Input, sp, nl, nl_f, noop};
use pattern::parse_pattern;
use expr::parse_expr;

named!(parse_if_stmt(Input) -> Stmt, alt_complete!(
    map!(
        tuple!(
            word!("if"), sp,
            parse_expr, sp,
            parse_stmt_block,
            tuple!(nl, word!("else"), sp),
            parse_stmt_block
        ),
        |(_, _, cond, _, pos, _, neg)| Stmt::If(cond, pos, Some(neg))
    ) |
    map!(
        tuple!(
            word!("if"), sp,
            parse_expr, sp,
            parse_stmt_block
        ),
        |(_, _, cond, _, pos)| Stmt::If(cond, pos, None)
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
    parse_let_stmt |
    parse_assign_stmt |
    map!(
        parse_expr,
        |expr| Stmt::Expr(expr)
    )
)));

named!(pub parse_stmt_sep(Input) -> (), alt_complete!(
    map!(tuple!(nl, tag!(";"), nl), noop) |
    map!(nl_f, noop)
));

named!(pub parse_stmt_block(Input) -> StmtBlock, delimited!(
    tuple!(tag!("{"), nl),
    separated_list_complete!(
        parse_stmt_sep,
        parse_stmt
    ),
    tuple!(nl, tag!("}"))
));