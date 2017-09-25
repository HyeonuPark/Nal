use ast::{Ast, Stmt, StmtBlock};

use common::{Input, sp, nl, nl_f};
use pattern::parse_pattern;
use expr::parse_expr;

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

named!(parse_if_stmt(Input) -> Stmt, map!(
    tuple!(
        word!("if"), sp,
        parse_expr, sp,
        parse_stmt_block,
        opt!(preceded!(
            tuple!(nl, word!("else"), sp),
            parse_stmt_block
        ))
    ),
    |(_, _, cond, _, on_true, opt_on_false)| {
        Stmt::If(cond, on_true, opt_on_false)
    }
));

named!(pub parse_stmt(Input) -> Ast<Stmt>, ast!(alt_complete!(
    parse_let_stmt |
    parse_assign_stmt |
    parse_if_stmt |
    map!(
        parse_expr,
        |expr| Stmt::Expr(expr)
    )
)));

named!(pub parse_stmt_block(Input) -> StmtBlock, delimited!(
    tuple!(tag!("{"), nl),
    separated_list_complete!(
        nl_f,
        parse_stmt
    ),
    tuple!(tag!("}"), nl)
));
