//! Sequence of instructions.
//!
//! Functions in IR consist of basic blocks with SSA-like form.
//!
//! Unlike traditional SSA, blocks in this IR are called `ParamBlock` as they have a parameter
//! and each jump-like instructions should contains parameter of its destination block.
//! It's like phi-node but located at tail of basic block.
//! This decision is made for type inference, mainly failable subtype casting.

use common::{Value, BlockToken, Ty};
use opcode::Opcode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    entry: ParamBlock,
    blocks: Vec<ParamBlock>,
}

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
pub struct Goto(BlockToken, Value);
