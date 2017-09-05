use nom::digit;

use ast::{Ast, Span, Literal};

named!(parse_number(Span) -> Ast<Literal>, ast!(map!(
  recognize!(alt_complete!(
    map!(tuple!(digit, tag!("."), digit), |_| ()) |
    map!(digit, |_| ())
  )),
  |src| Literal::Number(src.fragment.parse().unwrap())
)));

named!(parse_bool(Span) -> Ast<Literal>, ast!(alt_complete!(
  value!(Literal::Bool(true), tag!("true")) |
  value!(Literal::Bool(false), tag!("false"))
)));

named!(pub parse_literal(Span) -> Ast<Literal>, alt_complete!(
  parse_number |
  parse_bool
));

#[cfg(test)]
mod test {
  use super::*;
  use nom::IResult;

  #[test]
  fn test_parse_number() {
    assert_eq!(parse_number(Span::new("999")), IResult::Done(
      Span {
        offset: 3,
        line: 1,
        fragment: "",
      },
      Ast::dummy(Literal::Number(999.0))
    ));
    assert_eq!(parse_number(Span::new("42.7d")), IResult::Done(
      Span {
        offset: 4,
        line: 1,
        fragment: "d",
      },
      Ast::dummy(Literal::Number(42.7))
    ));

    assert_eq!(parse_number(Span::new("119gh")), IResult::Done(
      Span {
        offset: 3,
        line: 1,
        fragment: "gh",
      },
      Ast::dummy(Literal::Number(119.0))
    ));
  }

  #[test]
  fn test_parse_bool() {
    assert_eq!(parse_bool(Span::new("truefalse")), IResult::Done(
      Span {
        offset: 4,
        line: 1,
        fragment: "false",
      },
      Ast::dummy(Literal::Bool(true))
    ));
  }
}
