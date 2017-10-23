use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use common::{Value, Result};

#[derive(Debug, Clone)]
enum Bucket {
    Imm(Value),
    Mut(Rc<RefCell<Value>>),
}

use self::Bucket::*;

#[derive(Debug, Default)]
pub struct Env<'a> {
    map: HashMap<String, Bucket>,
    parent: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn decl(&mut self, name: &str, value: Value) {
        self.map.insert(name.into(), Imm(value));
    }

    pub fn decl_mut(&mut self, name: &str, value: Value) {
        self.map.insert(name.into(), Mut(Rc::new(value.into())));
    }

    pub fn assign(&self, name: &str, value: Value) -> Result<()> {
        match self.map.get(name) {
            Some(&Imm(_)) => Err(format!("Var {} is not mutable", name))?,
            Some(&Mut(ref v)) => {
                *(v.borrow_mut()) = value;
                Ok(())
            }
            None => match self.parent {
                Some(ref p) => p.assign(name, value),
                None => Err(format!("Var {} not declared", name))?,
            }
        }
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        match self.map.get(name) {
            Some(&Imm(ref v)) => Ok(v.clone()),
            Some(&Mut(ref v)) => Ok(v.borrow().clone()),
            None => match self.parent {
                Some(ref p) => p.get(name),
                None => Err(format!("Var {} not declared", name))?,
            }
        }
    }

    pub fn child(&'a self) -> Self {
        Env {
            map: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn clone(&self) -> Env<'static> {
        let mut env = Env::default();
        clone_env(self, &mut env);
        env
    }
}

fn clone_env(from: &Env, to: &mut Env) {
    if let Some(p) = from.parent {
        clone_env(p, to);
    }

    to.map.extend(from.map.iter().map(|(k, v)| (k.clone(), v.clone())));
}
