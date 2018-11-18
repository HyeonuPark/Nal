
use std::collections::HashSet;
use std::sync::Arc;
use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::fmt;
use std::slice::Iter;
// use std::iter::{}

/// Interned symbols.
/// Two `Symbol`s are equal only when they're made from same `Context`
/// AND contains same name.
#[derive(Debug, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    name: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolSeq {
    names: Arc<[Symbol]>,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    store: HashSet<Arc<str>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self, name: &str) -> Symbol {
        if let Some(name) = self.store.get(name).cloned() {
            return Symbol { name }
        }

        let symbol = Symbol { name: Arc::from(name) };
        self.store.insert(symbol.name.clone());

        symbol
    }
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        &*self.name
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for Symbol {
    fn eq(&self, rhs: &Self) -> bool {
        Arc::ptr_eq(&self.name, &rhs.name)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl SymbolSeq {
    pub fn as_slice(&self) -> &[Symbol] {
        &*self.names
    }
}

impl AsRef<[Symbol]> for SymbolSeq {
    fn as_ref(&self) -> &[Symbol] {
        self.as_slice()
    }
}

impl Borrow<[Symbol]> for SymbolSeq {
    fn borrow(&self) -> &[Symbol] {
        self.as_slice()
    }
}

impl<'a> IntoIterator for &'a SymbolSeq {
    type Item = &'a Symbol;
    type IntoIter = Iter<'a, Symbol>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}
