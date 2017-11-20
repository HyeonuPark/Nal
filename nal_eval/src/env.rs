use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

use owning_ref::RcRef;

use nal_ast::SourceBuffer;

use common::prelude::*;

/// Provide function's call stack
#[derive(Debug)]
pub struct Env<'a> {
    program: RcRef<SourceBuffer>,
    map: HashMap<Rc<str>, Bucket>,
    parent: Option<&'a Env<'a>>,
}

#[derive(Debug, Clone)]
enum Bucket {
    Imm(Rc<Value>),
    Mut(Rc<RefCell<Value>>),
}

use self::Bucket::*;

impl<'a> Env<'a> {
    pub fn new(program: RcRef<SourceBuffer>) -> Env<'static> {
        Env {
            program,
            map: Default::default(),
            parent: None,
        }
    }

    pub fn names(&self) -> HashSet<String> {
        let mut hset = self.parent.map(|env| env.names()).unwrap_or_default();
        hset.extend(self.map.keys().map(|k| k.to_string()));
        hset
    }

    pub fn get_fn(&self, f: &ast::Function) -> Value {
        use self::ast::Function as F;

        Value::Func(
            self.program.clone().map(|_| unsafe {
                ::std::mem::transmute::<&F, &F>(f)
            }),
            self.deep_clone().into()
        )
    }

    /// Clone this env's whole ancestors and produce new env with static lifetime.
    /// This function's purpose is to support closure scope dynamically.
    /// Note that it's really easy to accidently create circular Rc loop with this
    /// method. It's intended behavior, as it should be prevented by
    /// static analysis that will be introduced later.
    fn deep_clone(&self) -> Env<'static> {

        fn clone_env(from: &Env, to: &mut Env) {
            if let Some(p) = from.parent {
                clone_env(p, to);
            }

            to.map.extend(from.map.iter().map(|(k, v)| (Rc::clone(k), v.clone())));
        }

        let mut env = Env::new(self.program.clone());

        clone_env(self, &mut env);
        env
    }

    pub fn decl(&mut self, name: Rc<str>, value: Value) {
        self.map.insert(name, Imm(value.into()));
    }

    pub fn decl_mut(&mut self, name: Rc<str>, value: Value) {
        self.map.insert(name, Mut(Rc::new(value.into())));
    }

    pub fn get(&self, name: &str) -> Result<ValueRef> {
        match self.map.get(name) {
            Some(&Imm(ref v)) => Ok(Rc::clone(v).into()),
            Some(&Mut(ref v)) => ValueRef::try_from(Rc::clone(v)),
            None => match self.parent {
                Some(p) => p.get(name),
                None => Err(format!("Var {} is not declared", name))?,
            }
        }
    }

    pub fn get_mut(&self, name: &str) -> Result<ValueRefMut> {
        match self.map.get(name) {
            Some(&Imm(_)) => Err(format!("Var {} is not mutable", name))?,
            Some(&Mut(ref v)) => ValueRefMut::try_from(Rc::clone(v)),
            None => match self.parent {
                Some(p) => p.get_mut(name),
                None => Err(format!("Var {} is not declared", name))?,
            }
        }
    }

    pub fn child(&self) -> Env {
        Env {
            program: self.program.clone(),
            map: HashMap::new(),
            parent: Some(self),
        }
    }
}
