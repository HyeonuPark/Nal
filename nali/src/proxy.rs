use std::rc::Rc;
use std::cell::RefCell;

use ir::Ident;

use error::Result;
use value::{Value, ValueRef};
use value::Value::*;

pub fn proxy_bool(value: bool, name: &Ident) -> Result<ValueRef> {
    macro_rules! method {
        ($other:ident => $res:expr) => ({
            Rc::new(RefCell::new(Value::from(move|other:ValueRef| {
                match *other.borrow() {
                    Bool($other) => Ok(Rc::new(RefCell::new($res.into()))),
                    _ => Err("Not a Bool type")?,
                }
            })))
        });
    }

    Ok(match name.as_ref() {
        "equals"     => method!(other => value == other),
        "not_equals" => method!(other => value != other),
        _ => Err(format!("Method {} not exist on Bool type", name.as_ref()))?,
    })
}

pub fn proxy_num(value: f64, name: &Ident) -> Result<ValueRef> {
    macro_rules! method {
        ($other:ident => $res:expr) => ({
            Rc::new(RefCell::new(Value::from(move|other:ValueRef| {
                match *other.borrow() {
                    Num($other) => Ok(Rc::new(RefCell::new($res.into()))),
                    _ => Err("Not a Num type")?,
                }
            })))
        });
    }

    Ok(match name.as_ref() {
        "add"        => method!(other => value + other),
        "subtract"   => method!(other => value - other),
        "multiply"   => method!(other => value * other),
        "divide"     => method!(other => value / other),
        "equals"     => method!(other => (value - other).abs() < ::std::f64::EPSILON),
        "not_equals" => method!(other => (value - other).abs() >= ::std::f64::EPSILON),
        "greater_than"     => method!(other => value > other),
        "not_less_than"    => method!(other => value >= other),
        "less_than"        => method!(other => value < other),
        "not_greater_than" => method!(other => value <= other),
        _ => Err(format!("Method {} not exist on Num type", name.as_ref()))?,
    })
}

pub fn proxy_str(_: &str, value_ref: ValueRef, name: &Ident) -> Result<ValueRef> {
    macro_rules! method {
        ($value:ident, $other:ident => $res:expr) => ({
            Rc::new(RefCell::new(Value::from(move|other:ValueRef| {
                match *other.borrow() {
                    Str(ref $other) => {
                        match *value_ref.borrow() {
                            Str(ref $value) => {
                                Ok(Rc::new(RefCell::new($res.into())))
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => Err("Not a Str type")?,
                }
            })))
        });
    }

    Ok(match name.as_ref() {
        "add"        => method!(value, other => format!("{}{}", value, other)),
        "equals"     => method!(value, other => value == other),
        "not_equals" => method!(value, other => value != other),
        "greater_than"     => method!(value, other => value > other),
        "not_less_than"    => method!(value, other => value >= other),
        "less_than"        => method!(value, other => value < other),
        "not_greater_than" => method!(value, other => value <= other),
        _ => Err(format!("Method {} not exist on Str type", name.as_ref()))?,
    })
}
