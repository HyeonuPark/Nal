
use std::rc::{Rc, Weak};
use std::usize;

use common::{IResult, IError, Ident, Value};
use obj::{Obj, Ref, RefMut};

pub const STACK_SIZE: usize = 1024 * 4;

#[derive(Debug)]
pub struct State {
    scope: Rc<Scope>,
    /// This buffer should never be reallocated.
    vars: Vec<Variable>,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    start_idx: usize,
}

#[derive(Debug)]
pub struct Variable {
    name: Option<Ident>,
    obj: Obj,
}

impl State {
    pub fn new() -> Self {
        State {
            scope: Rc::new(Scope {
                parent: None,
                start_idx: 0,
            }),
            vars: Vec::with_capacity(STACK_SIZE),
            stack: vec![],
        }
    }

    pub fn push_scope(&mut self) {
        let parent = Rc::clone(&self.scope);
        self.scope = Rc::new(Scope {
            parent: Some(parent),
            start_idx: self.vars.len(),
        });
    }

    pub fn pop_scope(&mut self) -> IResult<()> {
        self.vars.truncate(self.scope.start_idx);

        let scope = self.scope.parent.clone().ok_or(IError::PopRootScope)?;
        self.scope = scope;

        Ok(())
    }

    fn push_var(&mut self, name: Option<Ident>, obj: Obj) -> IResult<(usize, Weak<Scope>)> {
        let idx = self.vars.len();
        if idx >= STACK_SIZE {
            return Err(IError::StackOverflow);
        }

        self.vars.push(Variable {
            name,
            obj,
        });

        let scope = Rc::downgrade(&self.scope);

        Ok((idx, scope))
    }

    pub fn decl_var(&mut self, name: Option<Ident>, obj: Obj) -> IResult<Ref> {
        let (idx, scope) = self.push_var(name, obj)?;
        unsafe { Ref::new(&mut self.vars[idx].obj, scope) }
    }

    pub fn decl_mut_var(&mut self, name: Option<Ident>, obj: Obj) -> IResult<RefMut> {
        let (idx, scope) = self.push_var(name, obj)?;
        unsafe { RefMut::new(&mut self.vars[idx].obj, scope) }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}
