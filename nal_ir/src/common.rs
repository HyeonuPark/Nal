use std::fmt;

use internship::InternStr;

pub type Ty = ();

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident(pub InternStr);

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

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
    FreeVar(Ident),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ConstToken(usize);

impl ConstToken {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        ConstToken(id)
    }

    pub fn to_value(self) -> Slot {
        Slot::Constant(self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct FuncToken(usize);

impl FuncToken {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        FuncToken(id)
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
pub enum Slot {
    Constant(ConstToken),
    Local {
        id: usize,
        ty: Ty,
    }
}

impl Slot {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        Slot::Local { id, ty: Ty::default() }
    }
}
