//! Top level structures of Nal code.
//!
//! Each Nal source files are compiled to either `Script` or `Module`.

use std::collections::HashMap;

use common::{Constant, VarName, Ident, ConstToken};
use block::Function;

/// `Script` represents executable code.
/// Source file is compiled as `Script` when it is used as an entry point of compilation.
/// It's top level scope is treated as `main` function in other languages.
///
/// `Script` should not have any `export` statements.
pub struct Script {
    constants: Vec<Constant>,
    imports: Vec<ImportClause>,
    main: Function,
}

/// `Module` represents linkable code.
/// Source file is compiled as `Module` when it is `import`ed from other code,
/// which is either `Script` or `Module`.
///
/// `Script` and `Module` has same syntax in source code,
/// but any variables in top level scope of `Module` should be a constant.
///
/// With this limitation, `Module`s don't need to be initialized before imported
/// so cyclic dependency problems become trivial.
pub struct Module {
    constants: Vec<Constant>,
    imports: Vec<ImportClause>,
    scope: HashMap<VarName, ConstToken>,
    exports: HashMap<Ident, ConstToken>,
}

pub struct ImportClause {
    from: String,
    names: HashMap<Ident, ConstToken>,
}
