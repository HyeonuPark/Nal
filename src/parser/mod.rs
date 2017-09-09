use ast::Span;

#[macro_use]
mod ast_macro {
  macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
      map!($i,
        tuple!(
          position!(),
          $submac!($($args)*),
          position!()
        ),
        (|(left, body, right)| $crate::ast::Ast::with_merge(body, left, right))
      )
    );
    ($i:expr, $f:ident) => (
      ast!($i, call!($f))
    );
  }
}

mod expr;
pub use self::expr::*;

mod literal;
pub use self::literal::*;

mod ident;
pub use self::ident::*;

mod stmt;
pub use self::stmt::*;

named!(pub space(Span) -> (), map!(
  fold_many0!(is_a!(" \t\r"), (), |_, _| ()),
  |_| ()
));

named!(pub space_force(Span) -> (), map!(
  tuple!(is_a!(" \t"), space),
  |_| ()
));

named!(pub new_line(Span) -> (), map!(
  tuple!(space, fold_many0!(tuple!(tag!("\n"), space), (), |_, _| ())),
  |_| ()
));

named!(pub new_line_force(Span) -> (), map!(
  tuple!(space, fold_many1!(tuple!(tag!("\n"), space), (), |_, _| ())),
  |_| ()
));
