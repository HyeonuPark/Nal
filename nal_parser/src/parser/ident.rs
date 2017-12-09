use nom::{alpha, digit};

use parse_tree::*;
use super::common::*;

named!(nondigit(Input) -> (), fold_many0!(
    alt_complete!(
        alpha => {noop}
        | one_of!("_") => {noop}
    ),
    (), noop2
));

named!(pub ident_chars(Input) -> (), fold_many0!(
    alt_complete!(
        digit => {noop}
        | nondigit => {noop}
    ),
    (), noop2
));

named!(pub parse_ident(Input) -> Node<Ident>, node!(value!(
    Ident,
    tuple!(nondigit, ident_chars)
)));
