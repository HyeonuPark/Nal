use block::{ParamBlock, Function};

// TODO: implement type structure
pub type Ty = ();

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Index(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VarName(Ident, usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    Bool(bool),
    Num(f64),
    Str(String),
    Func(Function),
}

pub trait TokenSlice {
    type Token: Copy;
    type Data;

    fn token_ref(&self, token: Self::Token) -> &Self::Data;
    fn token_mut(&mut self, token: Self::Token) -> &mut Self::Data;
}

macro_rules! tokens {
    ($($token:ident - $data:ident)*) => ($(
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
        pub struct $token(usize);

        impl $token {
            pub fn new(ctx: &mut usize) -> Self {
                let id = *ctx;
                *ctx += 1;
                $token(id)
            }
        }

        impl TokenSlice for [$data] {
            type Token = $token;
            type Data = $data;

            fn token_ref(&self, token: Self::Token) -> &Self::Data {
                &self[token.0]
            }

            fn token_mut(&mut self, token: Self::Token) -> &mut Self::Data {
                &mut self[token.0]
            }
        }
    )*);
}

tokens!{
    ConstToken - Constant
    BlockToken - ParamBlock
    FuncToken - Function
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Value(usize);

impl Value {
    pub fn new(ctx: &mut usize) -> Self {
        let id = *ctx;
        *ctx += 1;
        Value(id)
    }
}
