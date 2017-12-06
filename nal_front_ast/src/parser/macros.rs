macro_rules! optional {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        alt_complete!($i,
            map!($submac!($($args)*), Some)
            | value!(None, tag!(""))
        )
    );
    ($i:expr, $f:expr) => (optional!($i, call!($f)));
}

macro_rules! word {
    ($i:expr, $word:expr) => (
        map!($i,
            tuple!(
                tag!($word),
                peek!(alt_complete!(
                    not!($crate::parser::ident::ident_char)
                    | eof!()
                ))
            ),
            noop
        )
    );
}

macro_rules! ast {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        map!($i,
            tuple!(
                position!(),
                $submac!($($args)*),
                position!()
            ),
            |(left, res, right)| {
                use codebuf::Span;
                use $crate::ast::Ast;

                Ast::new(Span::new(left.offset, right.offset), res)
            }
        )
    );
    ($i:expr, $f:expr) => (ast!($i, call!($f)));
}

macro_rules! opt_line {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        do_parse!($i,
            content: optional!($submac!($($args)*)) >>
            failed: cond!(
                content.is_none(),
                ast!(map!(
                    take_until_and_consume_s!("\n"),
                    noop
                ))
            ) >>
            (content.ok_or_else(|| failed.unwrap()))
        )
    );
    ($i:expr, $f:expr) => (opt_line!($i, call!($f)));
}

macro_rules! block {
    ($i:expr,
        $left:expr, $sep:expr, $right:expr,
        $submac:ident!( $($args:tt)* )
    ) => (
        ast!($i,
            delimited!(
                tuple!(tag!($left), nl),
                separated_list_complete!(
                    alt_complete!(
                        tuple!(sp, tag!($sep), sp) => {noop}
                        | nl
                    ),
                    opt_line!($submac!($($args)*))
                ),
                tuple!(nl, tag!($right))
            )
        )
    );
    ($i:expr, $left:expr, $sep:expr, $right:expr, $f:expr) => (
        block!($i, $left, $sep, $right, call!($f))
    );
}
