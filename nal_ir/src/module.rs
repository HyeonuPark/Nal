use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use common::{Constant, ConstToken, FuncToken, Ident};
use func::Function;

#[derive(Debug)]
pub struct EntryModule {
    pub module: Module,
    pub main: Function,
}

#[derive(Debug)]
pub struct Module {
    pub constants: HashMap<ConstToken, Constant>,
    pub functions: HashMap<FuncToken, Rc<Function>>,
    pub free_vars: HashSet<Ident>,
}

#[derive(Debug)]
pub struct ModuleBuilder {
    count: usize,
    constants: HashMap<ConstToken, Constant>,
    functions: HashMap<FuncToken, Rc<Function>>,
    free_vars: HashSet<Ident>,
    token_unit: ConstToken,
    token_true: ConstToken,
    token_false: ConstToken,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        let mut count = 0;
        let mut constants = HashMap::new();

        let token_unit = ConstToken::new(&mut count);
        let token_true = ConstToken::new(&mut count);
        let token_false = ConstToken::new(&mut count);
        constants.insert(token_unit, Constant::Unit);
        constants.insert(token_true, Constant::Bool(true));
        constants.insert(token_false, Constant::Bool(false));

        ModuleBuilder {
            count,
            constants,
            functions: HashMap::new(),
            free_vars: HashSet::new(),
            token_unit,
            token_true,
            token_false,
        }
    }

    pub fn get_unit(&self) -> ConstToken {
        self.token_unit
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

    pub fn add_free_var(&mut self, value: Ident) -> ConstToken {
        self.free_vars.insert(value.clone());

        self.add_const(Constant::FreeVar(value))
    }

    pub fn add_func(&mut self, value: Function) -> FuncToken {
        let token = FuncToken::new(&mut self.count);
        self.functions.insert(token, value.into());
        token
    }

    pub fn finish(self) -> Module {
        Module {
            constants: self.constants,
            functions: self.functions,
            free_vars: self.free_vars,
        }
    }
}
