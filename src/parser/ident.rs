use std::rc::Rc;

use nom::{IResult, Offset};

use ast::{self, Src};
use ast::front::*;

pub const IDENT_CHARS: &'static str
    = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";

named!(
    keywords(&str) -> &str,
    preceded!(
        alt_complete!(
            tag_s!("_") |
            tag_s!("break") |
            tag_s!("continue") |
            tag_s!("enum") |
            tag_s!("export") |
            tag_s!("false") |
            tag_s!("fn") |
            tag_s!("for") |
            tag_s!("from") |
            tag_s!("if") |
            tag_s!("import") |
            tag_s!("in") |
            tag_s!("is") |
            tag_s!("let") |
            tag_s!("match") |
            tag_s!("mut") |
            tag_s!("obj") |
            tag_s!("return") |
            tag_s!("trait") |
            tag_s!("true") |
            tag_s!("type") |
            tag_s!("while") |
            tag_s!("with")
        ),
        not!(one_of!(IDENT_CHARS))
    )
);

named!(
    ident_inner(&str) -> &str,
    preceded!(
        not!(keywords),
        is_a_s!(IDENT_CHARS)
    )
);

pub fn ident<'a>(input: &'a str, src: Src) -> IResult<&'a str, Ast<Ident>> {
    ident_inner(input).map(|tok| {
        let offset = src.as_str().offset(tok);
        ast::create(Ident(tok.into()), src, offset, offset + tok.len() - 1)
    })
}

#[test]
fn test_ident() {
    assert_eq!(ident_inner("foo bar"), IResult::Done(" bar", "foo"));
    assert_eq!(ident_inner("foo[bar]"), IResult::Done("[bar]", "foo"));
    
    assert!(ident_inner("is foo").is_err());
    assert_eq!(ident_inner("is_foo"), IResult::Done("", "is_foo"));
}
