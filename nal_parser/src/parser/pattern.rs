use parse_tree::*;
use super::common::*;

use super::ident::parse_ident;

named!(parse_ident_pattern(Input) -> Ast<Pattern>, ast!(map!(
    tuple!(
        optional!(tuple!(word!("mut"), sp)),
        parse_ident
    ),
    |(is_mut, ident)| Pattern::Ident { is_mut: is_mut.is_some(), ident }
)));

named!(parse_tuple_elem_pattern(Input) -> Ast<TupleElemPattern>, ast!(
    alt_complete!(
        map!(
            parse_pattern,
            TupleElemPattern::Atom
        )
        | map!(
            tuple!(tag!("..."), sp, parse_ident),
            |(_, _, ident)| TupleElemPattern::Spread(ident)
        )
    )
));

named!(pub parse_tuple_pattern(Input) -> Ast<Pattern>, ast!(map!(
    block!(
        "(", ",", ")",
        parse_tuple_elem_pattern
    ),
    Pattern::Tuple
)));

named!(parse_obj_prop_pattern(Input) -> Ast<ObjPropPattern>, ast!(
    alt_complete!(
        map!(
            tuple!(parse_ident, sp, word!("as"), sp, parse_pattern),
            |(ident, _, _, _, pat)| ObjPropPattern::Named(ident, pat)
        )
        | map!(
            parse_ident,
            ObjPropPattern::Short
        )
    )
));

named!(parse_obj_pattern(Input) -> Ast<Pattern>, ast!(map!(
    block!(
        "{", ",", "}",
        parse_obj_prop_pattern
    ),
    Pattern::Obj
)));

named!(pub parse_pattern(Input) -> Ast<Pattern>, alt_complete!(
    parse_tuple_pattern
    | parse_obj_pattern
    | parse_ident_pattern
));
