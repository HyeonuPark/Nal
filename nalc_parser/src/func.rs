use super::prelude::*;
use super::ident::parse_ident;
use super::stmt::{parse_tuple_pattern, parse_stmt_block};

named!{pub parse_func(Src) -> Node<Function>, node!(map!(
    tuple!(
        tuple!(word!("fn"), sp),
        opt!(parse_ident), sp,
        parse_tuple_pattern, sp,
        parse_stmt_block
    ),
    |(_, name, _, param, _, body)| Function {
        name,
        param,
        body,
    }
))}
