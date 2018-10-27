
use std::collections::HashSet;
use std::sync::Arc;
use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::fmt;

/// Interned identifier.
/// Two `Ident`s are equal only when they're made from same `IdentStore`
/// AND contains same name.
#[derive(Debug, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    name: Arc<str>,
}

#[derive(Debug, Clone, Default)]
pub struct IdentStore {
    store: HashSet<Arc<str>>,
}

impl IdentStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self, name: &str) -> Ident {
        if let Some(name) = self.store.get(name).cloned() {
            return Ident { name }
        }

        let ident = Ident { name: Arc::from(name) };
        self.store.insert(ident.name.clone());

        ident
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        &*self.name
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for Ident {
    fn eq(&self, rhs: &Self) -> bool {
        Arc::ptr_eq(&self.name, &rhs.name)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
