use std::collections::HashMap;

use common::{Value, BlockToken};
use opcode::Opcode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub entry: ParamBlock,
    pub blocks: HashMap<BlockToken, ParamBlock>,
}

impl Function {
    pub fn builder() -> FunctionBuilder {
        FunctionBuilder::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamBlock {
    pub param: Value,
    pub body: Vec<Opcode>,
    pub exit: ExitCode,
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
    Panic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goto {
    pub block: BlockToken,
    pub param: Value,
}

pub struct FunctionBuilder {
    count: usize,
    param: Value,
    current_block: BlockToken,
    current_ops: Vec<Opcode>,
    blocks: HashMap<BlockToken, ParamBlock>,
    entry_token: BlockToken,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        let mut count = 0;

        let entry_token = BlockToken::new(&mut count);
        let param = Value::new(&mut count);

        FunctionBuilder {
            count,
            param,
            current_block: entry_token,
            current_ops: vec![],
            blocks: HashMap::new(),
            entry_token,
        }
    }

    pub fn entry(&self) -> BlockToken {
        self.entry_token
    }

    pub fn value(&mut self) -> Value {
        Value::new(&mut self.count)
    }

    pub fn block(&mut self) -> BlockToken {
        BlockToken::new(&mut self.count)
    }

    pub fn push(&mut self, op: Opcode) {
        self.current_ops.push(op);
    }

    pub fn wrap(&mut self, next_block: BlockToken, exit: ExitCode) {
        use std::mem::replace;

        let body = replace(&mut self.current_ops, vec![]);

        self.blocks.insert(self.current_block, ParamBlock {
            param: self.param,
            body,
            exit,
        });

        self.current_block = next_block;
        self.param = self.value();
    }

    pub fn finish(mut self) -> Function {
        let entry = self.blocks.remove(&self.entry_token).unwrap();

        Function {
            entry,
            blocks: self.blocks,
        }
    }
}
