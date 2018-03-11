use std::collections::HashMap;

use internship::InternStr;

use {ast, ir, Result as Res, ConvertError};

#[derive(Debug, Default)]
pub struct Scope {
    counter: HashMap<InternStr, usize>,
    frame: Frame,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child<T, F: FnOnce(&mut Self) -> Res<T>>(&mut self, f: F) -> Res<T> {
        self.frame.push();
        let res = f(self);
        self.frame.pop();
        res
    }

    pub fn declare(&mut self, name: &ast::Ident) -> ir::VarName {
        let name = &name.0;

        let count = {
            let place = self.counter.entry(name.clone()).or_insert(0);
            let count = *place;
            *place += 1;
            count
        };
        self.frame.declare(name.clone(), count);

        ir::VarName {
            name: ir::Ident(name.clone()),
            count,
        }
    }

    pub fn get(&self, name: &ast::Ident) -> Result<ir::VarName, ConvertError> {
        let name = &name.0;

        self.frame.get(name).map(|count| ir::VarName {
            name: ir::Ident(name.clone()),
            count,
        }).ok_or(ConvertError)
    }
}

#[derive(Debug, Default)]
struct Frame {
    local: HashMap<InternStr, usize>,
    parent: Option<Box<Frame>>,
}

impl Frame {
    fn declare(&mut self, name: InternStr, count: usize) {
        let place = self.local.entry(name).or_insert(0);
        *place = count;
    }

    fn get(&self, name: &InternStr) -> Option<usize> {
        match self.local.get(name) {
            Some(count) => Some(*count),
            None => match self.parent {
                Some(ref parent) => parent.get(name),
                None => None,
            },
        }
    }

    fn push(&mut self) {
        use std::mem::replace;

        let parent = replace(self, Default::default());
        replace(&mut self.parent, Some(parent.into()));
    }

    fn pop(&mut self) {
        use std::mem::replace;

        let parent = *replace(&mut self.parent, None).unwrap();
        replace(self, parent);
    }
}
