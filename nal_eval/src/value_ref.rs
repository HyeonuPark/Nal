use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::mem::{ManuallyDrop, transmute, replace, uninitialized, forget};

use owning_ref::{RcRef, BoxRef, RefRef, BoxRefMut, RefMutRefMut};

use common::prelude::*;

/// Abstracts over owned or borrowed values
///
/// During evaluation, value consumed by program is either freshly created or
/// borrowed from variable.
/// Also, sometimes we need only part of existing value like a property of
/// object stored in variable.
/// ValueRef just provide this abstraction to avoid unnecessary copy.
///
/// Like using RefCell, ValueRef/Mut follows Rust's borrow rules at runtime.
/// You can't use ValueRef and ValueRefMut from same variable at the same time.
/// Also, only single ValueRefMut can exist for single variable at the same time.
#[derive(Debug)]
pub struct ValueRef(ValueRefVar);

#[derive(Debug)]
enum ValueRefVar {
    Own(Value),
    Part(BoxRef<Value>),
    Imm(RcRef<Value>),
    Mut(BorrowRef),
}

use self::ValueRefVar as VR;

impl ValueRef {
    pub fn map<F>(self, f: F) -> Self
        where F: FnOnce(&Value) -> &Value {
            self.try_map(|arg| Ok(f(arg))).unwrap()
        }

    pub fn try_map<F>(self, f: F) -> Result<Self>
        where F: FnOnce(&Value) -> Result<&Value> {
            Ok(ValueRef(match self.0 {
                VR::Own(v) => {
                    let bref = BoxRef::new(v.into());
                    VR::Part(bref.try_map(f)?)
                }
                VR::Part(v) => VR::Part(v.try_map(f)?),
                VR::Imm(v) => VR::Imm(v.try_map(f)?),
                VR::Mut(mut bref) => unsafe {
                    let vref = replace(&mut bref.1, uninitialized());
                    let vref = ManuallyDrop::into_inner(vref).try_map(f)?;
                    forget(replace(&mut bref.1, ManuallyDrop::new(vref)));
                    VR::Mut(bref)
                }
            }))
        }
}

impl AsRef<Value> for ValueRef {
    fn as_ref(&self) -> &Value {
        match self.0 {
            VR::Own(ref v) => v,
            VR::Part(ref v) => &*v,
            VR::Imm(ref v) => &**v,
            VR::Mut(BorrowRef(_, ref v)) => &***v
        }
    }
}

impl ::std::ops::Deref for ValueRef {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<Value> for ValueRef {
    fn from(value: Value) -> Self {
        ValueRef(VR::Own(value))
    }
}

impl From<Rc<Value>> for ValueRef {
    fn from(value: Rc<Value>) -> Self {
        ValueRef(VR::Imm(RcRef::new(value)))
    }
}

impl ValueRef {
    pub fn try_from(value: Rc<RefCell<Value>>) -> Result<Self> {
        let borrowed = unsafe {
            transmute::<Ref<Value>, Ref<'static, Value>>(
                value.try_borrow()
                    .map_err(|_| "Can't borrow this variable immutably")?)
        };
        Ok(ValueRef(VR::Mut(BorrowRef(
            ManuallyDrop::new(value.clone()),
            ManuallyDrop::new(borrowed.into()),
        ))))
    }
}

#[derive(Debug)]
struct BorrowRef(ManuallyDrop<Rc<RefCell<Value>>>, ManuallyDrop<RefRef<'static, Value>>);

impl ::std::ops::Drop for BorrowRef {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.1);
            ManuallyDrop::drop(&mut self.0);
        }
    }
}

/// Abstracts over owned or mutably borrowed values
///
/// During evaluation, we need to mutate some value, either fresh created or
/// borrowed from variable.
/// Also, sometimes we need only part of existing value like a property of
/// object stored in variable.
/// ValueRefMut provide this abstraction like ValueRef.
/// Obviously, you can't create ValueRefMut from immutable variable.
///
/// By using RefCell, ValueRef/Mut follows Rust's borrow rules. You can't use
/// ValueRef and ValueRefMut from same variable at the same time. Also,
/// only single ValueRefMut can exist for single variable at the same time.
#[derive(Debug)]
pub struct ValueRefMut(ValueRefMutVar);

#[derive(Debug)]
enum ValueRefMutVar {
    Own(Value),
    Part(BoxRefMut<Value>),
    Mut(BorrowMut),
}

use self::ValueRefMutVar as VRM;

impl ValueRefMut {
    pub fn map<F>(self, f: F) -> Self
        where F: FnOnce(&mut Value) -> &mut Value {
            self.try_map(|arg| Ok(f(arg))).unwrap()
        }

        pub fn try_map<F>(self, f: F) -> Result<Self>
            where F: FnOnce(&mut Value) -> Result<&mut Value> {
                Ok(ValueRefMut(match self.0 {
                    VRM::Own(v) => {
                        let bref = BoxRefMut::new(v.into());
                        VRM::Part(bref.try_map_mut(f)?)
                    }
                    VRM::Part(v) => VRM::Part(v.try_map_mut(f)?),
                    VRM::Mut(mut bmut) => unsafe {
                        let vref = replace(&mut bmut.1, uninitialized());
                        let vref = ManuallyDrop::into_inner(vref).try_map_mut(f)?;
                        forget(replace(&mut bmut.1, ManuallyDrop::new(vref)));
                        VRM::Mut(bmut)
                    }
                }))
            }
}

impl AsRef<Value> for ValueRefMut {
    fn as_ref(&self) -> &Value {
        match self.0 {
            VRM::Own(ref v) => v,
            VRM::Part(ref v) => &*v,
            VRM::Mut(BorrowMut(_, ref v)) => &***v,
        }
    }
}

impl AsMut<Value> for ValueRefMut {
    fn as_mut(&mut self) -> &mut Value {
        match self.0 {
            VRM::Own(ref mut v) => v,
            VRM::Part(ref mut v) => &mut *v,
            VRM::Mut(BorrowMut(_, ref mut v)) => &mut ***v,
        }
    }
}

impl ::std::ops::Deref for ValueRefMut {
    type Target = Value;

    fn deref(&self) -> &Value {
        self.as_ref()
    }
}

impl ::std::ops::DerefMut for ValueRefMut {
    fn deref_mut(&mut self) -> &mut Value {
        self.as_mut()
    }
}

impl From<Value> for ValueRefMut {
    fn from(value: Value) -> Self {
        ValueRefMut(VRM::Own(value))
    }
}

impl ValueRefMut {
    pub fn try_from(value: Rc<RefCell<Value>>) -> Result<Self> {
        let borrowed = unsafe {
            transmute::<RefMut<Value>, RefMut<'static, Value>>(
                value.try_borrow_mut()
                    .map_err(|_| "Can't borrow this variable mutably")?)
        };
        Ok(ValueRefMut(VRM::Mut(BorrowMut(
            ManuallyDrop::new(value.clone()),
            ManuallyDrop::new(borrowed.into()),
        ))))
    }
}

#[derive(Debug)]
struct BorrowMut(ManuallyDrop<Rc<RefCell<Value>>>, ManuallyDrop<RefMutRefMut<'static, Value>>);

impl ::std::ops::Drop for BorrowMut {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.1);
            ManuallyDrop::drop(&mut self.0);
        }
    }
}
