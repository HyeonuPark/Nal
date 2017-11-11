use std::rc::Rc;

use nal_ast::ast::prelude::{Expr, BinaryOp as Bop, UnaryOp as Uop};

use common::prelude::*;
use super::function::eval_call;

impl Eval for Ast<Expr> {
    type Output = ValueRef;

    fn eval(&self, env: &mut Env) -> Result<ValueRef> {
        use self::Expr as X;

        setup!(eval, self, env);
        setup!(eval_tuple[], self, env, *);

        Ok(match ***self {
            X::Literal(_) => eval!(X::Literal(ref t) => t)?,
            X::Binary(op, _, _) if op == Bop::And || op == Bop::Or => {
                eval_short_circuit(env, op, self)?.into()
            }
            X::Binary(op, _, _) => eval_binary(
                op,
                eval!(X::Binary(_, ref t, _) => t)?,
                eval!(X::Binary(_, _, ref t) => t)?,
            )?.into(),
            X::Unary(op, _) => eval_unary(
                op,
                eval!(X::Unary(_, ref t) => t)?,
            )?.into(),
            X::Call(_, _) => eval_call(
                eval!(X::Call(ref t, _) => t)?,
                eval_tuple!(X::Call(_, ref t) => t)?,
            )?.into(),
            X::Prop(_, ref name) => eval_prop(
                eval!(X::Prop(ref t, _) => t)?,
                &name.0,
            )?,
            X::Return(ref ret) => Err(Control::Return(
                if ret.is_some() {
                    eval!(X::Return(ref t) => t.as_ref().unwrap())?.clone()
                } else {
                    Value::Unit
                }
            ))?,
            X::Break => Err(Control::Break)?,
            X::Continue => Err(Control::Continue)?,
            X::Function(_) => eval!(X::Function(ref t) => t)?,
            X::Ident(ref name) => env.get(name)?,
        })
    }
}

fn eval_short_circuit(env: &mut Env, op: Bop, expr: &Ast<Expr>) -> Result<Value> {
    setup!(eval, expr, env);
    const INV_AND: &str = "Invalid type - operands of And op should be bool type";
    const INV_OR: &str = "Invalid type - operands of Or op should be bool type";

    if let Bop::And = op {
        return Ok(match *eval!(Expr::Binary(_, ref t, _) => t)? {
            Value::Bool(false) => Value::Bool(false),
            Value::Bool(true) => match *eval!(Expr::Binary(_, _, ref t) => t)? {
                Value::Bool(v) => Value::Bool(v),
                _ => Err(INV_AND)?,
            }
            _ => Err(INV_AND)?,
        });
    }

    if let Bop::Or = op {
        return Ok(match *eval!(Expr::Binary(_, ref t, _) => t)? {
            Value::Bool(true) => Value::Bool(true),
            Value::Bool(false) => match *eval!(Expr::Binary(_, _, ref t) => t)? {
                Value::Bool(v) => Value::Bool(v),
                _ => Err(INV_OR)?,
            }
            _ => Err(INV_OR)?,
        });
    }

    unreachable!()
}

fn eval_binary(op: Bop, left: ValueRef, right: ValueRef) -> Result<Value> {
    use self::Bop::*;
    use self::Value::*;

    Ok(match (op, &*left, &*right) {
        (Add, &Num(l),  &Num(r) ) => Num(l + r),
        (Sub, &Num(l),  &Num(r) ) => Num(l - r),
        (Mul, &Num(l),  &Num(r) ) => Num(l * r),
        (Div, &Num(l),  &Num(r) ) => Num(l / r),

        (Eq,  &Unit,    &Unit   ) => Bool(true),
        (Eq,  &Num(l),  &Num(r) ) => Bool(l == r),
        (Eq,  &Bool(l), &Bool(r)) => Bool(l == r),
        (Neq, &Unit,    &Unit   ) => Bool(false),
        (Neq, &Num(l),  &Num(r) ) => Bool(l != r),
        (Neq, &Bool(l), &Bool(r)) => Bool(l != r),

        (Gt,  &Num(l),  &Num(r) ) => Bool(l > r),
        (Gte, &Num(l),  &Num(r) ) => Bool(l >= r),
        (Lt,  &Num(l),  &Num(r) ) => Bool(l < r),
        (Lte, &Num(l),  &Num(r) ) => Bool(l <= r),

        (Add, &Str(ref l), &Str(ref r)) => Str(format!("{}{}", l, r).into()),

        // these will handled in eval_short_circuit function above
        (And, _, _) | (Or, _, _) => unreachable!(),

        (Add, l, r) => {
            Err(format!("Invalid type - operands of Add op should be both num \
            or both str, but found {:?} and {:?}", l, r))?
        }

        (Sub, l, r) | (Mul, l, r) | (Div, l, r) => {
            Err(format!("Invalid type - operands of arithmetic op should be \
            num type, but found {:?} and {:?}", l, r))?
        }
        (Eq, &Func(_, _), &Func(_, _)) | (Neq, &Func(_, _), &Func(_, _)) |
        (Eq, &Native(_), &Native(_)) | (Neq, &Native(_), &Native(_)) => {
            Err(format!("Invalid type - function types cannot be compared"))?
        }
        (Eq, l, r) | (Neq, l, r) => {
            Err(format!("Invalid type - both operands of equality op should \
            be equal type, but found {:?} and {:?}", l, r))?
        }
        (Gt, l, r) | (Gte, l, r) | (Lt, l, r) | (Lte, l, r) => {
            Err(format!("Invalid type - operands of comparison op should be \
            num type, but found {:?} and {:?}", l, r))?
        }
})
}

fn eval_unary(op: Uop, expr: ValueRef) -> Result<Value> {
    Ok(match op {
        Uop::Neg => match *expr {
            Value::Num(v) => Value::Num(-v),
            ref other => Err(format!("Invalid type - operand of Neg op should be \
                        num type, but found {:?}", other))?,
        }
        Uop::Not => match *expr {
            Value::Bool(v) => Value::Bool(!v),
            ref other => Err(format!("Invalid type - operand of Not op should be \
                        bool type, but found {:?}", other))?,
        }
    })
}

fn eval_prop<'a>(parent: ValueRef, name: &Rc<str>) -> Result<ValueRef> {

    fn inv_struct(name: &Rc<str>) -> Control {
        format!("Invalid struct - object doesn't have property {}", name).into()
    }
    const PRIM_PROP: &str = "Primitive type's property is not implemented";

    parent.try_map(|parent| match *parent {
        Value::Obj(ref table) => table.get(name).ok_or(inv_struct(name)).map(|v| v.into()),
        _ => Err(PRIM_PROP)?,
    })
}
