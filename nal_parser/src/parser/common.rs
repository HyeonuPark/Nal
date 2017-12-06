use nom_locate::LocatedSpan;

pub use codebuf::{Span, Spanned};

pub fn noop<T>(_: T) {}
pub fn noop2<T, U>(_: T, _: U) {}

pub type Input<'src> = LocatedSpan<&'src str>;

named!(pub sp(Input) -> (), map!(
    is_a!(" \t\r"),
    noop
));

named!(newline(Input) -> (), alt_complete!(
    tuple!(tag!("//"), is_not_s!("\n"), tag!("\n")) => {noop}
    | tag!("\n") => {noop}
));

named!(pub nl(Input) -> (), map!(
    tuple!(
        sp,
        fold_many1!(
            tuple!(newline, sp),
            (),
            noop2
        )
    ),
    noop
));
