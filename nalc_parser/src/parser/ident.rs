use super::prelude::*;
use nom::{alpha1, alphanumeric1};

pub const IDENT_CHARS: &str = "_ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

named!{pub parse_ident(Src) -> Node<Ident>, node!(map!(
    alt!(
        recognize!(tuple!(alpha1, is_a_s!(IDENT_CHARS)))
        | recognize!(tuple!(
            many1!(tag!("_")),
            alphanumeric1,
            is_a_s!(IDENT_CHARS)
        ))
    ),
    |src| Ident(src.0.into())
))}
