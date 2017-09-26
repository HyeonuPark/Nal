
macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i,
            tuple!(
                position!(),
                $submac!($($args)*),
                position!()
            ),
            |(l, b, r)| {
                $crate::ast::Ast::new(b, $crate::ast::Span(l.offset, r.offset))
            }
        )
    );
}

macro_rules! word {
    ($i:expr, $t:expr) => (
        map!($i,
            tuple!(tag!($t), not!(peek!(
                $crate::ident::ident_char
            ))),
            |(res, _)| res
        )
    );
}
