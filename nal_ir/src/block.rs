use common::{Value, BlockToken, Ty};
use opcode::Opcode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goto(BlockToken, Value);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamBlock {
    param: Value,
    body: Vec<Opcode>,
    exit: ExitCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitCode {
    Return(Value),
    Jump(Goto),
    Branch {
        when: Value,
        then: Goto,
        or: Goto,
    },
    Cast {
        when: Value,
        cast_to: Ty,
        then: Goto,
        or: Goto,
    },
    Panic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    entry: ParamBlock,
    blocks: Vec<ParamBlock>,
}
