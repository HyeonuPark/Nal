use std::collections::HashMap;

use common::{Constant, VarName, Ident, ConstToken};
use block::Function;

pub struct Module {
    constants: Vec<Constant>,
    imports: Vec<ImportClause>,
    scope: HashMap<VarName, ConstToken>,
    exports: HashMap<Ident, ConstToken>,
}

pub struct Script {
    constants: Vec<Constant>,
    imports: Vec<ImportClause>,
    main: Function,
}

pub struct ImportClause {
    from: String,
    names: HashMap<Ident, ConstToken>,
}
