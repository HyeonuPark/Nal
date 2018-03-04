use internship::InternStr;

use block::Function;

// TODO: implement type structure
pub type Ty = ();

/// Identifier represents any names in code, includes variables and properties.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident(InternStr);

/// Unique name for every variables.
///
/// In source code, multiple variables with same name can be declared,
/// and only innermost and last visible one is used when referenced.
///
/// In IR structure, each variables have unique `VarName` which has name and counter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VarName(Ident, usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    Bool(bool),
    Num(f64),
    Str(String),
    Func(Function),
    // Import {
    //     from: InternStr,
    //     name: Ident,
    // }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ConstToken(usize);

impl ConstToken {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        ConstToken(id)
    }

    pub fn to_value(self) -> Value {
        Value::Constant(self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct BlockToken(usize);

impl BlockToken {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        BlockToken(id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    Constant(ConstToken),
    Local {
        id: usize,
        ty: Ty,
    }
}

impl Value {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        Value::Local { id, ty: Ty::default() }
    }
}
