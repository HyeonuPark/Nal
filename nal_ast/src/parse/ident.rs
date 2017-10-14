use nom::{alpha, alphanumeric};

use ast::common::{Ast, Ident};

use super::common::{Input, noop};

named!(ident_head(Input) -> (), alt_complete!(
    map!(alpha, noop) |
    map!(one_of!("_"), noop)
));

named!(pub ident_char(Input) -> (), alt_complete!(
    map!(alphanumeric, noop) |
    map!(one_of!("_"), noop)
));

named!(pub parse_ident(Input) -> Ast<Ident>, ast!(map!(
        recognize!(tuple!(
            ident_head,
            fold_many0!(ident_char, (), |_, _| ())
        )),
    |name| Ident(name.fragment.into())
)));
