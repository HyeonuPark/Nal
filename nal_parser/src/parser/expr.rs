use parse_tree::*;
use super::common::*;

use super::literal::parse_literal;
use super::compound::parse_tuple_literal;
use super::ident::parse_ident;

named!(parse_atom_expr(Input) -> Span<Expr>, span!(alt_complete!(
      map!(parse_literal, Expr::Literal)
    | value!(Expr::Break, word!("break"))
    | value!(Expr::Continue, word!("continue"))
    | map!(parse_ident, Expr::Ident)
)));

#[derive(Debug)]
enum Attachment {
    Call(Block<TupleElem>),
    Prop(Span<Ident>),
}

named!(parse_primary_attachment(Input) -> Attachment, alt_complete!(
    map!(
        preceded!(
            sp,
            parse_tuple_literal
        ),
        Attachment::Call
    )
    | map!(
        preceded!(
            tuple!(nl, tag!("."), sp),
            parse_ident
        ),
        Attachment::Prop
    )
));

named!(parse_primary_expr(Input) -> Span<Expr>, do_parse!(
    head: parse_atom_expr >>
    res: fold_many0!(
        parse_primary_attachment,
        head,
        |head: Span<Expr>, attachment| {
            use self::Attachment as A;

            let lspan = head.span;
            let (rspan, expr) = match attachment {
                A::Call(args) => (
                    args.span,
                    Expr::Call(head, args)
                ),
                A::Prop(field) => (
                    field.span,
                    Expr::Prop(head, field)
                ),
            };

            Span::new(lspan + rspan, expr)
        }
    ) >>
    (res)
));

named!(parse_unary_op(Input) -> UnaryOp, alt_complete!(
      value!(UnaryOp::Neg, tag!("-"))
    | value!(UnaryOp::Not, tag!("!"))
));

named!(parse_unary_expr(Input) -> Span<Expr>, span!(map!(
    tuple!(parse_unary_op, sp, parse_primary_expr),
    |(op, _, expr)| Expr::Unary(op, expr)
)));

named!(parse_binary_op(Input) -> BinaryOp, alt_complete!(
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
));

named!(parse_binary_expr(Input) -> Span<Expr>, map!(
    tuple!(
        parse_unary_expr,
        many0!(map!(
            tuple!(sp, parse_binary_op, sp, parse_unary_expr),
            |(_, op, _, expr)| (op, expr)
        ))
    ),
    |(head, tail)| precedence_parser(head, &mut tail.into_iter().peekable(), 0)
));

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

type AttachIter = ::std::vec::IntoIter<(BinaryOp, Span<Expr>)>;
type Remain = ::std::iter::Peekable<AttachIter>;

fn precedence_parser(
    head: Span<Expr>, remain: &mut Remain, min_prec: usize
) -> Span<Expr> {
    let mut head = head;

    while let Some(&(op, _)) = remain.peek() {
        if prec(op) < min_prec {
            break;
        }

        let (op, mut next) = remain.next().unwrap();

        while let Some(&(next_op, _)) = remain.peek() {
            if prec(next_op) <= prec(op) {
                break;
            }

            next = precedence_parser(next, remain, prec(next_op));
        }

        head = Span::new(
            head.span + next.span,
            Expr::Binary(op, head, next),
        );
    }

    head
}

enum TagLike {
    Tag(Span<Ident>),
    Return,
}

named!(parse_taglike(Input) -> TagLike, alt_complete!(
    map!(delimited!(tag!(":"), parse_ident, sp), TagLike::Tag)
    | value!(TagLike::Return, tuple!(word!("return"), sp))
));

named!(parse_tagged_expr(Input) -> Span<Expr>, alt_complete!(
    map!(
        tuple!(
            many1!(tuple!(span!(tag!("")), parse_taglike)),
            optional!(parse_binary_expr),
            span!(tag!(""))
        ),
        |(tags, expr, end)| tags.into_iter().rev().fold(
            expr,
            |prev, (marker, tag)| Some(Span::new(
                marker.span + end.span,
                match tag {
                    TagLike::Tag(ident) => Expr::Tagged(ident, prev),
                    TagLike::Return => Expr::Return(prev),
                }
            ))
        ).unwrap()
    )
    | parse_binary_expr
));

named!(pub parse_expr(Input) -> Span<Expr>, alt_complete!(
    parse_tagged_expr
));
