use ast::Span;

named!(pub space(Span) -> (), map!(
  fold_many0!(is_a!(" \t\r"), (), |_, _| ()),
  |_| ()
));

named!(pub new_line(Span) -> (), map!(
  tuple!(space, fold_many0!(tuple!(tag!("\n"), space), (), |_, _| ())),
  |_| ()
));
