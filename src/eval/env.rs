use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use eval::{Value, Result, Control};

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
  Value(Value),
  ValueMut(Rc<RefCell<Value>>),
}

#[derive(Debug, Default, Clone)]
pub struct Env<'a>(HashMap<&'a str, Element>);

impl<'a> Env<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn declare(&mut self, id: &'a str, value: Value) {
    self.0.insert(id, Element::Value(value));
  }

  pub fn declare_mut(&mut self, id: &'a str, value: Value) {
    self.0.insert(id, Element::ValueMut(Rc::new(value.into())));
  }

  pub fn get(&self, id: &'a str) -> Result<Value> {
    match self.0.get(id) {
      None => Err(Control::RuntimeError("UndeclaredVariableError".into())),
      Some(elem) => match *elem {
        Element::Value(ref value) => Ok(value.clone()),
        Element::ValueMut(ref value) => Ok(value.borrow().clone()),
      }
    }
  }

  pub fn set(&mut self, id: &'a str, value: Value) -> Result<()> {
    match self.0.get(id) {
      None => Err(Control::RuntimeError("UndeclaredVariableError".into())),
      Some(elem) => match *elem {
        Element::Value(_) => Err(Control::RuntimeError("MutabilityError".into())),
        Element::ValueMut(ref variable) => {
          *variable.borrow_mut() = value;
          Ok(())
        }
      }
    }
  }
}
