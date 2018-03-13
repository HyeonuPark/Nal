use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use ir::{self, Ident};

use error::Result;
use value::{Value, ValueRef};

#[derive(Debug)]
struct ScopeBox {
    stack: RefCell<Vec<LiteralFrame>>,
    local: RefCell<HashMap<ir::VarName, ValueRef>>,
    parent: Option<Scope>,
    constants: Rc<HashMap<ir::ConstToken, ValueRef>>,
    functions: Rc<HashMap<ir::FuncToken, Rc<ir::Function>>>,
}

#[derive(Debug)]
enum LiteralFrame {
    Obj(HashMap<Ident, ValueRef>),
    Tuple(Vec<ValueRef>),
}

#[derive(Debug, Clone)]
pub struct Scope(Rc<ScopeBox>);

impl Scope {
    pub fn new(
        constants: HashMap<ir::ConstToken, ValueRef>,
        functions: HashMap<ir::FuncToken, Rc<ir::Function>>,
    ) -> Self {
        Scope(Rc::new(ScopeBox {
            stack: Default::default(),
            local: Default::default(),
            parent: None,
            constants: Rc::new(constants),
            functions: Rc::new(functions),
        }))
    }

    pub fn child(&self) -> Self {
        Scope(Rc::new(ScopeBox {
            stack: Default::default(),
            local: Default::default(),
            parent: Some(self.clone()),
            constants: self.0.constants.clone(),
            functions: self.0.functions.clone(),
        }))
    }

    pub fn declare(&self, name: ir::VarName, _: ir::Ty) {
        let mut local = self.0.local.borrow_mut();
        local.insert(name, Value::default().into_ref());
    }

    pub fn load(&self, name: &ir::VarName) -> Option<ValueRef> {
        let local = self.0.local.borrow();
        local.get(name).cloned().or_else(|| {
            self.0.parent.as_ref().unwrap().load(name)
        })
    }

    pub fn store(&self, name: &ir::VarName, value: ValueRef) {
        let mut local = self.0.local.borrow_mut();
        *(local.get_mut(name).unwrap()) = value;
    }

    pub fn push_obj(&self) {
        self.0.stack.borrow_mut().push(LiteralFrame::Obj(Default::default()));
    }

    pub fn push_tuple(&self) {
        self.0.stack.borrow_mut().push(LiteralFrame::Tuple(Default::default()));
    }

    pub fn peek_obj(&self) -> Result<RefMut<HashMap<Ident, ValueRef>>> {
        match self.0.stack.borrow().last() {
            None => Err("Scope::peek_obj on empty stack")?,
            Some(&LiteralFrame::Tuple(_)) => {
                Err("Scope::peek_obj within tuple literal")?
            },
            Some(&LiteralFrame::Obj(_)) => {}
        }
        Ok(RefMut::map(self.0.stack.borrow_mut(), |stack| {
            match stack.last_mut() {
                None => unreachable!(),
                Some(&mut LiteralFrame::Tuple(_)) => unreachable!(),
                Some(&mut LiteralFrame::Obj(ref mut map)) => map,
            }
        }))
    }

    pub fn peek_tuple(&self) -> Result<RefMut<Vec<ValueRef>>> {
        match self.0.stack.borrow().last() {
            None => Err("Scope::peek_tuple on empty stack")?,
            Some(&LiteralFrame::Obj(_)) => {
                Err("Scope::peek_tuple within obj literal")?
            },
            Some(&LiteralFrame::Tuple(_)) => {}
        }
        Ok(RefMut::map(self.0.stack.borrow_mut(), |stack| {
            match stack.last_mut() {
                None => unreachable!(),
                Some(&mut LiteralFrame::Obj(_)) => unreachable!(),
                Some(&mut LiteralFrame::Tuple(ref mut tup)) => tup,
            }
        }))
    }

    pub fn pop_obj(&self) -> Result<HashMap<Ident, ValueRef>> {
        match self.0.stack.borrow_mut().pop() {
            None => Err("Scope::pop_obj on empty stack")?,
            Some(LiteralFrame::Tuple(_)) => Err("Scope::pop_obj within tuple literal")?,
            Some(LiteralFrame::Obj(map)) => Ok(map),
        }
    }

    pub fn pop_tuple(&self) -> Result<Vec<ValueRef>> {
        match self.0.stack.borrow_mut().pop() {
            None => Err("Scope::pop_tuple on empty stack")?,
            Some(LiteralFrame::Obj(_)) => Err("Scope::pop_tuple within obj literal")?,
            Some(LiteralFrame::Tuple(tup)) => Ok(tup),
        }
    }

    pub fn func(&self, token: &ir::FuncToken) -> Option<Rc<ir::Function>> {
        self.0.functions.get(token).cloned()
    }
}
