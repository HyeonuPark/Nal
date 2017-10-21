use std::collections::HashMap;
use std::mem::replace;

use ast::common::{Ast, Span, Ident};
use super::Error as E;

#[derive(Debug, Default)]
pub struct Ctx {
    scope: Scope,
    error_list: Vec<E>,
    within_fn: bool,
    within_loop: bool,
}

impl Ctx {
    pub fn report(&mut self, err: E) {
        self.error_list.push(err);
    }

    pub fn subscope<F: FnOnce(&mut Self)>(&mut self, sub: F) {
        let prev = replace(&mut self.scope, Scope::default());
        self.scope.parent = Some(prev.into());

        sub(self);

        let scope = self.scope.parent.take().unwrap();
        self.scope = *scope;
    }

    pub fn insert(&mut self, name: &str, decl: DeclInfo) {
        self.scope.map.insert(name.into(), decl);
    }

    pub fn exist(&mut self, ident: &Ast<Ident>) {
        if self.scope.get(ident).is_none() {
            self.report(E::VarNotDecl(ident.span));
        }
    }

    pub fn exist_mut(&mut self, ident: &Ast<Ident>) {
        let err = match self.scope.get(ident) {
            None => E::VarNotDecl(ident.span),
            Some(decl) if !decl.is_mut => E::VarNotMut(ident.span, decl.span),
            _ => return,
        };

        self.report(err);
    }

    pub fn errors(self) -> Vec<E> {
        self.error_list
    }

    pub fn with_fn<F: FnOnce(&mut Self)>(&mut self, sub: F) {
        let prev_fn = self.within_fn;
        let prev_loop = self.within_loop;

        self.within_fn = true;
        self.within_loop = false;

        sub(self);

        self.within_fn = prev_fn;
        self.within_loop = prev_loop;
    }

    pub fn with_loop<F: FnOnce(&mut Self)>(&mut self, sub: F) {
        let prev_loop = self.within_loop;
        self.within_loop = true;

        sub(self);

        self.within_loop = prev_loop;
    }

    pub fn is_fn(&self) -> bool {
        self.within_fn
    }

    pub fn is_loop(&self) -> bool {
        self.within_loop
    }
}

#[derive(Debug, Default)]
struct Scope {
    map: HashMap<String, DeclInfo>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    fn get(&self, name: &str) -> Option<&DeclInfo> {
        match self.map.get(name) {
            Some(decl) => Some(decl),
            None => match self.parent {
                Some(ref p) => p.get(name),
                None => None,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclInfo {
    pub span: Span,
    pub is_mut: bool,
}

impl DeclInfo {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            is_mut: false,
        }
    }

    pub fn set_mut(&self, is_mut: bool) -> Self {
        Self {
            span: self.span,
            is_mut,
        }
    }
}
