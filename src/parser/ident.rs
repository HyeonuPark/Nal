use nom::{alpha, alphanumeric};

use ast::Span;

named!(ident_head(Span) -> (), alt_complete!(
  map!(alpha, |_| ()) |
  map!(one_of!("_"), |_| ())
));

named!(ident_char(Span) -> (), alt_complete!(
  map!(alphanumeric, |_| ()) |
  map!(one_of!("_"), |_| ())
));

named!(pub parse_ident(Span) -> &str, map!(
  recognize!(tuple!(
    ident_head,
    fold_many0!(ident_char, (), |_, _| ())
  )),
  |span| span.fragment
));
