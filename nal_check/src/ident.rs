use std::collections::HashSet;

use nal_ast::ast::common::{Ast, Ident};

use common::{Check, Scope, Acc};

macro_rules! hash_set {
    ($($elem:expr),*) => ({
        let mut hset = HashSet::new();
        $(hset.insert($elem);)*
        hset
    });
}

lazy_static! {
    static ref RESERVED: HashSet<&'static str> = hash_set!(
        "if", "while", "for", "in", "let"
    );
}

#[derive(Debug)]
pub enum Error {
    ConflictWithKeyword,
}

impl Check for Ast<Ident> {
    fn check(&self, _scope: Scope, acc: Acc) {
        if RESERVED.contains(&***self) {
            acc.push((Error::ConflictWithKeyword.into(), self.span));
        }
    }
}
