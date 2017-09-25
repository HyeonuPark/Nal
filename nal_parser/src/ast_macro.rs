
macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i,
            tuple!(
                position!(),
                $submac!($($args)*),
                position!()
            ),
            |(l, b, r)| {
                let l = $crate::common::convert_span(l);
                let r = $crate::common::convert_span(r);
                ::ast::Ast::new(b, l.merge(&r))
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
