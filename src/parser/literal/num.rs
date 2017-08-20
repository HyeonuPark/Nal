use nom::{IResult, digit, double_s};

use ast::Src;
use ast::front::{Ast, Literal};

named!(
    integer_inner(&str) -> i32,
    map!(
        recognize!(tuple!(
            opt!(char!('-')),
            not!(char!('0')),
            digit
        )),
        |tok| tok.parse().unwrap()
    )
);

pub fn integer<'a>(input: &'a str, src: Src) -> IResult<&'a str, Ast<Literal>> {
    integer_inner(input).map(|tok| {
        let offset = src.as_
    })
}
