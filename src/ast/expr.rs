use std::vec;
use std::iter;
use std::ops::Deref;

use ast::{Ast, Span, Value, parse_literal};
use ast::parser::space;
use ast::interpreter::{Result, Control};

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
  Literal(Ast<'a, Value>),
  Binary(BinaryOp, Ast<'a, Expr<'a>>, Ast<'a, Expr<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
}

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
  map!(parse_literal, |value| Expr::Literal(value)) |
  map!(tuple!(tag!("("), space, parse_expr, space, tag!(")")), |(_, _, expr, _, _)| Ast::inner(expr))
)));

named!(parse_binary_op(Span) -> BinaryOp, alt_complete!(
  value!(BinaryOp::Or, tag!("||")) |
  value!(BinaryOp::And, tag!("&&")) |
  value!(BinaryOp::Eq, tag!("==")) |
  value!(BinaryOp::Neq, tag!("!=")) |
  value!(BinaryOp::Gte, tag!(">=")) |
  value!(BinaryOp::Gt, tag!(">")) |
  value!(BinaryOp::Lte, tag!("<=")) |
  value!(BinaryOp::Lt, tag!("<")) |
  value!(BinaryOp::Add, tag!("+")) |
  value!(BinaryOp::Sub, tag!("-")) |
  value!(BinaryOp::Mul, tag!("*")) |
  value!(BinaryOp::Div, tag!("/"))
));

named!(parse_binary_expr(Span) -> Ast<Expr>, map!(
  tuple!(parse_primary_expr, many0!(
    map!(
      tuple!(space, parse_binary_op, space, parse_primary_expr),
      |(_, op, _, expr)| (op, expr)
    )
  )),
  |(head, tail)| operator_precedence(head, &mut tail.into_iter().peekable(), 0)
));

type Ops<'a> = iter::Peekable<vec::IntoIter<(BinaryOp, Ast<'a, Expr<'a>>)>>;

fn operator_precedence<'a>(head: Ast<'a, Expr<'a>>, tail: &mut Ops<'a>, min_prec: usize) -> Ast<'a, Expr<'a>> {
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

    let head_span = head.span.clone();
    let next_span = next.span.clone();
    head = Ast::with_merge(Expr::Binary(op, head, next), head_span, next_span);
  }

  head
}

named!(pub parse_expr(Span) -> Ast<Expr>, alt_complete!(
  parse_binary_expr
));

impl<'a> Expr<'a> {
  pub fn evaluate(&self) -> Result<Value> {
    use self::Expr::*;
    use self::Value::*;
    use self::BinaryOp::*;

    match self {
      &Literal(ref value) => Ok(value.deref().clone()),
      &Binary(op, ref left, ref right) => match (left.evaluate()?, right.evaluate()?) {
        (Number(left), Number(right)) => Ok(match op {
          Add => Number(left + right),
          Sub => Number(left - right),
          Mul => Number(left * right),
          Div => Number(left / right),
          Eq  => Bool(left == right),
          Neq => Bool(left != right),
          Gt  => Bool(left > right),
          Gte => Bool(left >= right),
          Lt  => Bool(left < right),
          Lte => Bool(left <= right),
          _ => return Err(Control::RuntimeError(format!("TypeError"))),
        }),
        (Bool(left), Bool(right)) => Ok(match op {
          Eq  => Bool(left == right),
          Neq => Bool(left != right),
          And => Bool(left && right),
          Or  => Bool(left || right),
          _ => return Err(Control::RuntimeError(format!("TypeError"))),
        }),
        _ => Err(Control::RuntimeError("Cannot calculate bool or None".into())),
      },
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use self::Expr::*;
  use self::Value::*;
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
          Ast::new(
            Span {
              offset: 0,
              line: 1,
              fragment: "3+7*5-6",
            },
            Binary(
              BinaryOp::Sub,
              Ast::new(
                Span {
                  offset: 0,
                  line: 1,
                  fragment: "3+7*5",
                },
                Binary(
                  BinaryOp::Add,
                  Ast::new(
                    Span {
                      offset: 0,
                      line: 1,
                      fragment: "3",
                    },
                    Literal(Ast::new(
                      Span {
                        offset: 0,
                        line: 1,
                        fragment: "3",
                      },
                      Number(3.0)
                    )),
                  ),
                  Ast::new(
                    Span {
                      offset: 2,
                      line: 1,
                      fragment: "7*5",
                    },
                    Binary(
                      BinaryOp::Mul,
                      Ast::new(
                        Span {
                          offset: 2,
                          line: 1,
                          fragment: "7",
                        },
                        Literal(Ast::new(
                          Span {
                            offset: 2,
                            line: 1,
                            fragment: "7",
                          },
                          Number(7.0)
                        )),
                      ),
                      Ast::new(
                        Span {
                          offset: 4,
                          line: 1,
                          fragment: "5",
                        },
                        Literal(Ast::new(
                          Span {
                            offset: 4,
                            line: 1,
                            fragment: "5",
                          },
                          Number(5.0)
                        )),
                      )
                    )
                  )
                )
              ),
              Ast::new(
                Span {
                  offset: 6,
                  line: 1,
                  fragment: "6",
                },
                Literal(Ast::new(
                  Span {
                    offset: 6,
                    line: 1,
                    fragment: "6",
                  },
                  Number(6.0)
                )),
              ),
            )
          )
        )
      );
  }

  #[test]
  fn test_evaluate_binary_expr() {
      assert_eq!(
        parse_expr(Span::new("3+7*5-6")).unwrap().1.evaluate().unwrap(),
        Number(32.0)
      );
      assert_eq!(
        parse_expr(Span::new("5 + 2 * 4 * 5 * 9")).unwrap().1.evaluate().unwrap(),
        Number(365.0)
      );
      assert_eq!(
        parse_expr(Span::new("7 > 2 + 4")).unwrap().1.evaluate().unwrap(),
        Bool(true)
      );
  }
}
