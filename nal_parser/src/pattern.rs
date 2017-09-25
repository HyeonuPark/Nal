use ast::{Ast, Pattern};

use common::{Input, sp_f};
use ident::parse_ident;

named!(parse_ident_pattern(Input) -> Pattern, map!(
    tuple!(
        opt!(tuple!(tag!("mut"), sp_f)),
        parse_ident
    ),
    |(is_mut, ident)| Pattern::Ident(ident, is_mut.is_some())
));

named!(pub parse_pattern(Input) -> Ast<Pattern>, ast!(alt_complete!(
    parse_ident_pattern
)));
