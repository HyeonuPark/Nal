use nom::{digit, anychar};

use parse_tree::*;
use super::common::*;

use super::compound::parse_compound;

named!(parse_bool(Input) -> Literal, value!(
    Literal::Bool,
    alt_complete!(word!("true") | word!("false"))
));

named!(parse_num(Input) -> Literal, value!(
    Literal::Num,
    alt_complete!(
        tuple!(digit, tag!("."), digit) => {noop}
        | digit => {noop}
    )
));

named!(parse_str(Input) -> Literal, value!(
    Literal::Str,
    map!(
        tuple!(
            tag!("\'"),
            fold_many0!(
                alt_complete!(
                    none_of!("\\\'") => {noop}
                    | tuple!(tag!("\\"), anychar) => {noop}
                ),
                (), noop2
            ),
            tag!("\'")
        ),
        noop
    )
));

named!(pub parse_literal(Input) -> Span<Literal>, alt_complete!(
      span!(parse_bool)
    | span!(parse_num)
    | span!(parse_str)
    | parse_compound
));
