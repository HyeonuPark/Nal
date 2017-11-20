use std::rc::Rc;
use std::f64::EPSILON as EPS;

use common::prelude::*;
use super::function::eval_call;

use self::ast::{Expr, BinaryOp as Bop, UnaryOp as Uop};

impl Eval for Expr {
    type Output = ValueRef;

    fn eval(&self, env: &mut Env) -> Result<ValueRef> {
        use self::Expr as X;

        Ok(match *self {
            X::Literal(ref lit) => lit.eval(env)?.into(),
            X::Binary(op, ref left, ref right)
                if op == Bop::And || op == Bop::Or => {
                    eval_short_circuit(env, op, left, right)?.into()
                }
            X::Binary(op, ref left, ref right) => {
                eval_binary(op, left.eval(env)?, right.eval(env)?)?.into()
            }
            X::Unary(op, ref operand) => {
                eval_unary(op, operand.eval(env)?)?.into()
            }
            X::Call(ref callee, ref args) => {
                let args = args.iter()
                    .map(|a| a.eval(env))
                    .collect::<Result<Vec<_>>>()?;

                eval_call(callee.eval(env)?, args)?.into()
            }
            X::Prop(ref parent, ref ident) => {
                eval_prop(parent.eval(env)?, ident.name())?
            }
            X::Return(ref retval) => Err(Control::Return(
                match *retval {
                    Some(ref retval) => retval.eval(env)?.into(),
                    None => V::Unit,
                }
            ))?,
            X::Break => Err(Control::Break)?,
            X::Continue => Err(Control::Continue)?,
            X::Function(ref func) => func.eval(env)?.into(),
            X::Ident(ref name) => env.get(name)?,
        })
    }
}

fn eval_short_circuit(
    env: &mut Env, op: Bop,
    left: &Expr, right: &Expr,
) -> Result<Value> {
    const INV_AND: &str = "Invalid type - operands of And op should be bool type";
    const INV_OR: &str = "Invalid type - operands of Or op should be bool type";

    if let Bop::And = op {
        return Ok(match *left.eval(env)? {
            V::Bool(false) => V::Bool(false),
            V::Bool(true) => match *right.eval(env)? {
                V::Bool(v) => V::Bool(v),
                _ => Err(INV_AND)?,
            }
            _ => Err(INV_AND)?,
        });
    }

    if let Bop::Or = op {
        return Ok(match *left.eval(env)? {
            V::Bool(true) => V::Bool(true),
            V::Bool(false) => match *right.eval(env)? {
                V::Bool(v) => V::Bool(v),
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
        (Eq,  &Num(l),  &Num(r) ) => Bool((l - r).abs() < EPS),
        (Eq,  &Bool(l), &Bool(r)) => Bool(l == r),
        (Neq, &Unit,    &Unit   ) => Bool(false),
        (Neq, &Num(l),  &Num(r) ) => Bool((l - r).abs() >= EPS),
        (Neq, &Bool(l), &Bool(r)) => Bool(l != r),

        (Gt,  &Num(l),  &Num(r) ) => Bool(l > r),
        (Gte, &Num(l),  &Num(r) ) => Bool(l >= r),
        (Lt,  &Num(l),  &Num(r) ) => Bool(l < r),
        (Lte, &Num(l),  &Num(r) ) => Bool(l <= r),

        (Add, &Str(ref l), &Str(ref r)) => Str(format!("{}{}", l, r)),

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
            Err("Invalid type - function types cannot be compared")?
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
            V::Num(v) => V::Num(-v),
            ref other => Err(format!("Invalid type - operand of Neg op should be \
                        num type, but found {:?}", other))?,
        }
        Uop::Not => match *expr {
            V::Bool(v) => V::Bool(!v),
            ref other => Err(format!("Invalid type - operand of Not op should be \
                        bool type, but found {:?}", other))?,
        }
    })
}

fn eval_prop(parent: ValueRef, name: Rc<str>) -> Result<ValueRef> {
    parent.try_map(|parent| match *parent {
        V::Obj(ref table) => table.get(&name)
            .ok_or_else(|| format!(
                "Invlaid struct - object doesn't have property {}",
                name,
            ).into()),

        V::Unit | V::Num(_) | V::Bool(_) | V::Str(_) | V::Func(_, _) | V::Native(_)
            => Err("Primitive type's property is not implemented")?,
    })
}
