use super::prelude::*;
use nom::digit;

named!{parse_bool_literal(Src) -> Node<Literal>, node!(alt!(
    value!(Literal::Bool(true), word!("true")) |
    value!(Literal::Bool(false), word!("false"))
))}

named!{parse_num_literal(Src) -> Node<Literal>, node!(alt!(
    map!(
        recognize!(alt!(
            tuple!(digit, tag!("."), digit) => {noop}
            | digit => {noop}
        )),
        |num| Literal::Num(num.0.parse().unwrap())
    )
))}

named!{parse_str_literal(Src) -> Node<Literal>, node!(map!(
    delimited!(
        tag!("\'"),
        fold_many0!(
            parse_str_char,
            String::new(),
            |mut s: String, ch| {
                s.push(ch);
                s
            }
        ),
        tag!("\'")
    ),
    Literal::Str
))}

named!{parse_str_char(Src) -> char, alt!(
    map!(
        tuple!(tag!("\\"), uni_char),
        |(_, ch)| match ch {
            '0' => '\0',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            other => other,
        }
    ) |
    uni_none_of!("\n\'\"")
)}

named!{pub parse_literal(Src) -> Node<Literal>, alt!(
    parse_bool_literal
    | parse_num_literal
    | parse_str_literal
)}
