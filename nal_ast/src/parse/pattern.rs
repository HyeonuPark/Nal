use ast::common::{Ast, Ident};
use ast::stmt::Pattern;

use super::common::{Input, sp, sp_f, nl};
use super::ident::parse_ident;
use super::expr::parse_expr_sep;

named!(parse_ident_pattern(Input) -> Pattern, map!(
    tuple!(
        opt!(tuple!(tag!("mut"), sp_f)),
        parse_ident
    ),
    |(is_mut, ident)| Pattern::Ident(ident, is_mut.is_some())
));

named!(parse_obj_pattern(Input) -> Pattern, map!(
    delimited!(
        tuple!(tag!("{"), nl),
        separated_list!(
            parse_expr_sep,
            alt_complete!(
                tuple!(parse_ident, sp, word!("as"), sp, parse_pattern) => {
                    |(name, _, _, _, pat)| (name, pat)
                } |
                tuple!(word!("mut"), sp, parse_ident) => {
                    |(_, _, name)| {
                        let name = name as Ast<Ident>;
                        let name2 = name.clone();
                        let pat = name.clone()
                            .map(|_| Pattern::Ident(name2, true));

                        (name, pat)
                    }
                } |
                parse_ident => {
                    |name| {
                        let name = name as Ast<Ident>;
                        let name2 = name.clone();
                        let pat = name.clone()
                            .map(|_| Pattern::Ident(name2, false));

                        (name, pat)
                    }
                }
            )
        ),
        tuple!(nl, tag!("}"))
    ),
    |elems| Pattern::Obj(elems)
));

named!(pub parse_pattern(Input) -> Ast<Pattern>, ast!(alt_complete!(
    parse_ident_pattern |
    parse_obj_pattern
)));
