macro_rules! node {
    ($i:expr, $submac:ident!($($args:tt)*)) => (
        map!($i,
            $submac!($($args)*),
            Node::dummy
        )
    );
    ($i:expr, $func:expr) => (node!($i, call!($func)));
}

macro_rules! block {
    ($i:expr, $left:expr, $right:expr, $sep:ident!($($arg1:tt)*), $elem:ident!($($arg2:tt)*)) => (
        map!($i,
            delimited!(
                tuple!(tag!($left), nl),
                separated_list!(
                    $sep!($($arg1)*),
                    $elem!($($arg2)*)
                ),
                tuple!(nl, tag!($right))
            ),
            Node::dummy
        )
    );
    ($i:expr, $left:expr, $right:expr, $sep:ident!($($arg1:tt)*), $elem:expr) => (
        block!($i, $left, $right, $sep!($($arg1)*), call!($elem))
    );
    ($i:expr, $left:expr, $right:expr, $sep:expr, $elem:ident!($($arg2:tt)*)) => (
        block!($i, $left, $right, call!($sep), $elem!($($arg2)*))
    );
    ($i:expr, $left:expr, $right:expr, $sep:expr, $elem:expr) => (
        block!($i, $left, $right, call!($sep), call!($elem))
    );
}

macro_rules! word {
    ($i:expr, $word:expr) => (
        recognize!($i,
            tuple!(
                tag!($word),
                not!(one_of!(super::ident::IDENT_CHARS))
            )
        )
    );
}

macro_rules! opt {
    ($i:expr, $submac:ident!($($args:tt)*)) => (
        alt!($i,
            map!($submac!($($args)*), Some)
            | value!(None, tag!(""))
        )
    );
    ($i:expr, $func:expr) => (opt!($i, call!($func)));
}

macro_rules! uni_none_of {
    ($i:expr, $filter:expr) => ({
        use ::nom::{FindToken, Err as NomErr};
        use ::nom::ErrorKind::NoneOf;
        match $crate::common::uni_char($i) {
            Err(e) => Err(e),
            Ok((left, ch)) => match $filter.find_token(ch) {
                true => Err(NomErr::Error(error_position!($i, NoneOf::<u32>))),
                false => Ok((left, ch)),
            }
        }
    });
}

macro_rules! void_many0 {
    ($i:expr, $submac:ident!($($args:tt)*)) => (
        fold_many0!($i,
            $submac!($($args)*),
            (), |_, _| ()
        )
    );
    ($i:expr, $func:expr) => (void_many0!($i, call!($func)));
}

macro_rules! void_many1 {
    ($i:expr, $submac:ident!($($args:tt)*)) => (
        fold_many1!($i,
            $submac!($($args)*),
            (), |_, _| ()
        )
    );
    ($i:expr, $func:expr) => (void_many0!($i, call!($func)));
}
