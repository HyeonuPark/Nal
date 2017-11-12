use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

use common::prelude::*;

#[derive(Debug, Clone)]
enum Bucket {
    Imm(Rc<Value>),
    Mut(Rc<RefCell<Value>>),
}

use self::Bucket::*;

#[derive(Debug, Default)]
pub struct Env<'a> {
    map: HashMap<Rc<str>, Bucket>,
    parent: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn names(&self) -> HashSet<String> {
        let mut hset = self.parent.map(|env| env.names()).unwrap_or_default();
        hset.extend(self.map.keys().map(|k| k.to_string()));
        hset
    }

    pub fn decl(&mut self, name: Rc<str>, value: Value) {
        self.map.insert(name, Imm(value.into()));
    }

    pub fn decl_mut(&mut self, name: Rc<str>, value: Value) {
        self.map.insert(name, Mut(Rc::new(value.into())));
    }

    pub fn get(&self, name: &str) -> Result<ValueRef> {
        match self.map.get(name) {
            Some(&Imm(ref v)) => Ok(v.clone().into()),
            Some(&Mut(ref v)) => Ok(v.clone().into()),
            None => match self.parent {
                Some(ref p) => p.get(name),
                None => Err(format!("Var {} is not declared", name))?,
            }
        }
    }

    pub fn get_mut(&self, name: &str) -> Result<ValueRefMut> {
        match self.map.get(name) {
            Some(&Imm(_)) => Err(format!("Var {} is not mutable", name))?,
            Some(&Mut(ref v)) => Ok(v.clone().into()),
            None => match self.parent {
                Some(ref p) => p.get_mut(name),
                None => Err(format!("Var {} is not declared", name))?,
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

        fn clone_env(from: &Env, to: &mut Env) {
            if let Some(p) = from.parent {
                clone_env(p, to);
            }

            to.map.extend(from.map.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        let mut env = Env::default();
        clone_env(self, &mut env);
        env
    }
}
