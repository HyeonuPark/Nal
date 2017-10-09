
macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i,
            tuple!(
                position!(),
                $submac!($($args)*),
                position!()
            ),
            |(l, b, r)| {
                $crate::ast::common::Ast::new(b, $crate::ast::common::Span(l.offset, r.offset - 1))
            }
        )
    );
    ($i:expr, $e:expr) => (
        ast!($i, call!($e))
    );
}

macro_rules! word {
    ($i:expr, $t:expr) => (
        map!($i,
            tuple!(tag!($t), not!(peek!(
                $crate::parser::ident::ident_char
            ))),
            |(res, _)| res
        )
    );
}
