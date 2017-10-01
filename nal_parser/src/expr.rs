use std::{iter, vec};

use ast::{Ast, Expr, BinaryOp, UnaryOp};

use literal::parse_literal;
use ident::parse_ident;
use common::{Input, nl, sp};

named!(parse_atom_expr(Input) -> Ast<Expr>, ast!(alt_complete!(
    map!(parse_literal, Expr::Literal) |
    map!(parse_ident, Expr::Ident) |
    map!(
        tuple!(tag!("("), nl, parse_expr, nl, tag!(")")),
        |(_, _, expr, _, _)| expr.into_inner()
    )
)));

named!(parse_unary_op(Input) -> UnaryOp, alt_complete!(
    value!(UnaryOp::Not, tag!("!")) |
    value!(UnaryOp::Neg, tag!("-"))
));

named!(parse_unary_expr(Input) -> Ast<Expr>, alt_complete!(
    ast!(map!(
        tuple!(parse_unary_op, sp, parse_atom_expr),
        |(op, _, expr)| Expr::Unary(op, expr)
    )) |
    parse_atom_expr
));

named!(parse_binary_op(Input) -> BinaryOp, alt_complete!(
    value!(BinaryOp::Add, tag!("+"))  |
    value!(BinaryOp::Sub, tag!("-"))  |
    value!(BinaryOp::Mul, tag!("*"))  |
    value!(BinaryOp::Div, tag!("/"))  |
    value!(BinaryOp::Eq,  tag!("==")) |
    value!(BinaryOp::Neq, tag!("!=")) |
    value!(BinaryOp::Gte, tag!(">=")) |
    value!(BinaryOp::Gt,  tag!(">"))  |
    value!(BinaryOp::Lte, tag!("<=")) |
    value!(BinaryOp::Lt,  tag!("<"))  |
    value!(BinaryOp::And, tag!("&&")) |
    value!(BinaryOp::Or,  tag!("||"))
));

named!(parse_binary_expr(Input) -> Ast<Expr>, map!(
    tuple!(parse_unary_expr, many0!(map!(
        tuple!(sp, parse_binary_op, sp, parse_unary_expr),
        |(_, op, _, expr)| (op, expr)
    ))),
    |(head, tail)| parse_prec(head, &mut tail.into_iter().peekable(), 0)
));

trait Operator {
    fn precedence(&self) -> usize;
}

impl Operator for BinaryOp {
    fn precedence(&self) -> usize {
        use self::BinaryOp::*;

        match *self {
            Or => 1,
            And => 2,
            Eq | Neq => 3,
            Gt | Gte | Lt | Lte => 4,
            Add | Sub => 5,
            Mul | Div => 6,
        }
    }
}

type Tail = iter::Peekable<vec::IntoIter<(BinaryOp, Ast<Expr>)>>;

fn parse_prec(head: Ast<Expr>, tail: &mut Tail, min_prec: usize) -> Ast<Expr> {
    let mut head = head;

    while let Some(&(op, _)) = tail.peek() {
        if op.precedence() < min_prec {
            break;
        }

        let (op, mut next) = tail.next().unwrap();

        while let Some(&(next_op, _)) = tail.peek() {
            if next_op.precedence() <= op.precedence() {
                break;
            }

            next = parse_prec(next, tail, next_op.precedence());
        }

        let span = head.span + next.span;
        head = Ast::new(Expr::Binary(op, head, next), span);
    }

    head
}

named!(pub parse_expr(Input) -> Ast<Expr>, alt_complete!(
    parse_binary_expr
));
