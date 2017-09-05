use nom::digit;
use ast::{Ast, Span};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  Bool(bool),
}

named!(parse_number(Span) -> Ast<Value>, ast!(map!(
  recognize!(alt_complete!(
    map!(tuple!(digit, tag!("."), digit), |_| ()) |
    map!(digit, |_| ())
  )),
  |src| Value::Number(src.fragment.parse().unwrap())
)));

named!(parse_bool(Span) -> Ast<Value>, ast!(alt_complete!(
  value!(Value::Bool(true), tag!("true")) |
  value!(Value::Bool(false), tag!("false"))
)));

named!(pub parse_literal(Span) -> Ast<Value>, alt_complete!(
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
      Ast::new(
        Span {
          offset: 0,
          line: 1,
          fragment: "999",
        },
        Value::Number(999.0),
      )
    ));
    assert_eq!(parse_number(Span::new("42.7d")), IResult::Done(
      Span {
        offset: 4,
        line: 1,
        fragment: "d",
      },
      Ast::new(
        Span {
          offset: 0,
          line: 1,
          fragment: "42.7",
        },
        Value::Number(42.7),
      )
    ));

    assert_eq!(parse_number(Span::new("119gh")), IResult::Done(
      Span {
        offset: 3,
        line: 1,
        fragment: "gh",
      },
      Ast::new(
        Span {
          offset: 0,
          line: 1,
          fragment: "119",
        },
        Value::Number(119.0),
      )
    ))
  }
}
