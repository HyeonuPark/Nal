use std::iter;
use std::vec;

use ast::{Ast, Span, Expr, BinaryOp, UnaryOp};
use parser::{space, new_line, parse_literal, parse_function_expr, parse_ident};

impl BinaryOp {
  #[inline]
  pub fn precedence(&self) -> usize {
    use self::BinaryOp::*;

    match *self {
      Or => 1,
      And => 2,
      Eq | Neq | Gt | Gte | Lt | Lte => 3,
      Add | Sub => 4,
      Mul | Div => 5,
    }
  }
}

named!(parse_primary_expr(Span) -> Ast<Expr>, ast!(alt_complete!(
  map!(parse_literal, Expr::Literal) |
  map!(parse_ident, Expr::Identifier) |
  map!(
    tuple!(tag!("("), new_line, parse_expr, new_line, tag!(")")),
    |(_, _, expr, _, _)| Ast::unwrap(expr)
  )
)));

named!(parse_binary_op(Span) -> BinaryOp, alt_complete!(
  value!(BinaryOp::Or,  tag!("||")) |
  value!(BinaryOp::And, tag!("&&")) |
  value!(BinaryOp::Eq,  tag!("==")) |
  value!(BinaryOp::Neq, tag!("!=")) |
  value!(BinaryOp::Gte, tag!(">=")) |
  value!(BinaryOp::Gt,  tag!(">" )) |
  value!(BinaryOp::Lte, tag!("<=")) |
  value!(BinaryOp::Lt,  tag!("<" )) |
  value!(BinaryOp::Add, tag!("+" )) |
  value!(BinaryOp::Sub, tag!("-" )) |
  value!(BinaryOp::Mul, tag!("*" )) |
  value!(BinaryOp::Div, tag!("/" ))
));

named!(parse_unary_op(Span) -> UnaryOp, alt_complete!(
  value!(UnaryOp::Neg, tag!("-")) |
  value!(UnaryOp::Not, tag!("!"))
));

named!(parse_unary_expr(Span) -> Ast<Expr>, alt_complete!(
  ast!(map!(
    tuple!(parse_unary_op, space, parse_primary_expr),
    |(op, _, expr)| Expr::Unary(op, expr)
  )) |
  parse_primary_expr
));

named!(parse_binary_expr(Span) -> Ast<Expr>, map!(
  tuple!(parse_unary_expr, many0!(
    map!(
      tuple!(space, parse_binary_op, space, parse_unary_expr),
      |(_, op, _, expr)| (op, expr)
    )
  )),
  |(head, tail)| operator_precedence(head, &mut tail.into_iter().peekable(), 0)
));

type Ops<'a> = iter::Peekable<vec::IntoIter<(BinaryOp, Ast<'a, Expr<'a>>)>>;

fn operator_precedence<'a>(
  head: Ast<'a, Expr<'a>>, tail: &mut Ops<'a>, min_prec: usize
) -> Ast<'a, Expr<'a>> {
  let mut head = head;

  while let Some(&(op, _)) = tail.peek() {
    if op.precedence() < min_prec {
      break;
    }

    let (op, mut next) = tail.next().unwrap();

    while let Some(&(next_op, _)) = tail.peek() {
      if next_op.precedence() <= op.precedence() {
        break;
      }

      next = operator_precedence(next, tail, next_op.precedence());
    }

    let head_span = head.span;
    let next_span = next.span;
    head = Ast::with_merge(Expr::Binary(op, head, next), head_span, next_span);
  }

  head
}

named!(pub parse_expr(Span) -> Ast<Expr>, alt_complete!(
  parse_binary_expr
));

#[cfg(test)]
mod test {
  use super::*;
  use ast::Expr::*;
  use ast::Literal::*;
  use nom::IResult;

  #[test]
  fn test_ast_binary_expr() {
    assert_eq!(
      parse_expr(Span::new("3+7*5-6")),
      IResult::Done(
        Span {
          offset: 7,
          line: 1,
          fragment: "",
        },
        Ast::dummy(Binary(
          BinaryOp::Sub,
          Ast::dummy(Binary(
            BinaryOp::Add,
            Ast::dummy(Literal(Ast::dummy(Number(3.0)))),
            Ast::dummy(Binary(
              BinaryOp::Mul,
              Ast::dummy(Literal(Ast::dummy(Number(7.0)))),
              Ast::dummy(Literal(Ast::dummy(Number(5.0)))),
            )),
          )),
          Ast::dummy(Literal(Ast::dummy(Number(6.0)))),
        )),
      )
    );
  }
}
