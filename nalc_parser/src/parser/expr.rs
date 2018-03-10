use nom::digit;

use super::prelude::*;
use super::ident::parse_ident;
use super::literal::parse_literal;
use super::func::parse_func;

named!{parse_var_expr(Src) -> Node<Expr>, node!(map!(
    parse_ident,
    Expr::Variable
))}

named!{parse_lit_expr(Src) -> Node<Expr>, node!(map!(
    parse_literal,
    Expr::Literal
))}

named!{parse_tuple_expr(Src) -> Node<Expr>, node!(map!(
    block!(
        "(", ")",
        comma_sep,
        parse_tuple_elem
    ),
    Expr::Tuple
))}

named!{parse_tuple_elem(Src) -> Node<TupleElem>, node!(alt!(
    map!(parse_expr, TupleElem::Atom)
))}

named!{parse_obj_expr(Src) -> Node<Expr>, node!(map!(
    block!(
        "{", "}",
        comma_sep,
        parse_obj_elem
    ),
    Expr::Obj
))}

named!{parse_obj_elem(Src) -> Node<ObjElem>, node!(alt!(
    map!(
        tuple!(parse_ident, tuple!(sp, tag!(":"), sp), parse_expr),
        |(name, _, value)| ObjElem::Named(name, value)
    )
))}

named!{parse_func_expr(Src) -> Node<Expr>, node!(map!(
    parse_func,
    Expr::Function
))}

named!{parse_control_expr(Src) -> Node<Expr>, node!(alt!(
    value!(Expr::Break, word!("break"))
    | value!(Expr::Continue, word!("Continue"))
))}

named!{parse_atom_expr(Src) -> Node<Expr>, alt!(
    parse_tuple_expr
    | parse_obj_expr
    | parse_func_expr
    | parse_lit_expr
    | parse_control_expr
    | parse_var_expr
)}

enum Attachment {
    Call(Node<Expr>),
    ObjField(Node<Ident>),
    TupleField(Node<usize>),
}

named!{parse_attachment(Src) -> Node<Attachment>, node!(alt!(
    parse_tuple_expr => {Attachment::Call} |
    map!(
        tuple!(nl, tag!("."), parse_ident),
        |(_, _, name)| Attachment::ObjField(name)
    ) |
    map!(
        tuple!(
            nl, tag!("."),
            node!(map!(digit, |idx| idx.0.parse::<usize>().unwrap()))
        ),
        |(_, _, idx)| Attachment::TupleField(idx)
    )
))}

fn attach_primary(base: Node<Expr>, attach: Node<Attachment>) -> Node<Expr> {
    use self::Attachment as A;

    Node::new(
        base.span() + attach.span(),
        match attach.into() {
            A::Call(arg) => Expr::Call {
                callee: base,
                argument: arg,
            },
            A::ObjField(field) => Expr::ObjField {
                parent: base,
                field,
            },
            A::TupleField(field) => Expr::TupleField {
                parent: base,
                field,
            },
        }
    )
}

named!{parse_primary_expr(Src) -> Node<Expr>, do_parse!(
    base: parse_atom_expr >>
    res: fold_many0!(parse_attachment, base, attach_primary) >>
    (res)
)}

named!{parse_unary_op(Src) -> Node<UnaryOp>, node!(alt!(
    value!(UnaryOp::Neg, tag!("-")) |
    value!(UnaryOp::Not, word!("not"))
))}

named!{parse_unary_expr(Src) -> Node<Expr>, map!(
    tuple!(
        separated_list!(sp, parse_unary_op),
        sp, parse_primary_expr
    ),
    |(ops, _, expr)| ops.into_iter().rev().fold(expr, |expr, op| {
        Node::new(
            op.span() + expr.span(),
            Expr::Unary(op.into(), expr),
        )
    })
)}

named!{parse_binary_op(Src) -> BinaryOp, alt!(
      value!(BinaryOp::Add, tag!("+"))
    | value!(BinaryOp::Sub, tag!("-"))
    | value!(BinaryOp::Mul, tag!("*"))
    | value!(BinaryOp::Div, tag!("/"))
    | value!(BinaryOp::Eq,  tag!("=="))
    | value!(BinaryOp::Neq, tag!("!="))
    | value!(BinaryOp::Gte, tag!(">="))
    | value!(BinaryOp::Gt,  tag!(">"))
    | value!(BinaryOp::Lte, tag!("<="))
    | value!(BinaryOp::Lt,  tag!("<"))
    | value!(BinaryOp::And, tag!("&&"))
    | value!(BinaryOp::Or,  tag!("||"))
)}

fn prec(op: BinaryOp) -> usize {
    use self::BinaryOp::*;

    match op {
        Or => 1,
        And => 2,
        Eq | Neq => 3,
        Gt | Gte | Lt | Lte => 4,
        Add | Sub => 5,
        Mul | Div => 6,
    }
}

mod iter {
    use super::*;
    use std::iter::Peekable;
    use std::vec::IntoIter;
    pub type Iter = Peekable<IntoIter<(BinaryOp, Node<Expr>)>>;
}

fn parse_precedence(mut head: Node<Expr>, tail: &mut iter::Iter, min_prec: usize) -> Node<Expr> {
    while let Some(&(op, _)) = tail.peek() {
        if prec(op) <= min_prec {
            break;
        }

        let (op, mut next) = tail.next().unwrap();

        while let Some(&(next_op, _)) = tail.peek() {
            if prec(next_op) <= prec(op) {
                break;
            }

            next = parse_precedence(next, tail, prec(next_op));
        }

        head = Node::new(
            head.span() + next.span(),
            Expr::Binary(op.into(), head, next),
        )
    }

    head
}

named!{parse_binary_expr(Src) -> Node<Expr>, map!(
    tuple!(
        parse_unary_expr,
        many0!(tuple!(
            parse_binary_op,
            parse_unary_expr
        ))
    ),
    |(head, tail)| parse_precedence(head, &mut tail.into_iter().peekable(), 0)
)}

named!{pub parse_expr(Src) -> Node<Expr>, alt!(
    node!(map!(
        tuple!(word!("return"), sp, parse_binary_expr),
        |(_, _, expr)| Expr::Return(Some(expr))
    ))
    | node!(value!(Expr::Return(None), word!("return")))
    | parse_binary_expr
)}
