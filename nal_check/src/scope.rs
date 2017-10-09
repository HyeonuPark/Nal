use std::collections::HashMap;

use nal_ast::ast::common::Span;

#[derive(Debug)]
pub enum Error {
    NotExist,
    NotMutable,
}

#[derive(Debug, Clone, Default)]
pub struct Scope(HashMap<String, DeclInfo>);

impl Scope {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, name: &str, info: DeclInfo) {
        self.0.insert(name.into(), info);
    }

    pub fn exist<E: From<Error>>(&self, name: &str) -> Result<(), E> {
        if self.0.contains_key(name) {
            Ok(())
        } else {
            Err(Error::NotExist.into())
        }
    }

    pub fn exist_mut<E: From<Error>>(&self, name: &str) -> Result<(), E> {
        match self.0.get(name) {
            Some(info) if info.is_mut => Ok(()),
            Some(_) => Err(Error::NotMutable.into()),
            None => Err(Error::NotExist.into()),
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
        DeclInfo {
            span,
            is_mut: false,
        }
    }

    pub fn set_mut(mut self, is_mut: bool) -> Self {
        self.is_mut = is_mut;
        self
    }
}
