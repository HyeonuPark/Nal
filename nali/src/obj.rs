
use std::rc::Weak;
use std::cell::Cell;
use std::ops::{Deref, Drop};
use std::{usize, isize};

use common::{IResult, IError, ObjKind};
use state::Scope;

#[derive(Debug)]
pub struct Obj {
    refcount: Cell<usize>,
    pub kind: ObjKind,
}

impl Obj {
    pub fn new(kind: ObjKind) -> Self {
        Obj {
            refcount: 0.into(),
            kind,
        }
    }
}

#[derive(Debug)]
pub struct Ref {
    value: *mut Obj,
    scope: Weak<Scope>,
}

#[derive(Debug)]
pub struct RefMut {
    oref: Ref,
}

impl Deref for RefMut {
    type Target = Ref;

    fn deref(&self) -> &Ref {
        &self.oref
    }
}

impl Ref {
    pub unsafe fn new(value: *mut Obj, scope: Weak<Scope>) -> IResult<Self> {
        scope.upgrade().ok_or(IError::ScopeEnded)?;

        let count = &(*value).refcount;

        if count.get() == usize::MAX {
            return Err(IError::RefMutBorrowed);
        }

        if count.get() >= isize::MAX as usize {
            return Err(IError::RefOverflow);
        }

        count.set(count.get() + 1);
        return Ok(Ref {
            value,
            scope,
        })
    }

    pub fn get(&self) -> IResult<&Obj> {
        self.scope.upgrade().ok_or(IError::ScopeEnded)?;
        Ok(unsafe { &*self.value })
    }

    pub fn try_map<F>(&self, f: F) -> IResult<Self> where
        F: FnOnce(&Obj) -> IResult<&Obj>,
    {
        let orig = self.get()?;
        let obj = f(orig)?;

        Ok(Ref {
            value: obj as *const Obj as *mut Obj,
            scope: self.scope.clone(),
        })
    }

    pub fn map<F>(&self, f: F) -> IResult<Self> where
        F: FnOnce(&Obj) -> &Obj,
    {
        self.try_map(|oref| Ok(f(oref)))
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        if let Some(_) = self.scope.upgrade() {
            let count = unsafe { &(*self.value).refcount };
            assert!(count.get() < isize::MAX as usize);
            count.set(count.get() - 1);
        }
    }
}

impl Clone for Ref {
    fn clone(&self) -> Self {
        unsafe {
            Ref::new(self.value, self.scope.clone()).unwrap()
        }
    }
}

impl RefMut {
    pub unsafe fn new(value: *mut Obj, scope: Weak<Scope>) -> IResult<Self> {
        scope.upgrade().ok_or(IError::ScopeEnded)?;

        let count = &(*value).refcount;

        if count.get() == usize::MAX {
            return Err(IError::RefMutBorrowed);
        }

        if count.get() != 0 {
            return Err(IError::ObjBorrowed);
        }

        count.set(usize::MAX);
        Ok(RefMut {
            oref: Ref {
                value,
                scope,
            }
        })
    }

    pub fn get_mut(&mut self) -> IResult<&mut Obj> {
        self.oref.scope.upgrade().ok_or(IError::ScopeEnded)?;
        Ok(unsafe { &mut *self.oref.value })
    }

    pub fn try_map_mut<F>(&mut self, f: F) -> IResult<Self> where
        F: FnOnce(&mut Obj) -> IResult<&mut Obj>,
    {
        let scope = self.scope.clone();
        let orig = self.get_mut()?;
        let obj = f(orig)?;

        Ok(RefMut {
            oref: Ref {
                value: obj as *mut Obj,
                scope,
            }
        })
    }

    pub fn map_mut<F>(&mut self, f: F) -> IResult<Self> where
        F: FnOnce(&mut Obj) -> &mut Obj,
    {
        self.try_map_mut(|oref| Ok(f(oref)))
    }
}

impl Drop for RefMut {
    fn drop(&mut self) {
        if let Some(_) = self.scope.upgrade() {
            let count = unsafe { &(*self.value).refcount };
            assert!(count.get() == usize::MAX);
            count.set(0);
        }
    }
}
