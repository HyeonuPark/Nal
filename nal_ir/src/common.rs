use internship::InternStr;

use func::Function;

pub type Ty = ();

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident(pub InternStr);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VarName {
    pub name: Ident,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    Unit,
    Bool(bool),
    Num(f64),
    Str(String),
    Func(Function),
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
