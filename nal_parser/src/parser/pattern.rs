use parse_tree::*;
use super::common::*;

use super::ident::parse_ident;

named!(parse_ident_decl(Input) -> Node<Decl>, node!(map!(
    tuple!(
        optional!(tuple!(word!("mut"), sp)),
        parse_ident
    ),
    |(is_mut, ident)| Decl::Ident { is_mut: is_mut.is_some(), ident }
)));

named!(parse_tuple_elem_delc(Input) -> Node<TupleElemDecl>, node!(
    alt_complete!(
        map!(
            parse_decl,
            TupleElemDecl::Atom
        )
    )
));

named!(pub parse_tuple_decl(Input) -> Node<Decl>, node!(map!(
    block!(
        "(", ",", ")",
        parse_tuple_elem_delc
    ),
    Decl::Tuple
)));

named!(parse_obj_prop_decl(Input) -> Node<ObjPropDecl>, node!(
    alt_complete!(
        map!(
            tuple!(parse_ident, sp, word!("as"), sp, parse_decl),
            |(ident, _, _, _, decl)| ObjPropDecl::Named(ident, decl)
        ) |
        map!(
            parse_ident,
            ObjPropDecl::Short
        )
    )
));

named!(parse_obj_decl(Input) -> Node<Decl>, node!(map!(
    block!(
        "{", ",", "}",
        parse_obj_prop_decl
    ),
    Decl::Obj
)));

named!(pub parse_decl(Input) -> Node<Decl>, alt_complete!(
    parse_tuple_decl |
    parse_obj_decl |
    parse_ident_decl
));

named!(parse_ident_pattern(Input) -> Node<Pattern>, node!(map!(
    parse_ident,
    Pattern::Ident
)));

named!(parse_tuple_elem_pattern(Input) -> Node<TupleElemPattern>, node!(
    alt_complete!(
        map!(
            parse_pattern,
            TupleElemPattern::Atom
        )
    )
));

named!(parse_tuple_pattern(Input) -> Node<Pattern>, node!(map!(
    block!(
        "(", ",", ")",
        parse_tuple_elem_pattern
    ),
    Pattern::Tuple
)));

named!(parse_obj_prop_pattern(Input) -> Node<ObjPropPattern>, node!(
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

named!(parse_obj_pattern(Input) -> Node<Pattern>, node!(map!(
    block!(
        "{", ",", "}",
        parse_obj_prop_pattern
    ),
    Pattern::Obj
)));

named!(pub parse_pattern(Input) -> Node<Pattern>, alt_complete!(
    parse_tuple_pattern
    | parse_obj_pattern
    | parse_ident_pattern
));
