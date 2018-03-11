extern crate internship;

extern crate nal_ir as ir;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use ir::{Ident, Slot};

pub mod stdnal;

pub mod value;
use self::value::{Value, ValueRef};

pub type RuntimeError = String;

pub fn exec(_src: &ir::EntryModule) -> Result<(), RuntimeError> {
    unimplemented!()
}

#[derive(Debug, Default)]
pub struct Engine {
    globals: HashMap<Ident, Value>,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<V: Into<Value>>(&mut self, name: &str, value: V) -> &mut Self {
        self.globals.insert(Ident(name.into()), value.into());
        self
    }

    pub fn exec(&mut self, src: &ir::EntryModule) -> Result<(), RuntimeError> {
        // check if uncovered free variable exist
        let mut free_vars = src.module.free_vars.clone();

        for name in self.globals.keys() {
            free_vars.remove(name);
        }

        if !free_vars.is_empty() {
            Err(format!("Require global variables: {:?}", free_vars))?;
        }

        // setup execution environment
        let constants: HashMap<_, _> = src.module.constants.iter()
            .map(|(token, con)| {
                use ir::Constant as C;
                use value::Value;

                (*token, match *con {
                    C::Unit => Value::default(),
                    C::Bool(v) => v.into(),
                    C::Num(v) => v.into(),
                    C::Str(ref v) => v.to_string().into(),
                    C::FreeVar(ref ident) => self.globals.remove(ident).unwrap(),
                }.into_ref())
            })
            .collect();
        let functions: HashMap<_, _> = src.module.functions.iter()
            .map(|(token, func)| (*token, func.clone()))
            .collect();

        let root_scope = Scope::new(constants, functions);

        exec_func(&src.main, root_scope, Value::default().into_ref())?;
        Ok(())
    }
}

fn exec_func(func: &ir::Function, scope: Scope, arg: ValueRef) -> Result<ValueRef, RuntimeError> {
    use ir::Opcode as O;
    use ir::ExitCode as X;

    let local = &mut HashMap::new();
    let mut block_token = func.entry;
    let mut parameter = arg;

    loop {
        let block = &func.blocks[&block_token];

        local.clear();
        local.insert(
            block.param,
            ::std::mem::replace(&mut parameter, Value::default().into_ref())
        );

        for code in &block.body {
            match *code {
                O::Variable(ref v) => exec_variable(v, scope.clone(), local)?,
                O::Obj(ref v) => exec_obj(v, scope.clone(), local)?,
                O::Tuple(ref v) => exec_tuple(v, scope.clone(), local)?,
                O::Misc(ref v) => exec_misc(v, scope.clone(), local)?,
            }
        }

        match block.exit {
            X::Jump(ir::Goto { block, argument }) => {
                block_token = block;
                parameter = local.remove(&argument).unwrap();
            }
            X::Branch {
                when,
                then: ir::Goto { block: then_block, argument: then_arg },
                or: ir::Goto { block: or_block, argument: or_arg },
            } => {
                let cond = local.remove(&when).unwrap();
                let cond = match *cond.borrow() {
                    Value::Bool(v) => v,
                    _ => Err("TypeError")?,
                };
                if cond {
                    block_token = then_block;
                    parameter = local.remove(&then_arg).unwrap();
                } else {
                    block_token = or_block;
                    parameter = local.remove(&or_arg).unwrap();
                }
            }
            X::Panic => unreachable!(),
            X::Return(res) => return Ok(local.remove(&res).unwrap()),
        }
    }
}

fn exec_variable(
    code: &ir::Variable,
    scope: Scope,
    local: &mut HashMap<Slot, ValueRef>
) -> Result<(), RuntimeError> {
    use ir::Variable as O;

    match *code {
        O::Declare(ref name, _) => {
            scope.declare(name.clone(), ir::Ty::default());
        }
        O::Get(ref name, slot) => {
            let value = scope.load(name).unwrap();
            local.insert(slot, value);
        }
        O::Set(ref name, slot) => {
            let value = local.get(&slot).unwrap().clone();
            scope.store(name, value);
        }
    }

    Ok(())
}

fn exec_obj(
    code: &ir::Obj,
    scope: Scope,
    local: &mut HashMap<Slot, ValueRef>
) -> Result<(), RuntimeError> {
    use ir::Obj as O;

    match *code {
        O::Open => {
            scope.push(LiteralFrame::Obj(Default::default()));
        }
        O::Push(ref ident, slot) => {
            match *scope.peek() {
                LiteralFrame::Obj(ref mut buf) => {
                    let value = local.get(&slot).unwrap().clone();
                    buf.insert(ident.clone(), value);
                }
                LiteralFrame::Tuple(_) => {
                    Err("Obj::Push within tuple literal")?
                }
            }
        }
        O::Close(slot) => {
            match scope.pop() {
                LiteralFrame::Obj(buf) => {
                    local.insert(slot, Value::Obj(buf).into_ref());
                }
                LiteralFrame::Tuple(_) => {
                    Err("Obj::Close within tuple literal")?
                }
            }
        }
        O::Get { parent, ref name, result } => {
            let parent = local.get(&parent).unwrap().clone();

            let value = match *parent.borrow() {
                Value::Obj(ref table) => {
                    match table.get(name) {
                        None => Err("Obj::Get field not exist")?,
                        Some(v) => v.clone(),
                    }
                }
                // TODO: add primitive method forwarding logic
                _ => Err("Obj::Get on non-obj value")?,
            };

            local.insert(result, value);
        }
        O::Set { parent, ref name, value } => {
            let parent = local.get(&parent).unwrap().clone();
            let value = local.get(&value).unwrap().clone();

            let mut parent = parent.borrow_mut();
            match *parent {
                Value::Obj(ref mut table) => {
                    table.insert(name.clone(), value);
                }
                _ => Err("Obj::Set on non-obj value")?,
            }
        }
    }

    Ok(())
}

fn exec_tuple(
    code: &ir::Tuple,
    scope: Scope,
    local: &mut HashMap<Slot, ValueRef>
) -> Result<(), RuntimeError> {
    use ir::Tuple as O;

    match *code {
        O::Open => {
            scope.push(LiteralFrame::Tuple(Default::default()));
        }
        O::Push(slot) => {
            match *scope.peek() {
                LiteralFrame::Tuple(ref mut buf) => {
                    let value = local.get(&slot).unwrap().clone();
                    buf.push(value);
                }
                LiteralFrame::Obj(_) => {
                    Err("Tuple::Push within obj literal")?
                }
            }
        }
        O::Close(slot) => {
            match scope.pop() {
                LiteralFrame::Tuple(buf) => {
                    local.insert(slot, Value::Tuple(buf).into_ref());
                }
                LiteralFrame::Obj(_) => {
                    Err("Tuple::Close within obj literal")?
                }
            }
        }
        O::Get { parent, index, result } => {
            let parent = local.get(&parent).unwrap().clone();

            let value = match *parent.borrow() {
                Value::Tuple(ref tuple) => {
                    match tuple.get(index) {
                        None => Err("Tuple::Get index out of bound")?,
                        Some(v) => v.clone(),
                    }
                }
                // TODO: add primitive method forwarding logic
                _ => Err("Tuple::Get on non-tuple value")?,
            };

            local.insert(result, value);
        }
        O::Set { parent, index, value } => {
            let parent = local.get(&parent).unwrap().clone();
            let value = local.get(&value).unwrap().clone();

            let mut parent = parent.borrow_mut();
            match *parent {
                Value::Tuple(ref mut tuple) => {
                    match tuple.get_mut(index) {
                        None => Err("Tuple::Set index out of bound")?,
                        Some(v) => {
                            *v = value;
                        }
                    }
                }
                _ => Err("Tuple::Set on non-tuple value")?,
            }
        }
    }

    Ok(())
}

fn exec_misc(
    code: &ir::Misc,
    scope: Scope,
    local: &mut HashMap<Slot, ValueRef>
) -> Result<(), RuntimeError> {
    use ir::Misc as O;

    match *code {
        O::Call { callee, argument, result } => {
            let callee = local.get(&callee).unwrap().clone();
            let argument = local.get(&argument).unwrap().clone();

            let mut callee = callee.borrow_mut();
            let value = match *callee {
                Value::Func(ref mut f) => f(argument)?,
                _ => Err("Misc::Call on non-function value")?,
            };

            local.insert(result, value);
        }
        O::LogicNot { operand, result } => {
            let operand = local.get(&operand).unwrap().clone();
            let operand = operand.borrow();

            let value = match *operand {
                Value::Bool(v) => Value::Bool(!v).into_ref(),
                _ => Err("Misc::LogicNot on non-bool value")?,
            };

            local.insert(result, value);
        }
        O::Closure { ref name, function, result } => {
            let scope = scope.child();
            let scope_copy = scope.clone();
            let func = scope.func(&function).unwrap();
            let value = Value::Func(Box::new(move|arg| {
                exec_func(&func, scope_copy.clone(), arg)
            })).into_ref();

            if let &Some(ref name) = name {
                scope.declare(name.clone(), ir::Ty::default());
                scope.store(name, value.clone());
            }

            local.insert(result, value);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct ScopeBox {
    stack: RefCell<Vec<LiteralFrame>>,
    local: RefCell<HashMap<ir::VarName, ValueRef>>,
    parent: Option<Scope>,
    constants: Rc<HashMap<ir::ConstToken, ValueRef>>,
    functions: Rc<HashMap<ir::FuncToken, Rc<ir::Function>>>,
}

#[derive(Debug)]
pub enum LiteralFrame {
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

    pub fn push(&self, frame: LiteralFrame) {
        self.0.stack.borrow_mut().push(frame);
    }

    pub fn peek(&self) -> RefMut<LiteralFrame> {
        RefMut::map(self.0.stack.borrow_mut(), |stack| {
            stack.last_mut().expect("Scope::peek on empty stack")
        })
    }

    pub fn pop(&self) -> LiteralFrame {
        self.0.stack.borrow_mut().pop()
            .expect("Scope::pop on empty stack")
    }

    pub fn func(&self, token: &ir::FuncToken) -> Option<Rc<ir::Function>> {
        self.0.functions.get(token).cloned()
    }
}
