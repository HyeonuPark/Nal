use ast::{Span, Ast, Stmt, StmtBlock};
use parser::{space, new_line, new_line_force, parse_expr, parse_ident};

named!(parse_let_stmt(Span) -> Ast<Stmt>, ast!(map!(
  tuple!(
    tag!("let"),
    opt!(preceded!(space, tag!("mut"))),
    space,
    parse_ident,
    tuple!(space, tag!("="), space),
    parse_expr
  ),
  |(_, is_mut, _, ident, _, expr)| Stmt::Let(ident, is_mut.is_some(), expr)
)));

named!(parse_assign_stmt(Span) -> Ast<Stmt>, ast!(map!(
  tuple!(parse_ident, space, tag!("="), space, parse_expr),
  |(ident, _, _, _, expr)| Stmt::Assign(ident, expr)
)));

named!(parse_stmt(Span) -> Ast<Stmt>, alt_complete!(
  parse_let_stmt |
  parse_assign_stmt |
  ast!(map!(
    parse_expr,
    |expr| Stmt::Expr(expr)
  ))
));

named!(pub parse_stmt_block(Span) -> StmtBlock, delimited!(
  tuple!(tag!("{"), new_line),
  separated_list_complete!(new_line_force, parse_stmt),
  tuple!(new_line, tag!("}"))
));
