use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use internship::InternStr;

use ir::Ident;

use error::Result;

pub type ValueRef = Rc<RefCell<Value>>;

pub enum Value {
    Unit,
    Bool(bool),
    Num(f64),
    Str(String),
    Func(Box<FnMut(ValueRef) -> Result<ValueRef> + 'static>),
    Obj(HashMap<Ident, ValueRef>),
    Tuple(Vec<ValueRef>),
}

impl Value {
    pub fn new<V: Into<Value>>(value: V) -> Self {
        value.into()
    }

    pub fn into_ref(self) -> ValueRef {
        Rc::new(RefCell::new(self))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Unit
    }
}

#[derive(Debug, Default)]
pub struct Obj(HashMap<Ident, ValueRef>);

impl Obj {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<S, V>(&mut self, name: S, value: V) -> &mut Self where
        S: Into<InternStr>, V: Into<ValueRef>
    {
        self.0.insert(Ident(name.into()), value.into());
        self
    }

    pub fn into_value(self) -> Value {
        Value::Obj(self.0)
    }
}

#[derive(Debug, Default)]
pub struct Tuple(Vec<ValueRef>);

impl Tuple {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<V: Into<ValueRef>>(&mut self, value: V) -> &mut Self {
        self.0.push(value.into());
        self
    }

    pub fn into_value(self) -> Value {
        Value::Tuple(self.0)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Bool(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Num(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Value {
        Value::Str(v)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(v: &'a str) -> Value {
        Value::Str(v.into())
    }
}

impl<F: FnMut(ValueRef) -> Result<ValueRef> + 'static> From<F> for Value {
    fn from(v: F) -> Value {
        Value::Func(Box::new(v))
    }
}

impl From<Obj> for Value {
    fn from(v: Obj) -> Value {
        v.into_value()
    }
}

impl From<Tuple> for Value {
    fn from(v: Tuple) -> Value {
        v.into_value()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value as V;

        match *self {
            V::Unit => write!(f, "Unit"),
            V::Bool(v) => write!(f, "Bool({:?})", v),
            V::Num(v) => write!(f, "Num({:?})", v),
            V::Str(ref v) => write!(f, "Str({:?})", v),
            V::Func(_) => write!(f, "Function"),
            V::Obj(ref v) => {
                let mut f = f.debug_struct("Obj");

                for (name, elem) in v {
                    f.field(name.0.as_ref(), elem);
                }

                f.finish()
            }
            V::Tuple(ref v) => {
                let mut f = f.debug_tuple("Tuple");

                for elem in v {
                    f.field(elem);
                }

                f.finish()
            }
        }
    }
}
