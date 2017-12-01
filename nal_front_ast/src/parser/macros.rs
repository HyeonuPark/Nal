macro_rules! opt_complete {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        opt!($i, complete!($submac!($($args)*)))
    );
    ($i:expr, $f:expr) => (opt_complete!($i, call!($f)));
}

macro_rules! word {
    ($i:expr, $word:expr) => (map!($i,
        tuple!(
            tag!($word),
            peek!(alt_complete!(
                not!(alphanumeric)
                | eof!()
            ))
        ),
        noop
    ));
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

                Ast::new(res, Span::new(left.offset, right.offset))
            }
        )
    );
    ($i:expr, $f:expr) => (ast!($i, call!($f)));
}

macro_rules! line {
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        do_parse!($i,
            content: opt_complete!(ast!($submac!($($args)*))) >>
            failed: cond!(
                content.is_none(),
                ast!(map!(
                    take_until_and_consume_s!("\n"),
                    |_| ()
                ))
            ) >>
            (content.ok_or_else(|| failed.unwrap()))
        )
    );
    ($i:expr, $f:expr) => (line!($i, call!($f)));
}

macro_rules! block {
    ($i:expr,
        $left:expr, $sep:expr, $right:expr,
        $submac:ident!( $($args:tt)* )
    ) => (
        delimited!($i,
            tuple!(tag!($left), nl),
            separated_list_complete!(
                alt_complete!(
                    tuple!(sp, tag!($sep), sp) => {noop}
                    | nl
                ),
                line!($submac!($($args)*))
            ),
            tuple!(nl, tag!($right))
        )
    );
    ($i:expr, $left:expr, $sep:expr, $right:expr, $f:expr) => (
        block!($i, $left, $sep, $right, call!($f))
    );
}
