use super::prelude::*;
use super::expr::parse_expr;
use super::ident::parse_ident;

named!{parse_expr_stmt(Src) -> Node<Stmt>, node!(map!(
    parse_expr,
    Stmt::Expr
))}

named!{parse_declare_stmt(Src) -> Node<Stmt>, node!(map!(
    tuple!(
        tuple!(word!("let"), sp),
        parse_pattern,
        tuple!(sp, tag!("="), sp),
        parse_expr
    ),
    |(_, variable, _, init)| Stmt::Declare { variable, init }
))}

named!{parse_assign_stmt(Src) -> Node<Stmt>, node!(map!(
    tuple!(
        parse_expr,
        tuple!(sp, tag!("="), sp),
        parse_expr
    ),
    |(target, _, value)| Stmt::Assign { target, value }
))}

named!{parse_if_stmt(Src) -> Node<Stmt>, node!(map!(
    tuple!(
        separated_nonempty_list!(
            tuple!(nl, word!("else"), sp),
            parse_if_block
        ),
        alt!(
            map!(
                tuple!(nl, word!("else"), sp, parse_stmt_block),
                |(_, _, _, stmt)| ElseCase::Else(stmt)
            )
            | value!(ElseCase::Omit, tag!(""))
        )
    ),
    |(heads, last)| {
        let mut it = heads.into_iter().rev();
        let (condition, then) = it.next().unwrap();
        let last_if = IfStmt {
            condition,
            then,
            else_case: last,
        };

        let if_stmt = it.fold(last_if, |else_if, (condition, then)| IfStmt {
            condition,
            then,
            else_case: ElseCase::ElseIf(else_if.into()),
        });
        Stmt::If(if_stmt)
    }
))}

named!{parse_if_block(Src) -> (Node<Expr>, Block<Stmt>), map!(
    tuple!(
        tuple!(word!("if"), sp),
        parse_expr, sp,
        parse_stmt_block
    ),
    |(_, expr, _, stmt)| (expr, stmt)
)}

named!{parse_while_stmt(Src) -> Node<Stmt>, node!(map!(
    tuple!(word!("while"), sp, parse_expr, sp, parse_stmt_block),
    |(_, _, condition, _, body)| Stmt::While { condition, body }
))}

named!{pub parse_stmt(Src) -> Node<Stmt>, alt!(
    parse_if_stmt
    | parse_while_stmt
    | parse_declare_stmt
    | parse_assign_stmt
    | parse_expr_stmt
)}

named!{pub parse_stmt_block(Src) -> Block<Stmt>, block!(
    "{", "}",
    comma_sep,
    parse_stmt
)}

named!{parse_void_pattern(Src) -> Node<Pattern>, node!(
    value!(Pattern::Void, word!("_"))
)}

named!{parse_variable_pattern(Src) -> Node<Pattern>, node!(map!(
    parse_ident,
    Pattern::Variable
))}

named!{pub parse_tuple_pattern(Src) -> Node<Pattern>, node!(map!(
    block!(
        "(", ")",
        alt!(
            tuple!(sp, tag!(","), nl) => {noop}
            | comma_sep => {noop}
        ),
        parse_pattern
    ),
    Pattern::Tuple
))}

named!{parse_obj_pattern(Src) -> Node<Pattern>, node!(map!(
    block!(
        "{", "}",
        comma_sep,
        tuple!(
            parse_ident,
            preceded!(
                tuple!(sp, word!("as"), sp),
                parse_pattern
            )
        )
    ),
    Pattern::Obj
))}

named!{pub parse_pattern(Src) -> Node<Pattern>, alt!(
    parse_tuple_pattern
    | parse_obj_pattern
    | parse_void_pattern
    | parse_variable_pattern
)}
