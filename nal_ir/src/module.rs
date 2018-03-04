//! Top level structures of Nal code.
//!
//! Each Nal source files are compiled to either `Script` or `Module`.

use std::collections::HashMap;

// use internship::InternStr;

use common::{Constant, ConstToken};
use block::Function;

#[derive(Debug)]
pub struct EntryModule {
    module: Module,
    main: Function,
}

#[derive(Debug)]
pub struct Module {
    constants: HashMap<ConstToken, Constant>,
}

pub struct ModuleBuilder {
    count: usize,
    constants: HashMap<ConstToken, Constant>,
    token_true: ConstToken,
    token_false: ConstToken,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        let mut count = 0;
        let mut constants = HashMap::new();

        let token_true = ConstToken::new(&mut count);
        let token_false = ConstToken::new(&mut count);
        constants.insert(token_true, Constant::Bool(true));
        constants.insert(token_false, Constant::Bool(false));

        ModuleBuilder {
            count,
            constants,
            token_true,
            token_false,
        }
    }

    pub fn get_bool(&self, value: bool) -> ConstToken {
        if value {
            self.token_true
        } else {
            self.token_false
        }
    }

    fn add_const(&mut self, value: Constant) -> ConstToken {
        let token = ConstToken::new(&mut self.count);
        self.constants.insert(token, value);
        token
    }

    pub fn add_num(&mut self, value: f64) -> ConstToken {
        self.add_const(Constant::Num(value))
    }

    pub fn add_str(&mut self, value: String) -> ConstToken {
        self.add_const(Constant::Str(value))
    }

    pub fn add_func(&mut self, value: Function) -> ConstToken {
        self.add_const(Constant::Func(value))
    }

    pub fn finish(self) -> Module {
        Module {
            constants: self.constants,
        }
    }
}
