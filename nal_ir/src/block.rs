//! Sequence of instructions.
//!
//! Functions in IR consist of basic blocks with SSA-like form.
//!
//! Unlike traditional SSA, blocks in this IR are called `ParamBlock` as they have a parameter
//! and each jump-like instructions should contains parameter of its destination block.
//! It's like phi-node but located at tail of basic block.
//! This decision is made for type inference, mainly failable subtype casting.

use std::collections::HashMap;

use common::{Value, BlockToken};
use opcode::Opcode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    entry: ParamBlock,
    blocks: HashMap<BlockToken, ParamBlock>,
}

impl Function {
    pub fn builder() -> FunctionBuilder {
        FunctionBuilder::new()
    }
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
    // Cast {
    //     when: Value,
    //     cast_to: Ty,
    //     then: Goto,
    //     or: Goto,
    // },
    Panic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goto {
    block: BlockToken,
    param: Value,
}

pub struct FunctionBuilder {
    count: usize,
    param: Value,
    current: Vec<Opcode>,
    blocks: HashMap<BlockToken, ParamBlock>,
    entry: BlockToken,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        let mut count = 0;

        let entry = BlockToken::new(&mut count);
        let param = Value::new(&mut count);

        FunctionBuilder {
            count,
            param,
            current: vec![],
            blocks: HashMap::new(),
            entry,
        }
    }

    pub fn entry(&self) -> BlockToken {
        self.entry
    }

    pub fn value(&mut self) -> Value {
        Value::new(&mut self.count)
    }

    pub fn block(&mut self) -> BlockToken {
        BlockToken::new(&mut self.count)
    }

    pub fn push(&mut self, op: Opcode) {
        self.current.push(op);
    }

    pub fn wrap(&mut self, block: BlockToken, exit: ExitCode) {
        use std::mem::replace;

        let body = replace(&mut self.current, vec![]);

        self.blocks.insert(block, ParamBlock {
            param: self.param,
            body,
            exit,
        });

        self.param = self.value();
    }

    pub fn finish(mut self) -> Function {
        let entry = self.blocks.remove(&self.entry).unwrap();

        Function {
            entry,
            blocks: self.blocks,
        }
    }
}
