use nom_locate::LocatedSpan;
use ast::Span;

pub type Input<'src> = LocatedSpan<&'src str>;

pub fn convert_span<'src>(input: Input<'src>) -> Span<'src> {
    Span {
        offset: input.offset,
        input: input.fragment,
    }
}

pub fn noop<T>(_: T) {}

named!(pub sp(Input) -> (), map!(
  fold_many0!(is_a!(" \t\r"), (), |_, _| ()),
  |_| ()
));

named!(pub sp_f(Input) -> (), map!(
  tuple!(is_a!(" \t"), sp),
  |_| ()
));

named!(pub nl(Input) -> (), map!(
  tuple!(sp, fold_many0!(tuple!(tag!("\n"), sp), (), |_, _| ())),
  |_| ()
));

named!(pub nl_f(Input) -> (), map!(
  tuple!(sp, fold_many1!(tuple!(tag!("\n"), sp), (), |_, _| ())),
  |_| ()
));
