use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::mem::{ManuallyDrop, transmute, replace, uninitialized, forget};

use owning_ref::{RcRef, BoxRef, RefRef, BoxRefMut, RefMutRefMut};

use common::prelude::*;

#[derive(Debug)]
pub struct ValueRef(ValueRefVar);

#[derive(Debug)]
enum ValueRefVar {
    Own(Value),
    OwnRef(BoxRef<Value>),
    Imm(RcRef<Value>),
    Mut(BorrowRef),
}

use self::ValueRefVar as VR;

impl ValueRef {
    pub fn map<F>(self, f: F) -> Self
        where F: FnOnce(&Value) -> &Value {
            ValueRef(match self.0 {
                VR::Own(v) => {
                    let bref = BoxRef::new(v.into());
                    VR::OwnRef(bref.map(f))
                }
                VR::OwnRef(v) => VR::OwnRef(v.map(f)),
                VR::Imm(v) => VR::Imm(v.map(f)),
                VR::Mut(mut bref) => unsafe {
                    let vref = replace(&mut bref.1, uninitialized());
                    let vref = ManuallyDrop::into_inner(vref).map(f);
                    forget(replace(&mut bref.1, ManuallyDrop::new(vref)));
                    VR::Mut(bref)
                }
            })
        }

    pub fn try_map<F>(self, f: F) -> Result<Self>
        where F: FnOnce(&Value) -> Result<&Value> {
            Ok(ValueRef(match self.0 {
                VR::Own(v) => {
                    let bref = BoxRef::new(v.into());
                    VR::OwnRef(bref.try_map(f)?)
                }
                VR::OwnRef(v) => VR::OwnRef(v.try_map(f)?),
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
            VR::OwnRef(ref v) => &*v,
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

impl From<Rc<RefCell<Value>>> for ValueRef {
    fn from(value: Rc<RefCell<Value>>) -> Self {
        let borrowed = unsafe {
            transmute::<Ref<Value>, Ref<'static, Value>>(
                value.borrow())
        };
        ValueRef(VR::Mut(BorrowRef(
            ManuallyDrop::new(value.clone()),
            ManuallyDrop::new(borrowed.into()),
        )))
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

#[derive(Debug)]
pub struct ValueRefMut(ValueRefMutVar);

#[derive(Debug)]
enum ValueRefMutVar {
    Own(Value),
    OwnRef(BoxRefMut<Value>),
    Mut(BorrowMut),
}

use self::ValueRefMutVar as VRM;

impl ValueRefMut {
    pub fn map<F>(self, f: F) -> Self
        where F: FnOnce(&mut Value) -> &mut Value {
            ValueRefMut(match self.0 {
                VRM::Own(v) => {
                    let bref = BoxRefMut::new(v.into());
                    VRM::OwnRef(bref.map_mut(f))
                }
                VRM::OwnRef(v) => VRM::OwnRef(v.map_mut(f)),
                VRM::Mut(mut bmut) => unsafe {
                    let vref = replace(&mut bmut.1, uninitialized());
                    let vref = ManuallyDrop::into_inner(vref).map_mut(f);
                    forget(replace(&mut bmut.1, ManuallyDrop::new(vref)));
                    VRM::Mut(bmut)
                }
            })
        }

        pub fn try_map<F>(self, f: F) -> Result<Self>
            where F: FnOnce(&mut Value) -> Result<&mut Value> {
                Ok(ValueRefMut(match self.0 {
                    VRM::Own(v) => {
                        let bref = BoxRefMut::new(v.into());
                        VRM::OwnRef(bref.try_map_mut(f)?)
                    }
                    VRM::OwnRef(v) => VRM::OwnRef(v.try_map_mut(f)?),
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
            VRM::OwnRef(ref v) => &*v,
            VRM::Mut(BorrowMut(_, ref v)) => &***v,
        }
    }
}

impl AsMut<Value> for ValueRefMut {
    fn as_mut(&mut self) -> &mut Value {
        match self.0 {
            VRM::Own(ref mut v) => v,
            VRM::OwnRef(ref mut v) => &mut *v,
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

impl From<Rc<RefCell<Value>>> for ValueRefMut {
    fn from(value: Rc<RefCell<Value>>) -> Self {
        let borrowed = unsafe {
            transmute::<RefMut<Value>, RefMut<'static, Value>>(
                value.borrow_mut())
        };
        ValueRefMut(VRM::Mut(BorrowMut(
            ManuallyDrop::new(value.clone()),
            ManuallyDrop::new(borrowed.into()),
        )))
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
