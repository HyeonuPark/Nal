use nom::anychar;

use ast::expr::Literal;

use super::common::Input;

macro_rules! str_char {
    ($name:ident($quote:expr)) => (named!($name(Input) -> char, alt_complete!(
        value!(
            ' ',
            tuple!(
                alt_complete!(tag!("\\\n") | tag!("\\\r\n")),
                fold_many0!(tag!(" "), (), |_, _| ())
            )
        ) |
        map!(
            preceded!(tag!("\\"), anychar),
            |ch| match ch {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                '\'' => '\'',
                '\"' => '\"',
                '\\' => '\\',
                other => other,
            }
        ) |
        preceded!(not!(peek!(one_of!(concat!("\n", $quote)))), anychar)
    )););
}

str_char!(single_quoted("\'"));
str_char!(double_quoted("\""));

named!(pub parse_string(Input) -> Literal, map!(
    alt_complete!(
        delimited!(
            tag!("\'"),
            fold_many0!(single_quoted, String::new(), |mut s: String, ch| {
                s.push(ch);
                s
            }),
            tag!("\'")
        ) |
        delimited!(
            tag!("\""),
            fold_many0!(double_quoted, String::new(), |mut s: String, ch| {
                s.push(ch);
                s
            }),
            tag!("\"")
        )
    ),
    Literal::Str
));
