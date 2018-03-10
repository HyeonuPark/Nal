use super::prelude::*;
use nom::{IResult, Err as NomErr, Needed};

fn is_space(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r'
}

named!{pub sp(Src) -> Src, recognize!(
    take_while!(is_space)
)}

named!{pub nl(Src) -> Src, recognize!(
    tuple!(
        sp,
        void_many0!(tuple!(tag!("\n"), sp))
    )
)}

named!{pub line_sep(Src) -> (), map!(
    tuple!(
        sp,
        void_many1!(tuple!(tag!("\n"), sp))
    ),
    noop
)}

named!{pub comma_sep(Src) -> (), alt!(
    tuple!(sp, tag!(","), nl) => {noop}
    | line_sep => {noop}
)}

pub fn uni_char(input: Src) -> IResult<Src, char> {
    use nom::{Slice, InputLength};

    let mut it = input.0.char_indices();

    match it.next() {
        None => Err(NomErr::Incomplete(Needed::Unknown)),
        Some((_, c)) => match it.next() {
            None => Ok((input.slice(input.input_len()..), c)),
            Some((pos, _)) => Ok((input.slice(pos..), c)),
        }
    }
}
