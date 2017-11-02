use nal_ast::ast::prelude::{Expr, BinaryOp, UnaryOp};

use common::{Eval, Env, Ast, Value, Control, Result};
use super::function::eval_call;

use self::Expr::*;

impl Eval for Ast<Expr> {
    type Output = Value;

    fn eval(&self, env: &mut Env) -> Result<Value> {
        setup_mapto!(mapto, self, env);
        setup_mapto!(mapto_vec[], self, env);
        match ***self {
            Literal(_) => mapto!(Literal(ref t) => t),
            Binary(op, _, _) if op == BinaryOp::And || op == BinaryOp::Or => {
                eval_short_circuit(env, op, self)
            }
            Binary(op, _, _) => eval_binary(
                op,
                mapto!(Binary(_, ref t, _) => t)?,
                mapto!(Binary(_, _, ref t) => t)?,
            ),
            Unary(op, _) => eval_unary(
                op,
                mapto!(Unary(_, ref t) => t)?,
            ),
            Call(_, _) => eval_call(
                mapto!(Call(ref t, _) => t)?,
                mapto_vec!(Call(_, ref t) => t)?
            ),
            Return(ref ret_val) => Err(Control::Return(
                if ret_val.is_some() {
                    mapto!(Return(ref t) => t.as_ref().unwrap())?
                } else {
                    Value::Unit
                }
            )),
            Break => Err(Control::Break),
            Continue => Err(Control::Continue),
            Function(_) => mapto!(Function(ref t) => t),
            Ident(ref name) => env.get(name),
        }
    }
}

fn eval_short_circuit(env: &mut Env, op: BinaryOp, expr: &Ast<Expr>) -> Result<Value> {
    setup_mapto!(mapto, expr, env);
    let left = mapto!(Binary(_, ref t, _) => t)?;

    Ok(match (op, left) {
        (BinaryOp::And, Value::Bool(false)) => Value::Bool(false),
        (BinaryOp::And, Value::Bool(_)) => {
            match mapto!(Binary(_, _, ref t) => t)? {
                Value::Bool(v) => Value::Bool(v),
                _ => {
                    Err("Invalid type - operands of And should be bool type")?;
                    unreachable!()
                }
            }
        }
        (BinaryOp::And, _) => {
            Err("Invalid type - operand of And should be bool type")?;
            unreachable!()
        }
        (BinaryOp::Or, Value::Bool(true)) => Value::Bool(true),
        (BinaryOp::Or, Value::Bool(false)) => {
            match mapto!(Binary(_, _, ref t) => t)? {
                Value::Bool(v) => Value::Bool(v),
                _ => {
                    Err("Invalid type - operands of Or should be bool type")?;
                    unreachable!()
                }
            }
        }
        (BinaryOp::Or, _) => {
            Err("Invalid type - operands of Or should be bool type")?;
            unreachable!()
        }
        _ => unreachable!()
    })
}

fn eval_binary(op: BinaryOp, left: Value, right: Value) -> Result<Value> {
    use self::BinaryOp::*;
    use self::Value::*;

    Ok(match (op, left, right) {
        (Add, Num(l),  Num(r) ) => Num(l + r),
        (Sub, Num(l),  Num(r) ) => Num(l - r),
        (Mul, Num(l),  Num(r) ) => Num(l * r),
        (Div, Num(l),  Num(r) ) => Num(l / r),

        (Add, Str(l),  Str(r) ) => Str(format!("{}{}", l, r).into()),

        (Eq,  Unit,    Unit   ) => Bool(true),
        (Eq,  Num(l),  Num(r) ) => Bool(l == r),
        (Eq,  Bool(l), Bool(r)) => Bool(l == r),
        (Neq, Unit,    Unit   ) => Bool(false),
        (Neq, Num(l),  Num(r) ) => Bool(l != r),
        (Neq, Bool(l), Bool(r)) => Bool(l != r),

        (Gt,  Num(l),  Num(r) ) => Bool(l > r),
        (Gte, Num(l),  Num(r) ) => Bool(l >= r),
        (Lt,  Num(l),  Num(r) ) => Bool(l < r),
        (Lte, Num(l),  Num(r) ) => Bool(l <= r),

        // these will handled in eval_short_circuit function above
        (And, _, _) => unreachable!(),
        (Or, _, _) => unreachable!(),

        (Add, l, r) => {
            Err(format!("Invalid type - operands of Add op should be both num \
            or both str, but found {:?} and {:?}", l, r))?;
            unreachable!()
        }

        (Sub, l, r) | (Mul, l, r) | (Div, l, r) => {
            Err(format!("Invalid type - operands of arithmetic op should be \
            num type, but found {:?} and {:?}", l, r))?;
            unreachable!()
        }
        (Eq, Func(_, _), Func(_, _)) | (Neq, Func(_, _), Func(_, _)) |
        (Eq, Native(_), Native(_)) | (Neq, Native(_), Native(_)) => {
            Err(format!("Invalid type - function types cannot be compared"))?;
            unreachable!()
        }
        (Eq, l, r) | (Neq, l, r) => {
            Err(format!("Invalid type - both operands of equality op should \
            be equal type, but found {:?} and {:?}", l, r))?;
            unreachable!()
        }
        (Gt, l, r) | (Gte, l, r) | (Lt, l, r) | (Lte, l, r) => {
            Err(format!("Invalid type - operands of comparison op should be \
            num type, but found {:?} and {:?}", l, r))?;
            unreachable!()
        }
    })
}

fn eval_unary(op: UnaryOp, expr: Value) -> Result<Value> {
    use self::UnaryOp::*;
    use self::Value::*;

    Ok(match (op, expr) {
        (Neg, Num(v) ) => Num(-v),
        (Not, Bool(v)) => Bool(!v),

        (Neg, v) => {
            Err(format!("Invalid type - operand of Neg op should be num \
            type, but found {:?}", v))?;
            unreachable!()
        }
        (Not, v) => {
            Err(format!("Invalid type - operand of Not op should be bool \
            type, but found {:?}", v))?;
            unreachable!()
        }
    })
}
