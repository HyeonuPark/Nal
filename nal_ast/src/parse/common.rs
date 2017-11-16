use nom_locate::LocatedSpan;

pub type Input<'src> = LocatedSpan<&'src str>;

pub fn noop<T>(_: T) {}

named!(newline(Input) -> (), alt_complete!(
    map!(
        tuple!(tag!("//"), is_not_s!("\n"), tag!("\n")),
        noop
    ) |
    map!(
        tag!("\n"),
        noop
    )
));

named!(pub sp(Input) -> (), map!(
  fold_many0!(is_a!(" \t\r"), (), |_, _| ()),
  noop
));

named!(pub sp_f(Input) -> (), map!(
  tuple!(is_a!(" \t"), sp),
  noop
));

named!(pub nl(Input) -> (), map!(
  tuple!(sp, fold_many0!(tuple!(newline, sp), (), |_, _| ())),
  noop
));

named!(pub nl_f(Input) -> (), map!(
  tuple!(sp, fold_many1!(tuple!(newline, sp), (), |_, _| ())),
  noop
));
